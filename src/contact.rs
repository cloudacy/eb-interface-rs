use crate::xml::XmlElement;

#[derive(Default)]
pub struct Contact<'a> {
    pub salutation: Option<&'a str>,
    pub name: &'a str,
    pub phone: Option<Vec<&'a str>>,
    pub email: Option<Vec<&'a str>>,
}

impl Contact<'_> {
    pub fn as_xml(&self) -> XmlElement {
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

        e
    }
}
