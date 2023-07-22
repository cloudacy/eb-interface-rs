pub struct OrderReference<'a> {
    pub order_id: &'a str,
    pub reference_date: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl OrderReference<'_> {
    pub fn as_xml(&self) -> String {
        let order_id = self.order_id;
        let reference_date = match self.reference_date {
            Some(d) => format!("<ReferenceDate>{d}</ReferenceDate>"),
            None => format!(""),
        };
        let description = match self.description {
            Some(d) => format!("<Description>{d}</Description>"),
            None => format!(""),
        };
        format!("<OrderReference><OrderID>{order_id}</OrderID>{reference_date}{description}</OrderReference>")
    }
}
