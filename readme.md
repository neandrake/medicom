# medicom #

## About ##
The `medicom` library provides baseline functionality for managing DICOM,
including reading and writing DICOM files, decoding the PixelData element, and
the DIMSE network protocol (C-ECHO, C-FIND, C-STORE, C-MOVE, C-GET).

See the `medicom_tools` sub-crate for example command-line utilities built using
the library:

![Screenshot](static_assets/screenshot.png "Screenshot")

## Quick Examples ##

### Parse through DICOM elements of a file.
```rust
// 1. Set up a parser for a DICOM file.
let parser: Parser<'_, File> = ParserBuilder::default()
    // Stops parsing once the PixelData element is seen to avoid loading it into
    // memory.
    .stop(TagStop::BeforeTagValue(&PixelData))
    // The dictionary is used during parsing for Implicit VR transfer syntaxes,
    // and associates the resolved VR to the resulting elements for parsing the
    // element values.
    .build(file, &STANDARD_DICOM_DICTIONARY);

// 2. Use the parser as an iterator over the elements.
for element_res in parser {
    let element = element_res?;
    // 3. Get a displayable name for the element, by looking it up in the
    //    dictionary or formatting the number as (GGGG,EEEE).
    let tag_name = STANDARD_DICOM_DICTIONARY.get_tag_by_number(elem.tag())
        .map(|tag| tag.ident().to_string())
        .unwrap_or_else(|| Tag::format_tag_to_display(elem.tag()));

    // 4. Parse/interpret the value of an element. The `parse_value()` funtion
    //    will parse as the explicit/implicit VR from the DICOM stream. Use
    //    `parse_value_as()` to parse the value as a different VR.
    //
    //    The `string()` function will attempt to interpret the parsed value as
    //    a single string, the first occurring string, returning `None` if
    //    inapplicable. There are variants for other common types for ease of
    //    parsing, `ushort()`, `int()`, etc.
    if let Some(tag_value) = element.parse_value()?.string() {
        println!("{tag_name}: {tag_value}");
    }
    
    // Refer to the `core::inspect` module for utilities to assist with
    // formatting DICOM element names and values.
}
```

### Decode PixelData for a DICOM SOP Instance.
```rust
// 1. Set up a parser for a DICOM file.
let parser: Parser<'_, File> = ParserBuidler::default()
    .build(file, &STANDARD_DICOM_DICTIONARY);

// 2. Process and load the PixelData into memory.
let pixeldata_slice = PixelDataInfo::process_dcm_parser(parser)?
    .load_pixel_data()?;

// 3. Use the loaded PixelData values.
match pixeldata_slice {
    // The U8 variant will be common for most RGB image data and `pixel_iter()`
    // provides an iterator over the raw pixel values.
    PixelDataSlice::U8(pdslice) => {
        let (width, height) = (pdslice.info().cols(), pdslice.info().rows());

        for PixelU8 { x, y, r, g, b } in pdslice.pixel_iter() {
            // The x, y are usize, within the width and height of the image.
            // The r, g, b are u8, represents the respective pixel color
            // component.
        }
    }

    // The I16 variant will be common for most monochrome image data and
    // `pixel_iter()` provides an iterator over pixels values normalized to I16
    // and rescale applied.
    PixelDataBuffer::I16(pdslice) => {
        let (width, height) = (pdslice.info().cols(), pdslice.info().rows());

        for PixelI16 { x, y, r, g, b } in pdslice.pixel_iter() {
            // The x, y are usize, within the width and height of the image.
            // The r, g, b are i16, and the same value for monochrome.
        }
    }

    // There are U8, U16, U32, I8, I16, and I32 variants, depending on the
    // DICOM's encoded values. See `PixelDataSlice::load_pixel_data()` for
    // details on when to expect which variants.
    _ => {}
}
```

Refer to `medicom/readme.md` folder for further examples, and also the
`medicom_tools` sample applications.

## Design Goals ##
The APIs are not designed to encode the DICOM Information Object Definitions
within the type system. It will allow both reading and writing structures which
are valid DICOM binary protocol but it is up to the user of the API to ensure
that IODs are structured appropriately, i.e. that all necessary DICOM elements
are present and valid for a CT, MR, etc.

While this design puts the burden on the API user to interpret and create well-
formed DICOM structures, it grants greater flexibility, especially for working
with existing malformed DICOM datasets.

The DICOM standard dictionary of tags, UIDs, transfer syntaxes, etc. are
available as an optional feature of the crate. Reading and writing DICOM does
not require the DICOM standard dictionary and can be excluded to minimize the
resulting binary size if desired.

The core crate has minimal dependencies, two required and two optional.

- `encoding_rs` (required) for properly handling text encoding supported by
  DICOM.
- `thiserror` (required) for deriving errors.
- `phf` (optional) the DICOM standard dictionary components are encoded in a
  lookup map using perfect hash maps.
- `libflate` (optional) for reading and writing deflated datasets.

The API is also focused on enabling efficient operations:

- DICOM datasets are parsed in a stream-like manner allowing the API user to
  decide what is necessary to retain in-memory.
- DICOM element values themselves are not parsed during parsing of the dataset.
- Flexible options for limiting how much of a DICOM dataset to parse, e.g.
  stopping before the PixelData element if only metadata about the DICOM SOP is
  needed.
- DIMSE handling of DICOM dataset communication does not require the entire
  dataset to be loaded into memory at once.
- At the moment, decoding PixelData does require the entire element value to be
  loaded into meomry.

## Crates ##

- `medicom_tools`: Example command-line tools using the `medicom` library. See
  the `readme.md` within that folder for more information.
- `medicom`: The core API for reading and writing DICOM, decoding images,and
  optional support for the DICOM Message Exchange network protocol.
- `medicom_dict`: Parses the DICOM Standard XML files for producing the
  standard DICOM dictionary. This is intended to be used by `build.rs` scripts.

