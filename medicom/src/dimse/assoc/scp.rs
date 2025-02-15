/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
};

use crate::{
    core::{
        charset::CSRef,
        defn::{dcmdict::DicomDictionary, ts::TSRef, uid::UIDRef},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, uids::DICOMApplicationContextName},
    dimse::{
        assoc::CommonAssoc,
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                AssocAC, AssocACPresentationContext, AssocRQ, TransferSyntaxItem,
                UserInformationItem,
            },
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem},
            Pdu, UserPdu,
        },
        Syntax,
    },
};

/// Represents a single association, for a service-provider, SCP.
pub struct ServiceAssoc {
    common: CommonAssoc,
    accept_aets: HashMap<String, String>,
}

impl ServiceAssoc {
    /// Access the fields & functions that are common to both SCU and SCP associations.
    #[must_use]
    pub fn common(&self) -> &CommonAssoc {
        &self.common
    }

    /// Mutably access the fields & functions that are common to both SCU and SCP associations.
    #[must_use]
    pub fn common_mut(&mut self) -> &mut CommonAssoc {
        &mut self.common
    }

    /// Check if the given AE title is known or should be accepted.
    #[must_use]
    pub fn accept_aet(&self, aet: &str) -> bool {
        self.accept_aets.is_empty() || self.accept_aets.contains_key(aet)
    }

    /// Gets the connection info for a known AE Title.
    #[must_use]
    pub fn aet_host(&self, aet: &str) -> Option<&String> {
        self.accept_aets.get(aet)
    }

    /// Accept the association request, negotiating the association parameters and leaving the
    /// reader/writer in a state to start exchanging PDUs.
    ///
    /// # Errors
    /// - I/O errors may occur when reading/writing from the reader/writer.
    /// - Any misbehaving SCU will be managed within, and this will not propagate these as errors.
    pub fn accept<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), AssocError> {
        let rq = Pdu::read(&mut reader)
            .map_err(AssocError::ab_failure)
            .and_then(|rq| match rq {
                Pdu::AssocRQ(rq) => Ok(rq),
                pdu => Err(AssocError::ab_unexpected_pdu(
                    DimseError::UnexpectedPduType(pdu.pdu_type()),
                )),
            });

        let rq = rq?;
        let (assoc_ac, agreed_abs) = self.validate_assoc_rq(&rq)?;

        for pres_ctx in assoc_ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                let Some(ab) = agreed_abs.get(&pres_ctx.ctx_id()) else {
                    continue;
                };
                self.common
                    .negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), (pres_ctx.to_owned(), ab));
            }
        }

        CommonAssoc::write_pdu(&Pdu::AssocAC(assoc_ac), &mut writer)?;

        Ok(())
    }

    /// Validates the association request, checking that this `Association`'s configuration can
    /// handle and will accept the request.
    ///
    /// # Return
    /// Whether to accept, reject, or abort the association request. If accepted, also includes the
    /// accepted abstract and transfer syntaxes.
    ///
    /// # Errors
    /// - If the result of the request is to reject or abort, those are propagated as an
    ///   `AssocError`.
    fn validate_assoc_rq(
        &mut self,
        rq: &AssocRQ,
    ) -> Result<(AssocAC, HashMap<u8, UIDRef>), AssocError> {
        Self::validate_app_ctx(rq)?;

        let host_ae = self.common.this_ae.trim();
        Self::validate_ae_titles(rq, host_ae, &self.accept_aets)?;

        // TODO: Do things with SOPClassCommonExtendedNegotiationItem, UserIdentityItem, etc.

        // Process the requested presentation contexts, confirming supported transfer syntaxes and
        // abstract syntaxes.
        let mut agreed_abs: HashMap<u8, UIDRef> =
            HashMap::with_capacity(self.common.supported_abs().len());
        let mut rsp_pres_ctx: Vec<AssocACPresentationContext> =
            Vec::with_capacity(rq.pres_ctxs().len());
        for req_pres_ctx in rq.pres_ctxs() {
            let ts = req_pres_ctx
                .transfer_syntaxes()
                .iter()
                .filter_map(|ts| String::try_from(&Syntax(ts.transfer_syntaxes())).ok())
                .filter_map(|ts| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&ts))
                .find(|ts| self.common.supported_ts().contains(ts));

            let Some(ts) = ts else {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    4,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            };

            let ab = String::try_from(&Syntax(req_pres_ctx.abstract_syntax().abstract_syntax()))
                .ok()
                .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&ab))
                .filter(|ab| self.common.supported_abs().contains(ab));

            let Some(ab) = ab else {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    3,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            };

            agreed_abs.insert(req_pres_ctx.ctx_id(), ab);

            let acc_pres_ctx = AssocACPresentationContext::new(
                req_pres_ctx.ctx_id(),
                0,
                TransferSyntaxItem::from(ts),
            );
            rsp_pres_ctx.push(acc_pres_ctx);
        }

        // Grab a copy of their user data for later, things like `MaxLengthItem` etc.
        self.common
            .their_user_data
            .clone_from(rq.user_info().user_data());

        // Copy the starting user data for this SCU, which should only have `MaxLengthItem` and
        // `AsyncOperationsWindowItem`, but not any `RoleSelectionItem`s.
        let mut accepted_user_data: Vec<UserPdu> = self.common.this_user_data.clone();
        accepted_user_data.append(&mut Self::validate_roles(rq, &agreed_abs));

        Ok((
            AssocAC::new(
                rq.called_ae().to_owned(),
                rq.calling_ae().to_owned(),
                rq.reserved_3().to_owned(),
                rq.app_ctx().to_owned(),
                rsp_pres_ctx,
                UserInformationItem::new(accepted_user_data),
            ),
            agreed_abs,
        ))
    }

    /// There's only a single DICOM Standard defined Application Context Name. Private Application
    /// Context Names are allowed by the standard.
    fn validate_app_ctx(rq: &AssocRQ) -> Result<(), AssocError> {
        let app_ctx = String::try_from(&Syntax(rq.app_ctx().app_context_name()))
            .map_err(AssocError::rj_unsupported)?;

        let app_ctx_uid = STANDARD_DICOM_DICTIONARY
            .get_uid_by_uid(&app_ctx)
            .ok_or_else(|| AssocError::rj_unsupported(DimseError::InvalidAppContext(app_ctx)))?;

        if app_ctx_uid != &DICOMApplicationContextName {
            return Err(AssocError::rj_unsupported(DimseError::InvalidAppContext(
                format!("{}, {}", app_ctx_uid.name(), app_ctx_uid.uid()),
            )));
        }

        Ok(())
    }

    /// Verifies the called AE title matches this host AE and that the calling AE title is
    /// known (or this SCP accepts all calling AE titles).
    fn validate_ae_titles(
        rq: &AssocRQ,
        host_ae: &str,
        accepted_calling: &HashMap<String, String>,
    ) -> Result<(), AssocError> {
        let called_ae = CSRef::default()
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::CharsetError(e)))?;
        if called_ae != host_ae {
            return Err(AssocError::rj_called_aet(DimseError::InvalidCalledAeTitle(
                called_ae,
            )));
        }

        let calling_ae = CSRef::default()
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::from(e)))?;
        if !accepted_calling.is_empty() && !accepted_calling.contains_key(&calling_ae) {
            return Err(AssocError::rj_calling_aet(
                DimseError::InvalidCallingAeTitle(calling_ae),
            ));
        }

        Ok(())
    }

    /// Inspects the incoming requested roles to confirm it indicates the other SCU is acting as
    /// user and not provider, as well as for an accepted abstract syntax.
    fn validate_roles(rq: &AssocRQ, agreed_abs: &HashMap<u8, UIDRef>) -> Vec<UserPdu> {
        let mut accepted_user_data: Vec<UserPdu> = Vec::new();
        for user_pdu in rq.user_info().user_data() {
            if let UserPdu::RoleSelectionItem(role) = user_pdu {
                if !role.is_user() || role.is_provider() {
                    continue;
                }
                if let Some(ab) = String::try_from(&Syntax(role.sop_class_uid()))
                    .ok()
                    .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&ab))
                {
                    if agreed_abs.values().any(|uid| *uid == ab) {
                        accepted_user_data.push(UserPdu::RoleSelectionItem(role.clone()));
                    }
                }
            }
        }
        accepted_user_data
    }
}

