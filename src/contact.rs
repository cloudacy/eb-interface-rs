use crate::xml::{ToXml, XmlElement};

#[derive(Default)]
pub struct Contact<'a> {
    salutation: Option<&'a str>,
    name: &'a str,
    phone: Option<Vec<&'a str>>,
    email: Option<Vec<&'a str>>,
}

impl<'a> Contact<'a> {
    pub fn new(name: &'a str) -> Self {
        Contact {
            name,
            ..Default::default()
        }
    }

    pub fn with_salutation(mut self, salutation: &'a str) -> Self {
        self.salutation = Some(salutation);
        self
    }

    pub fn with_phone(mut self, phone_number: &'a str) -> Self {
        self.phone.get_or_insert_with(Vec::new).push(phone_number);
        self
    }

    pub fn with_email(mut self, email_address: &'a str) -> Self {
        self.email.get_or_insert_with(Vec::new).push(email_address);
        self
    }
}

impl ToXml for Contact<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("Contact");

        if let Some(s) = self.salutation {
            e = e.with_text_element("Salutation", s);
        }

        e = e.with_text_element("Name", self.name);

        if let Some(phone_numbers) = &self.phone {
            for phone_number in phone_numbers {
                e = e.with_text_element("Phone", phone_number);
            }
        }

        if let Some(email_addresses) = &self.email {
            for email_address in email_addresses {
                e = e.with_text_element("Email", email_address);
            }
        }

        e.to_xml()
    }
}
