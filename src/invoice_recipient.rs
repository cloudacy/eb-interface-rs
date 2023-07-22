use crate::{
    address::Address, contact::Contact, identification::FurtherIdentification,
    order_reference::OrderReference,
};

pub struct InvoiceRecipient<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Vec<FurtherIdentification<'a>>,
    pub order_reference: Option<OrderReference<'a>>,
    pub address: Option<Address<'a>>,
    pub contact: Option<Contact<'a>>,
}

impl InvoiceRecipient<'_> {
    pub fn as_xml(&self) -> String {
        let vat_identification_number = self.vat_identification_number;
        let further_identification_vec: Vec<String> = (&self.further_identification)
            .into_iter()
            .map(|id| id.as_xml())
            .collect();
        let further_identification = further_identification_vec.join("");
        let order_reference = match &self.order_reference {
            Some(order_reference) => order_reference.as_xml(),
            None => format!(""),
        };
        let address = match &self.address {
            Some(address) => address.as_xml(),
            None => format!(""),
        };
        let contact = match &self.contact {
            Some(contact) => contact.as_xml(),
            None => format!(""),
        };
        format!("<InvoiceRecipient><VATIdentificationNumber>{vat_identification_number}</VATIdentificationNumber>{further_identification}{order_reference}{address}{contact}</InvoiceRecipient>")
    }
}
