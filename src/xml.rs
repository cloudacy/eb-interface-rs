use std::borrow::Cow;

fn xml_escape(s: &mut Cow<str>) {
    if s.contains(['&', '"', '\'', '<', '>']) {
        let mut o = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '&' => o.push_str("&amp;"),
                '"' => o.push_str("&quot;"),
                '\'' => o.push_str("&apos;"),
                '<' => o.push_str("&lt;"),
                '>' => o.push_str("&gt;"),
                _ => o.push(c),
            }
        }
        *s.to_mut() = o;
    }
}

pub(crate) trait ToXml {
    fn to_xml(&self) -> String;
}

struct XmlText<'a> {
    text: Cow<'a, str>,
}

impl XmlText<'_> {
    fn to_xml(&mut self) -> String {
        xml_escape(&mut self.text);
        self.text.to_string()
    }
}

struct XmlAttribute<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl XmlAttribute<'_> {
    fn to_xml(&mut self) -> String {
        xml_escape(&mut self.name);
        xml_escape(&mut self.value);
        format!("{}=\"{}\"", &self.name, &self.value)
    }
}

#[derive(Default)]
pub(crate) struct XmlElement {
    name: String,
    attrs: Option<Vec<String>>,
    body: Vec<String>,
}

impl XmlElement {
    pub(crate) fn new<'a>(name: impl Into<Cow<'a, str>>) -> Self {
        let mut name = name.into();
        xml_escape(&mut name);

        XmlElement {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn with_attr<'a>(mut self, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        self.attrs.get_or_insert_with(Vec::new).push(
            XmlAttribute {
                name: name.into(),
                value: value.into(),
            }
            .to_xml(),
        );
        self
    }

    pub(crate) fn with_element(mut self, element: &impl ToXml) -> Self {
        self.body.push(element.to_xml());
        self
    }

    pub(crate) fn with_text_element<'a>(
        mut self,
        name: impl Into<Cow<'a, str>>,
        text: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.body.push(XmlElement::new(name).with_text(text).to_xml());
        self
    }

    pub(crate) fn with_text<'a>(mut self, text: impl Into<Cow<'a, str>>) -> Self {
        self.body.push(XmlText { text: text.into() }.to_xml());
        self
    }
}

impl ToXml for XmlElement {
    fn to_xml(&self) -> String {
        let attrs = self.attrs.as_ref().map_or("".to_string(), |a| a.join(" "));

        let body = self.body.join("");

        format!("<{}{}{attrs}>{body}</{}>", self.name, if !attrs.is_empty() { " " } else { "" }, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml() {
        let mut t1 = Cow::from("&");
        xml_escape(&mut t1);
        let mut t2 = Cow::from("\"");
        xml_escape(&mut t2);
        let mut t3 = Cow::from("'");
        xml_escape(&mut t3);
        let mut t4 = Cow::from("<");
        xml_escape(&mut t4);
        let mut t5 = Cow::from(">");
        xml_escape(&mut t5);
        let mut t6 = Cow::from("&\"");
        xml_escape(&mut t6);
        let mut t7 = Cow::from("&ü\"");
        xml_escape(&mut t7);
        let mut t8 = Cow::from("<test foo=\"bar\">baz</test>");
        xml_escape(&mut t8);
        assert_eq!(t1.as_ref(), "&amp;");
        assert_eq!(t2.as_ref(), "&quot;");
        assert_eq!(t3.as_ref(), "&apos;");
        assert_eq!(t4.as_ref(), "&lt;");
        assert_eq!(t5.as_ref(), "&gt;");
        assert_eq!(t6.as_ref(), "&amp;&quot;");
        assert_eq!(t7.as_ref(), "&amp;ü&quot;");
        assert_eq!(t8.as_ref(), "&lt;test foo=&quot;bar&quot;&gt;baz&lt;/test&gt;");
    }

    #[test]
    fn generates_xml_element() {
        assert_eq!(XmlElement::new("foo").to_xml(), "<foo></foo>");
    }

    #[test]
    fn generates_xml_text() {
        assert_eq!(XmlElement::new("foo").with_text("<bar&>").to_xml(), "<foo>&lt;bar&amp;&gt;</foo>");
    }

    #[test]
    fn generates_xml_texts() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("b")
                .with_text("a")
                .with_text("r")
                .to_xml(),
            "<foo>bar</foo>"
        );
    }

    #[test]
    fn generates_xml_attribute() {
        assert_eq!(XmlElement::new("foo").with_attr("a", "b").with_text("bar").to_xml(), "<foo a=\"b\">bar</foo>");
    }

    #[test]
    fn generates_xml_attributes() {
        assert_eq!(
            XmlElement::new("foo")
                .with_attr("a", "b")
                .with_attr("c", "d")
                .with_text("bar")
                .to_xml(),
            "<foo a=\"b\" c=\"d\">bar</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element() {
        assert_eq!(XmlElement::new("foo").with_element(&XmlElement::new("a")).to_xml(), "<foo><a></a></foo>");
    }

    #[test]
    fn generates_nested_xml_element_with_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_element(&XmlElement::new("a").with_text("b"))
                .to_xml(),
            "<foo><a>b</a></foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_before_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_element(&XmlElement::new("a"))
                .with_text("b")
                .to_xml(),
            "<foo><a></a>b</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_between_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("a")
                .with_element(&XmlElement::new("b"))
                .with_text("c")
                .to_xml(),
            "<foo>a<b></b>c</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_after_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("a")
                .with_element(&XmlElement::new("b"))
                .to_xml(),
            "<foo>a<b></b></foo>"
        );
    }

    #[test]
    fn generates_escaped_xml() {
        assert_eq!(
            XmlElement::new("a")
                .with_attr("foo", "b<>ar")
                .with_element(&XmlElement::new("b").with_attr("c", "\"d&e"))
                .to_xml(),
            "<a foo=\"b&lt;&gt;ar\"><b c=\"&quot;d&amp;e\"></b></a>"
        );
    }

    #[test]
    fn escapes_with_text_element() {
        assert_eq!(
            XmlElement::new("a&b").with_text_element("c&d", "f&g").to_xml(),
            "<a&amp;b><c&amp;d>f&amp;g</c&amp;d></a&amp;b>"
        );
    }
}
