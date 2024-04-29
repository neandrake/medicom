use std::collections::HashMap;
use std::fs::File;
use std::io::{stdout, Stdout};
use std::ops::Sub;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};

use crossterm::event::{self, Event::Key, Event::Mouse, KeyCode::Char};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind, MouseButton,
    MouseEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::read::Parser;
use dcmpipe_lib::defn::tag::{Tag, TagNode, TagPath};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use ratatui::{Frame, Terminal};

use crate::app::CommandApplication;
use crate::args::BrowseArgs;

use super::{get_nth_child, ElementWithLineFmt, TagName, TagValue};

pub struct BrowseApp {
    args: BrowseArgs,
}

#[derive(Debug)]
enum BrowseError {
    InvalidTagPath(TagPath),
}

impl std::fmt::Display for BrowseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrowseError::InvalidTagPath(tagpath) => write!(f, "No model for path {tagpath:?}"),
        }
    }
}

impl std::error::Error for BrowseError {}

/// The result of parsing all elements in a DICOM data set.
struct DicomDocumentModel<'app> {
    /// The file path the DICOM dataset was loaded from.
    path: &'app Path,
    /// The mapping of some `TagPath` to that path's parsed child nodes. The empty path represents
    /// the root of the DICOM data set, whose model contains all the top-level DICOM elements. If
    /// the data set includes sequences then additional entries for each sequence element will be
    /// present to include its parsed sub-elements.
    map: HashMap<TagPath, DicomElementModel<'app>>,
}

/// The data model for an element. This represents one "level" within a DICOM document model, where
/// the rows for this model are the first-level child elements of some other `DicomNode`.
/// This model only contains the data necessary for rendering, so all DICOM element values are
/// parsed in order to build this struct.
#[derive(Clone)]
struct DicomElementModel<'model> {
    /// The ordered values parsed from the DICOM elements at this level.
    rows: Vec<Row<'model>>,
    /// For each row, the maximum length of DICOM tag name, which aside from DICOM value will be
    /// the only other column of variable width.
    max_name_width: u16,
}

/// The ViewState of what's displayed on screen. This should remain minimal (i.e. not include the
/// data model), as it will be cloned every frame render. This contains both view-level information
/// about the current model being displayed as well as view state from user input.
#[derive(Clone)]
struct ViewState {
    /// Title to show in top-left of table
    dataset_title: String,
    /// The number of rows of the current model.
    num_rows: usize,
    /// The maximum width of all DICOM tag names of the current model.
    max_name_width: u16,
    /// The Ratatui table state which contains offset and selection.
    table_state: TableState,
    /// Whether the user has requested to quit/close.
    user_quit: bool,
    /// The user selected a row to dive deeper into.
    user_nav: UserNav,
    /// The current path of elements to display.
    current_root_element: TagPath,
}

/// Actions the user can take to navigate the DICOM document.
#[derive(Clone)]
enum UserNav {
    None,
    IntoLevel(usize),
    UpLevel,
}

impl CommandApplication for BrowseApp {
    fn run(&mut self) -> Result<()> {
        let path: &Path = self.args.file.as_path();
        let mut parser: Parser<'_, File> = super::parse_file(path, true)?;
        let parse_result = DicomRoot::parse(&mut parser);

        let dcmroot = match parse_result {
            Ok(Some(dcmroot)) => dcmroot,
            Ok(None) => return Err(anyhow!("Not valid DICOM.")),
            Err(err) => return Err(anyhow!(err)),
        };

        let doc_model = DicomDocumentModel::parse(path, &dcmroot);

        let mut terminal = self.init()?;

        let app_result = self.run_loop(&mut terminal, &dcmroot, &doc_model);

        self.close(terminal)?;

        app_result?;

        Ok(())
    }
}

impl<'app> DicomDocumentModel<'app> {
    fn parse<'dict>(path: &'app Path, dcmroot: &DicomRoot<'dict>) -> DicomDocumentModel<'app> {
        let map = DicomElementModel::parse(dcmroot);
        DicomDocumentModel { path, map }
    }
}

