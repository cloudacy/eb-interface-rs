use crate::xml::XmlElement;

pub struct OrderReference<'a> {
    pub order_id: &'a str,
    pub reference_date: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl OrderReference<'_> {
    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("OrderReference").with_text_element("OrderID", self.order_id);

        if let Some(d) = self.reference_date {
            e = e.with_text_element("ReferenceDate", d);
        }

        if let Some(d) = self.description {
            e = e.with_text_element("Description", d);
        }

        e
    }
}
