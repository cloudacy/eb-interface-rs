use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref XML_ESCAPE_REGEX: Regex = Regex::new("[&\"'<>]").unwrap();
}

fn xml_escape<'a>(s: impl AsRef<str>) -> String {
    let r = s.as_ref();
    if let Some(m) = XML_ESCAPE_REGEX.find(r) {
        let mut o = r[0..m.start()].to_string();
        for c in (&r[m.start()..]).chars() {
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

    pub fn with_boxed_text_element(
        mut self,
        name: &'a str,
        text: Box<impl AsRef<str> + 'a>,
    ) -> Self {
        self.body.push(Box::new(XmlElement {
            name,
            body: vec![Box::new(XmlText { text })],
            ..Default::default()
        }));

        self
    }

    pub fn with_text_element(self, name: &'a str, text: &'a str) -> Self {
        self.with_boxed_text_element(name, Box::new(text))
    }

    pub fn with_boxed_text(mut self, text: Box<impl AsRef<str> + 'a>) -> Self {
        self.body.push(Box::new(XmlText { text }));

        self
    }

    pub fn with_text(self, text: &'a str) -> Self {
        self.with_boxed_text(Box::new(text))
    }
}

impl<'a> XmlToString for XmlElement<'a> {
    fn to_string(&self) -> String {
        let name = xml_escape(&self.name);
        let mut attrs: String = match &self.attrs {
            Some(attrs) => {
                if attrs.len() < 1 {
                    return "".to_string();
                }

                let attr_str_vec: Vec<String> = attrs.into_iter().map(|a| a.to_string()).collect();
                attr_str_vec.join(" ")
            }
            None => "".to_string(),
        };

        let body_str_vec: Vec<String> = (&self.body).into_iter().map(|e| e.to_string()).collect();
        let body = body_str_vec.join("");

        if attrs.len() > 0 {
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
    fn generates_xml() {
        assert_eq!(
            XmlElement::new("test")
                .with_attr("foo", "bar")
                .with_text("baz")
                .to_string(),
            "<test foo=\"bar\">baz</test>"
        );
        assert_eq!(
            XmlElement::new("a")
                .with_attr("foo", "bar")
                .with_element(XmlElement::new("b").with_attr("c", "d&e"))
                .to_string(),
            "<a foo=\"bar\"><b c=\"d&amp;e\"></b></a>"
        );
    }
}
