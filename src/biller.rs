use crate::{
    address::Address, contact::Contact, identification::FurtherIdentification,
    order_reference::OrderReference, xml::XmlElement,
};

#[derive(Default)]
pub struct Biller<'a> {
    vat_identification_number: &'a str,
    further_identification: Option<Vec<FurtherIdentification<'a>>>,
    order_reference: Option<OrderReference<'a>>,
    address: Option<Address<'a>>,
    contact: Option<Contact<'a>>,
}

impl<'a> Biller<'a> {
    pub fn new(vat_identification_number: &str) -> Biller {
        Biller {
            vat_identification_number,
            ..Default::default()
        }
    }

    pub fn with_further_identification(
        mut self,
        further_identification: FurtherIdentification<'a>,
    ) -> Self {
        let mut fi = match self.further_identification {
            Some(fi) => fi,
            None => vec![],
        };
        fi.push(further_identification);
        self.further_identification = Some(fi);
        self
    }

    pub fn with_address(mut self, address: Address<'a>) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_contact(mut self, contact: Contact<'a>) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("Biller")
            .with_text_element("VATIdentificationNumber", self.vat_identification_number);

        if let Some(fis) = &self.further_identification {
            for fi in fis {
                e = e.with_element(fi.as_xml())
            }
        }

        if let Some(or) = &self.order_reference {
            e = e.with_element(or.as_xml());
        }

        if let Some(a) = &self.address {
            e = e.with_element(a.as_xml());
        }

        if let Some(c) = &self.contact {
            e = e.with_element(c.as_xml());
        }

        e
    }
}
