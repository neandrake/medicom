use std::{
    collections::HashSet,
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use dcmpipe_lib::{
    core::defn::constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
    dict::uids::{
        CTImageStorage, MRImageStorage, ModalityWorklistInformationModelFIND,
        NuclearMedicineImageStorage, PatientRootQueryRetrieveInformationModelFIND,
        PatientRootQueryRetrieveInformationModelGET, PatientRootQueryRetrieveInformationModelMOVE,
        PositronEmissionTomographyImageStorage, RTDoseStorage, RTPlanStorage,
        RTStructureSetStorage, SecondaryCaptureImageStorage,
        StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
        StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
    },
    dimse::{
        assoc::scu::{UserAssoc, UserAssocBuilder},
        error::AssocError,
    },
};

use crate::{
    args::{SvcUserArgs, SvcUserCommand},
    CommandApplication,
};

pub struct SvcUserApp {
    args: SvcUserArgs,
}

impl SvcUserApp {
    pub fn new(args: SvcUserArgs) -> SvcUserApp {
        SvcUserApp { args }
    }
}

impl CommandApplication for SvcUserApp {
    fn run(&mut self) -> anyhow::Result<()> {
        let supported_abs = HashSet::from([
            &VerificationSOPClass,
            &PatientRootQueryRetrieveInformationModelFIND,
            &StudyRootQueryRetrieveInformationModelFIND,
            &ModalityWorklistInformationModelFIND,
            &PatientRootQueryRetrieveInformationModelMOVE,
            &StudyRootQueryRetrieveInformationModelMOVE,
            &PatientRootQueryRetrieveInformationModelGET,
            &StudyRootQueryRetrieveInformationModelGET,
            &CTImageStorage,
            &MRImageStorage,
            &PositronEmissionTomographyImageStorage,
            &NuclearMedicineImageStorage,
            &SecondaryCaptureImageStorage,
            &RTStructureSetStorage,
            &RTDoseStorage,
            &RTPlanStorage,
        ]);
        let supported_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);

        let mut assoc = UserAssocBuilder::default()
            .id(0)
            .my_ae(self.args.my_ae.clone())
            .service_ae(self.args.host_ae.clone())
            .supported_abs(supported_abs)
            .supported_ts(supported_ts)
            .build();

        let stream = TcpStream::connect(&self.args.host)?;
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        if let Err(e) = self.start(reader, &mut writer, &mut assoc) {
            let _ = e.write(&mut writer);
            eprintln!("Error: {e:?}");
        }
        Ok(())
    }
}

impl SvcUserApp {
    fn start(
        &self,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        assoc: &mut UserAssoc,
    ) -> Result<(), AssocError> {
        assoc.request_association(&mut reader, &mut writer)?;

        match self.args.cmd {
            SvcUserCommand::Echo => {
                assoc.c_echo_rq(&mut reader, &mut writer)?;
            }
        }

        Ok(())
    }
}
