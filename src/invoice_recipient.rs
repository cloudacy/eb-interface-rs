use crate::{
    address::Address, contact::Contact, identification::FurtherIdentification,
    order_reference::OrderReference, xml::XmlElement,
};

#[derive(Default)]
pub struct InvoiceRecipient<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Option<Vec<FurtherIdentification<'a>>>,
    pub order_reference: Option<OrderReference<'a>>,
    pub address: Option<Address<'a>>,
    pub contact: Option<Contact<'a>>,
}

impl InvoiceRecipient<'_> {
    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("InvoiceRecipient")
            .with_text_element("VATIdentificationNumber", self.vat_identification_number);

        if let Some(fis) = &self.further_identification {
            for fi in fis {
                e = e.with_element(fi.as_xml());
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
