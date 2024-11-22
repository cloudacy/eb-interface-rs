use crate::xml::{ToXml, XmlElement};

pub enum FurtherIdentificationType {
    ARA,
    #[allow(non_camel_case_types)]
    BBG_GZ,
    Consolidator,
    Contract,
    DVR,
    EORI,
    ERSB,
    FN,
    FR,
    HG,
    Payer,
    FASTNR,
    VID,
    VN,
}

impl ToString for FurtherIdentificationType {
    fn to_string(&self) -> String {
        match self {
            FurtherIdentificationType::ARA => "ARA",
            FurtherIdentificationType::BBG_GZ => "BBG_GZ",
            FurtherIdentificationType::Consolidator => "Consolidator",
            FurtherIdentificationType::Contract => "Contract",
            FurtherIdentificationType::DVR => "DVR",
            FurtherIdentificationType::EORI => "EORI",
            FurtherIdentificationType::ERSB => "ERSB",
            FurtherIdentificationType::FN => "FN",
            FurtherIdentificationType::FR => "FR",
            FurtherIdentificationType::HG => "HG",
            FurtherIdentificationType::Payer => "Payer",
            FurtherIdentificationType::FASTNR => "FASTNR",
            FurtherIdentificationType::VID => "VID",
            FurtherIdentificationType::VN => "VN",
        }
        .to_owned()
    }
}

pub struct FurtherIdentification<'a> {
    id: &'a str,
    id_type: FurtherIdentificationType,
}

impl FurtherIdentification<'_> {
    pub fn new(id: &str, id_type: FurtherIdentificationType) -> FurtherIdentification {
        FurtherIdentification { id, id_type }
    }
}

impl ToXml for FurtherIdentification<'_> {
    fn to_xml(&self) -> String {
        XmlElement::new("FurtherIdentification")
            .with_attr("IdentificationType", self.id_type.to_string())
            .with_text(self.id)
            .to_xml()
    }
}