impl<'model> DicomElementModel<'model> {
    fn parse<'dict>(dcmnode: &'dict dyn DicomNode) -> HashMap<TagPath, DicomElementModel<'model>> {
        let mut map: HashMap<TagPath, DicomElementModel<'model>> = HashMap::new();

        let mut rows: Vec<Row<'model>> = Vec::with_capacity(dcmnode.get_child_count());
        let mut max_name_width: u16 = 0;
        for item in dcmnode.iter_items() {
            let (row, child_map, name_len) = DicomElementModel::parse_dcmobj(item);
            rows.push(row);
            map.extend(child_map);
            max_name_width = max_name_width.max(name_len);
        }
        for (_child_tag, child) in dcmnode.iter_child_nodes() {
            let (row, child_map, name_len) = DicomElementModel::parse_dcmobj(child);
            rows.push(row);
            map.extend(child_map);
            max_name_width = max_name_width.max(name_len);
        }

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let elem_tbl = DicomElementModel {
            rows,
            max_name_width,
        };

        let tagpath = dcmnode.get_element().map_or_else(
            || TagPath {
                nodes: Vec::with_capacity(0),
            },
            |e| e.get_tagpath(),
        );
        map.insert(tagpath, elem_tbl);

        map
    }

    fn parse_dcmobj(
        child: &DicomObject,
    ) -> (
        Row<'model>,
        HashMap<TagPath, DicomElementModel<'model>>,
        u16,
    ) {
        let mut map: HashMap<TagPath, DicomElementModel<'model>> = HashMap::new();
        let child_tag = child.as_element().get_tag();
        if child.get_item_count() > 0 || child.get_child_count() > 0 {
            let child_map = DicomElementModel::parse(child);
            map.extend(child_map);
        }

        let tag_render: TagName = child.as_element().into();
        let elem_name = tag_render.to_string();
        let name_len = elem_name.len() as u16;
        let elem_value: TagValue = ElementWithLineFmt(child.as_element(), false).into();

        let mut cells: Vec<Cell> = Vec::with_capacity(5);
        cells.push(
            Cell::from(if child.get_child_count() > 0 { "+" } else { "" })
                .style(Style::default().fg(Color::DarkGray)),
        );

        cells.push(
            Cell::from(Tag::format_tag_to_display(child_tag))
                .style(Style::default().fg(Color::DarkGray)),
        );

        match tag_render {
            TagName::Known(_, _) => {
                cells.push(Cell::from(elem_name));
            }
            _ => {
                cells.push(
                    Cell::from(elem_name).style(
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::ITALIC),
                    ),
                );
            }
        }

        cells.push(
            Cell::from(child.as_element().get_vr().ident)
                .style(Style::default().fg(Color::DarkGray)),
        );

        let cell = match elem_value {
            TagValue::Sequence => Cell::from(""),
            TagValue::Error(err_str) => Cell::from(err_str).style(Style::default().bg(Color::Red)),
            TagValue::Uid(uid, name) => Cell::from(Line::from(vec![
                Span::styled(uid, Style::default()),
                Span::styled(
                    format!(" {}", name),
                    Style::default().fg(Color::LightYellow),
                ),
            ])),
            TagValue::Stringified(str_val) => Cell::from(str_val),
        };
        cells.push(cell);

        (Row::new(cells), map, name_len)
    }
}