#[derive(Default, Clone)]
pub struct ServiceAssocBuilder {
    id: usize,
    host_ae: String,
    accept_aets: HashMap<String, String>,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    pdu_rcv_max_len: u32,
}

impl ServiceAssocBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    pub fn host_ae(mut self, host_ae: String) -> Self {
        self.host_ae = host_ae;
        self
    }

    #[must_use]
    pub fn accept_aets(mut self, accept_aets: HashMap<String, String>) -> Self {
        self.accept_aets = accept_aets;
        self
    }

    #[must_use]
    pub fn supported_abs(mut self, accept_abs: HashSet<UIDRef>) -> Self {
        self.supported_abs = accept_abs;
        self
    }

    #[must_use]
    pub fn supported_ts(mut self, accept_ts: HashSet<TSRef>) -> Self {
        self.supported_ts = accept_ts;
        self
    }

    #[must_use]
    pub fn pdu_rcv_max_len(mut self, pdu_rcv_max_len: u32) -> Self {
        self.pdu_rcv_max_len = pdu_rcv_max_len;
        self
    }

    #[must_use]
    pub fn build(self) -> ServiceAssoc {
        let num_abs = self.supported_abs.len();
        // MaxLengthItem + AsyncOperationsWindowItem + RoleSelectionItem * supported_abs.len()
        let num_user_data = 2 + self.supported_abs.len();
        let mut this_user_data = Vec::<UserPdu>::with_capacity(num_user_data);
        this_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        this_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        let common_assoc = CommonAssoc {
            _id: self.id,
            this_ae: self.host_ae,
            supported_abs: self.supported_abs,
            supported_ts: self.supported_ts,
            this_user_data,

            their_user_data: Vec::with_capacity(num_user_data),
            negotiated_pres_ctx: HashMap::with_capacity(num_abs),
            active_user_ops: HashMap::new(),
            active_svc_ops: HashMap::new(),
        };

        ServiceAssoc {
            common: common_assoc,
            accept_aets: self.accept_aets,
        }
    }
}
