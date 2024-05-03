use crate::{
    address::Address, contact::Contact, identification::FurtherIdentification,
    order_reference::OrderReference, utils::init_vec, xml::XmlElement,
};

#[derive(Default)]
pub struct InvoiceRecipient<'a> {
    vat_identification_number: &'a str,
    further_identification: Option<Vec<FurtherIdentification<'a>>>,
    order_reference: Option<OrderReference<'a>>,
    address: Option<Address<'a>>,
    contact: Option<Contact<'a>>,
}

impl<'a> InvoiceRecipient<'a> {
    pub fn new(vat_identification_number: &str) -> InvoiceRecipient {
        InvoiceRecipient {
            vat_identification_number,
            ..Default::default()
        }
    }

    pub fn with_further_identification(
        mut self,
        further_identification: FurtherIdentification<'a>,
    ) -> Self {
        let fi = self.further_identification.get_or_insert_with(init_vec);
        fi.push(further_identification);
        self
    }

    pub fn with_order_reference(mut self, order_reference: OrderReference<'a>) -> Self {
        self.order_reference = Some(order_reference);
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
