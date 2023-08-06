use crate::xml::XmlElement;

#[derive(Default)]
pub struct Address<'a> {
    pub name: &'a str,
    pub street: Option<&'a str>,
    pub town: &'a str,
    pub zip: &'a str,
    pub country: &'a str,
    pub country_code: Option<&'a str>,
    pub phone: Option<Vec<&'a str>>,
    pub email: Option<Vec<&'a str>>,
}

impl Address<'_> {
    pub fn as_xml(&self) -> XmlElement {
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
        e = e.with_element(ce);

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
