use crate::xml::XmlElement;

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

impl FurtherIdentificationType {
    pub fn as_str(&self) -> &str {
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
    }
}

pub struct FurtherIdentification<'a> {
    pub id: &'a str,
    pub id_type: FurtherIdentificationType,
}

impl FurtherIdentification<'_> {
    pub fn as_xml(&self) -> XmlElement {
        XmlElement::new("FurtherIdentification")
            .with_attr("IdentificationType", self.id_type.as_str())
            .with_text(self.id)
    }
}
