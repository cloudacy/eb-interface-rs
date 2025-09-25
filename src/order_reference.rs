use crate::xml::{ToXml, XmlElement};

#[derive(Default)]
pub struct OrderReference<'a> {
    order_id: &'a str,
    reference_date: Option<&'a str>,
    description: Option<&'a str>,
}

impl<'a> OrderReference<'a> {
    pub fn new(order_id: &'a str) -> Self {
        OrderReference {
            order_id,
            ..Default::default()
        }
    }

    pub fn with_reference_date(mut self, reference_date: &'a str) -> Self {
        self.reference_date = Some(reference_date);
        self
    }

    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

impl ToXml for OrderReference<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("OrderReference").with_text_element("OrderID", self.order_id);

        if let Some(d) = self.reference_date {
            e = e.with_text_element("ReferenceDate", d);
        }

        if let Some(d) = self.description {
            e = e.with_text_element("Description", d);
        }

        e.to_xml()
    }
}
