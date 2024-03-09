use once_cell::sync::Lazy;
use regex::Regex;

static XML_ESCAPE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[&\"'<>]").unwrap());

fn xml_escape(s: impl AsRef<str>) -> String {
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
        o
    } else {
        r.to_string()
    }
}

pub trait XmlToString {
    fn to_string(&self) -> String;
}

struct XmlText<'a> {
    text: Box<dyn AsRef<str> + 'a>,
}

impl<'a> XmlToString for XmlText<'a> {
    fn to_string(&self) -> String {
        xml_escape(self.text.as_ref())
    }
}

struct XmlAttribute<'a> {
    name: Box<dyn AsRef<str> + 'a>,
    value: Box<dyn AsRef<str> + 'a>,
}

impl XmlToString for XmlAttribute<'_> {
    fn to_string(&self) -> String {
        format!(
            "{}=\"{}\"",
            xml_escape(self.name.as_ref()),
            xml_escape(self.value.as_ref())
        )
    }
}

#[derive(Default)]
pub struct XmlElement<'a> {
    name: &'a str,
    attrs: Option<Vec<XmlAttribute<'a>>>,
    body: Vec<Box<dyn XmlToString + 'a>>,
}

impl<'a> XmlElement<'a> {
    pub fn new(name: &'a str) -> Self {
        XmlElement {
            name,
            ..Default::default()
        }
    }

    pub fn with_attr(mut self, name: impl AsRef<str> + 'a, value: impl AsRef<str> + 'a) -> Self {
        let attr = XmlAttribute {
            name: Box::new(name),
            value: Box::new(value),
        };

        match &mut self.attrs {
            Some(attrs) => attrs.push(attr),
            None => self.attrs = Some(vec![attr]),
        }

        self
    }

    pub fn with_element(mut self, element: impl XmlToString + 'a) -> Self {
        self.body.push(Box::new(element));

        self
    }

    pub fn with_text_element(mut self, name: &'a str, text: impl AsRef<str> + 'a) -> Self {
        self.body.push(Box::new(XmlElement {
            name,
            body: vec![Box::new(XmlText {
                text: Box::new(text),
            })],
            ..Default::default()
        }));

        self
    }

    pub fn with_text(mut self, text: impl AsRef<str> + 'a) -> Self {
        self.body.push(Box::new(XmlText {
            text: Box::new(text),
        }));

        self
    }
}

impl<'a> XmlToString for XmlElement<'a> {
    fn to_string(&self) -> String {
        let name = xml_escape(self.name);
        let mut attrs: String = match &self.attrs {
            Some(attrs) => {
                if attrs.is_empty() {
                    return "".to_string();
                }

                attrs
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            None => "".to_string(),
        };

        let body = self
            .body
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("");

        if !attrs.is_empty() {
            attrs.insert(0, ' ');
        }

        format!("<{name}{attrs}>{body}</{name}>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml() {
        assert_eq!(xml_escape("&"), "&amp;");
        assert_eq!(xml_escape("\""), "&quot;");
        assert_eq!(xml_escape("'"), "&apos;");
        assert_eq!(xml_escape("<"), "&lt;");
        assert_eq!(xml_escape(">"), "&gt;");
        assert_eq!(xml_escape("&\""), "&amp;&quot;");
        assert_eq!(xml_escape("&ü\""), "&amp;ü&quot;");
        assert_eq!(
            xml_escape("<test foo=\"bar\">baz</test>"),
            "&lt;test foo=&quot;bar&quot;&gt;baz&lt;/test&gt;"
        );
    }

    #[test]
    fn generates_xml_element() {
        assert_eq!(XmlElement::new("foo").to_string(), "<foo></foo>");
    }

    #[test]
    fn generates_xml_text() {
        assert_eq!(
            XmlElement::new("foo").with_text("bar").to_string(),
            "<foo>bar</foo>"
        );
    }

    #[test]
    fn generates_xml_texts() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("b")
                .with_text("a")
                .with_text("r")
                .to_string(),
            "<foo>bar</foo>"
        );
    }

    #[test]
    fn generates_xml_attribute() {
        assert_eq!(
            XmlElement::new("foo")
                .with_attr("a", "b")
                .with_text("bar")
                .to_string(),
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
                .to_string(),
            "<foo a=\"b\" c=\"d\">bar</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element() {
        assert_eq!(
            XmlElement::new("foo")
                .with_element(XmlElement::new("a"))
                .to_string(),
            "<foo><a></a></foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_with_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_element(XmlElement::new("a").with_text("b"))
                .to_string(),
            "<foo><a>b</a></foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_before_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_element(XmlElement::new("a"))
                .with_text("b")
                .to_string(),
            "<foo><a></a>b</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_between_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("a")
                .with_element(XmlElement::new("b"))
                .with_text("c")
                .to_string(),
            "<foo>a<b></b>c</foo>"
        );
    }

    #[test]
    fn generates_nested_xml_element_after_text() {
        assert_eq!(
            XmlElement::new("foo")
                .with_text("a")
                .with_element(XmlElement::new("b"))
                .to_string(),
            "<foo>a<b></b></foo>"
        );
    }

    #[test]
    fn generates_escaped_xml() {
        assert_eq!(
            XmlElement::new("a")
                .with_attr("foo", "b<>ar")
                .with_element(XmlElement::new("b").with_attr("c", "\"d&e"))
                .to_string(),
            "<a foo=\"b&lt;&gt;ar\"><b c=\"&quot;d&amp;e\"></b></a>"
        );
    }
}
