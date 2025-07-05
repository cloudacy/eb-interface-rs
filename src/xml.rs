use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

static XML_ESCAPE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[&\"'<>]").unwrap());

fn xml_escape(s: &mut Cow<str>) {
    let r = s.as_ref();
    if let Some(m) = XML_ESCAPE_REGEX.find(r) {
        let mut o = r[0..m.start()].to_string();
        for c in r[m.start()..].chars() {
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
    fn to_xml(&self) -> String {
        let mut text = self.text.clone();
        xml_escape(&mut text);
        text.to_string()
    }
}

struct XmlAttribute<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl XmlAttribute<'_> {
    fn to_xml(&self) -> String {
        let mut name = self.name.clone();
        let mut value = self.value.clone();
        xml_escape(&mut name);
        xml_escape(&mut value);
        format!("{name}=\"{value}\"")
    }
}

#[derive(Default)]
pub(crate) struct XmlElement {
    name: String,
    attrs: Option<Vec<String>>,
    body: Vec<String>,
}

impl XmlElement {
    pub(crate) fn new(name: &str) -> Self {
        XmlElement {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn with_attr<'a>(
        mut self,
        name: impl AsRef<str> + 'a,
        value: impl AsRef<str> + 'a,
    ) -> Self {
        self.attrs.get_or_insert_with(Vec::new).push(
            XmlAttribute {
                name: Cow::from(name.as_ref()),
                value: Cow::from(value.as_ref()),
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
        name: &'a str,
        text: impl AsRef<str> + 'a,
    ) -> Self {
        self.body.push(
            XmlElement {
                name: name.to_owned(),
                body: vec![
                    XmlText {
                        text: Cow::from(text.as_ref()),
                    }
                    .to_xml(),
                ],
                ..Default::default()
            }
            .to_xml(),
        );
        self
    }

    pub(crate) fn with_text<'a>(mut self, text: impl AsRef<str> + 'a) -> Self {
        self.body.push(
            XmlText {
                text: Cow::from(text.as_ref()),
            }
            .to_xml(),
        );
        self
    }
}

impl ToXml for XmlElement {
    fn to_xml(&self) -> String {
        let mut name = Cow::from(&self.name);
        xml_escape(&mut name);

        let attrs = self.attrs.as_ref().map_or("".to_string(), |a| a.join(" "));

        let body = self.body.join("");

        format!(
            "<{name}{}{attrs}>{body}</{name}>",
            if !attrs.is_empty() { " " } else { "" }
        )
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
        assert_eq!(
            t8.as_ref(),
            "&lt;test foo=&quot;bar&quot;&gt;baz&lt;/test&gt;"
        );
    }

    #[test]
    fn generates_xml_element() {
        assert_eq!(XmlElement::new("foo").to_xml(), "<foo></foo>");
    }

    #[test]
    fn generates_xml_text() {
        assert_eq!(
            XmlElement::new("foo").with_text("<bar&>").to_xml(),
            "<foo>&lt;bar&amp;&gt;</foo>"
        );
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
        assert_eq!(
            XmlElement::new("foo")
                .with_attr("a", "b")
                .with_text("bar")
                .to_xml(),
            "<foo a=\"b\">bar</foo>"
        );
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
        assert_eq!(
            XmlElement::new("foo")
                .with_element(&XmlElement::new("a"))
                .to_xml(),
            "<foo><a></a></foo>"
        );
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
}
