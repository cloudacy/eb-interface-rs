use crate::xml::{ToXml, XmlElement};

#[derive(Default)]
pub struct Address<'a> {
    name: &'a str,
    street: Option<&'a str>,
    town: &'a str,
    zip: &'a str,
    country: &'a str,
    country_code: Option<&'a str>,
    phone: Option<Vec<&'a str>>,
    email: Option<Vec<&'a str>>,
}

impl<'a> Address<'a> {
    pub fn new(name: &'a str, town: &'a str, zip: &'a str, country: &'a str) -> Self {
        Address {
            name,
            town,
            zip,
            country,
            ..Default::default()
        }
    }

    pub fn with_street(mut self, street: &'a str) -> Self {
        self.street = Some(street);
        self
    }

    pub fn with_country_code(mut self, country_code: &'a str) -> Self {
        self.country_code = Some(country_code);
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

impl ToXml for Address<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("Address").with_text_element("Name", self.name);

        if let Some(s) = self.street {
            e = e.with_text_element("Street", s);
        }

        e = e
            .with_text_element("Town", self.town)
            .with_text_element("ZIP", self.zip);

        let mut ce = XmlElement::new("Country").with_text(self.country);
        if let Some(cc) = self.country_code {
            ce = ce.with_attr("CountryCode", cc);
        }
        e = e.with_element(&ce);

        if let Some(phone_numbers) = &self.phone {
            for phone_number in phone_numbers {
                e = e.with_text_element("Phone", *phone_number);
            }
        }

        if let Some(email_addresses) = &self.email {
            for email_address in email_addresses {
                e = e.with_text_element("Email", *email_address);
            }
        }

        e.to_xml()
    }
}