impl<'app> BrowseApp {
    pub fn new(args: BrowseArgs) -> BrowseApp {
        BrowseApp { args }
    }

    fn init(&self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        Ok(terminal)
    }

    fn close(&self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.clear()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn run_loop<'dict>(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        dcmroot: &'dict DicomRoot,
        doc_model: &'app DicomDocumentModel<'app>,
    ) -> Result<()> {
        let root_path = TagPath {
            nodes: Vec::with_capacity(0),
        };
        let default_table_state = TableState::new().with_selected(Some(0));

        // Track table state per-path, to match DicomDocumentModel's layout. This allows navigation
        // of elements to retain their own offset + selection.
        let mut table_state_map: HashMap<TagPath, TableState> = HashMap::new();

        let mut view_state = ViewState {
            dataset_title: doc_model.path.to_str().unwrap_or_default().to_owned(),
            num_rows: 0,
            max_name_width: 0,
            table_state: table_state_map
                .entry(root_path.clone())
                .or_insert_with(|| default_table_state.clone())
                .clone(),
            user_quit: false,
            user_nav: UserNav::None,
            current_root_element: root_path.clone(),
        };

        loop {
            let Some(table_model) = doc_model.map.get(&view_state.current_root_element) else {
                return Err(BrowseError::InvalidTagPath(view_state.current_root_element).into());
            };

            // Apply state from current model.
            view_state.num_rows = table_model.rows.len();
            view_state.max_name_width = table_model.max_name_width;
            view_state.table_state = table_state_map
                .entry(view_state.current_root_element.clone())
                .or_insert_with(|| default_table_state.clone())
                .clone();
            // Reset user-input state.
            view_state.user_quit = false;
            view_state.user_nav = UserNav::None;

            view_state.dataset_title = if view_state.current_root_element.nodes.is_empty() {
                doc_model.path.to_str().unwrap_or_default().to_string()
            } else {
                TagPath::format_tagpath_to_display(
                    &view_state.current_root_element,
                    Some(dcmroot.get_dictionary()),
                )
            };

            // Ratatui's Table requires an iterator over owned Rows, so the model must be cloned
            // every render, apparently. The render_stateful_widget() function requires moving a
            // Table into it, so even if the Table was lifted up into view_state or similar, some
            // sort of clone would have to be passed into rendering.
            let render_model = table_model.clone();
            // The view_state is small and intended to be cloned every iteration.
            let render_view_state = view_state.clone();

            let current_path = view_state.current_root_element.clone();

            terminal.draw(|frame| self.render(render_model, render_view_state, frame))?;

            view_state = self.update_state_from_user_input(dcmroot, doc_model, view_state)?;

            // Update the previous table state to ensure the offset+selection actually persists.
            // This must use the path prior to `update_state_from_user_input()` which will modify
            // the path based on user navigation.
            table_state_map.insert(current_path, view_state.table_state.clone());

            if view_state.user_quit {
                break;
            }
        }
        Ok(())
    }

    fn update_state_from_user_input(
        &self,
        dcmroot: &DicomRoot,
        doc_model: &'app DicomDocumentModel<'app>,
        mut view_state: ViewState,
    ) -> Result<ViewState> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Key(key) => match key.kind {
                    KeyEventKind::Press => self.event_keypress(&mut view_state, key),
                    KeyEventKind::Release => self.event_keyrelease(&mut view_state, key),
                    _ => {}
                },
                Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(button) | MouseEventKind::Drag(button) => {
                        self.event_mouse_down(&mut view_state, mouse, button)
                    }
                    MouseEventKind::ScrollDown => {
                        self.event_mouse_scroll_down(&mut view_state, mouse)
                    }
                    MouseEventKind::ScrollUp => self.event_mouse_scroll_up(&mut view_state, mouse),
                    _ => {}
                },
                _ => {}
            }
        }

        // Handle user navigation
        match view_state.user_nav {
            UserNav::None => {}
            UserNav::IntoLevel(selected) => {
                let next_path = if view_state.current_root_element.nodes.is_empty() {
                    get_nth_child(dcmroot, selected)
                        .map(|o| o.as_element().get_tagpath())
                        .unwrap_or_else(|| view_state.current_root_element.clone())
                } else {
                    dcmroot
                        .get_child_by_tagpath(&view_state.current_root_element)
                        .and_then(|c| {
                            // Check items first and children second. Sequences will have a
                            // single child which is the delimiter at the end.
                            if c.get_item_count() > 0 {
                                if selected < c.get_item_count() {
                                    c.get_item_by_index(selected + 1)
                                } else if c.get_child_count() > 0 {
                                    // Subtract the # items because children appear after items
                                    // when both are present.
                                    get_nth_child(c, selected - c.get_item_count())
                                } else {
                                    None
                                }
                            } else if c.get_item_count() > 0 {
                                c.get_item_by_index(selected + 1)
                            } else {
                                None
                            }
                        })
                        .map(|o| o.as_element().get_tagpath())
                        .unwrap_or_else(|| view_state.current_root_element.clone())
                };

                if doc_model.map.contains_key(&next_path) {
                    view_state.current_root_element = next_path;
                }
            }
            UserNav::UpLevel => {
                if !view_state.current_root_element.nodes.is_empty() {
                    let mut nodes = view_state
                        .current_root_element
                        .nodes
                        .drain(
                            ..view_state
                                .current_root_element
                                .nodes
                                .len()
                                .saturating_sub(1),
                        )
                        .collect::<Vec<TagNode>>();

                    // Remove item # from the last element of the path as the model map uses a
                    // key of the TagPath with no index specified.
                    if let Some(last) = nodes.last_mut() {
                        last.get_item_mut().take();
                    }
                    view_state.current_root_element = nodes.into();
                }
            }
        }
        Ok(view_state)
    }

    fn event_keyrelease(&self, _view_state: &mut ViewState, _event: KeyEvent) {}

    fn event_keypress(&self, view_state: &'app mut ViewState, event: KeyEvent) {
        match event.code {
            Char('q') => view_state.user_quit = true,
            KeyCode::Esc => view_state.user_quit = true,
            KeyCode::Enter => {
                if let Some(selected) = view_state.table_state.selected() {
                    view_state.user_nav = UserNav::IntoLevel(selected);
                }
            }
            Char('h') | KeyCode::Left | KeyCode::Backspace => {
                view_state.user_nav = UserNav::UpLevel
            }
            Char('j') | KeyCode::Down => self.table_select_next(view_state, 1),
            Char('k') | KeyCode::Up => self.table_select_next(view_state, -1),
            _ => {}
        }
    }

    fn event_mouse_down(
        &self,
        view_state: &'app mut ViewState,
        event: MouseEvent,
        button: MouseButton,
    ) {
        if button != MouseButton::Left {
            return;
        }

        // Convert the event row (all widgets on screen) into the table row.
        // Subtract 2, 1 for the table border, 1 for the table header row.
        let row_index = event.row.saturating_sub(2) as usize;

        let index = Some(view_state.table_state.offset().saturating_add(row_index));
        // Only toggle the selection on click, not drag.
        if view_state.table_state.selected() == index
            && event.kind == MouseEventKind::Down(MouseButton::Left)
        {
            view_state.table_state.select(None)
        } else {
            view_state.table_state.select(index);
        }
    }

    fn event_mouse_scroll_up(&self, view_state: &'app mut ViewState, _event: MouseEvent) {
        self.table_scroll_next(view_state, -1);
    }

    fn event_mouse_scroll_down(&self, view_state: &'app mut ViewState, _event: MouseEvent) {
        self.table_scroll_next(view_state, 1);
    }

    fn table_scroll_next(&self, view_state: &'app mut ViewState, modifier: isize) {
        let i = view_state
            .table_state
            .offset()
            .saturating_add_signed(modifier)
            .min(view_state.num_rows)
            .max(0);
        *view_state.table_state.offset_mut() = i;
    }

    fn table_select_next(&self, view_state: &'app mut ViewState, modifier: isize) {
        let i = match view_state.table_state.selected() {
            None => 0,
            Some(i) => view_state
                .num_rows
                .sub(1)
                .min(i.saturating_add_signed(modifier))
                .max(0),
        };
        view_state.table_state.select(Some(i));
    }

    fn render(&self, model: DicomElementModel, view_state: ViewState, frame: &mut Frame) {
        let column_widths = [
            Constraint::Length(1),
            Constraint::Length(11),
            Constraint::Length(view_state.max_name_width),
            Constraint::Length(2),
            Constraint::Max(1024),
        ];

        let table = Table::new(model.rows, column_widths)
            .header(
                Row::new(vec!["+", "Tag", "Name", "VR", "Value"])
                    .style(Style::default().fg(Color::LightYellow)),
            )
            .block(
                Block::default()
                    .title(
                        Title::from(Line::from(Span::styled(
                            "[DICOM Browser]".to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        )))
                        .alignment(Alignment::Left),
                    )
                    .title(
                        Title::from(Line::from(Span::styled(
                            format!("[{}]", &view_state.dataset_title),
                            Style::default().fg(Color::LightBlue),
                        )))
                        .alignment(Alignment::Right),
                    )
                    .borders(Borders::all()),
            )
            .highlight_style(Style::default().bg(Color::LightBlue));

        let sections = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        frame.render_stateful_widget(table, sections[0], &mut view_state.table_state.clone());
    }
}
