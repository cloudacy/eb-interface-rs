fn xml_escape(s: String) -> String {
    s.replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub trait XmlAsString {
    fn as_str(&self) -> String;
}

struct XmlText {
    text: String,
}

impl XmlAsString for XmlText {
    fn as_str(&self) -> String {
        xml_escape(self.text.to_owned())
    }
}

struct XmlAttribute {
    name: String,
    value: String,
}

impl XmlAsString for XmlAttribute {
    fn as_str(&self) -> String {
        format!(
            "{}=\"{}\"",
            xml_escape(self.name.to_owned()),
            xml_escape(self.value.to_owned())
        )
    }
}

pub struct XmlElement {
    name: String,
    attrs: Option<Vec<XmlAttribute>>,
    body: Vec<Box<dyn XmlAsString>>,
}

impl XmlElement {
    pub fn new(name: &str) -> XmlElement {
        XmlElement {
            name: name.to_owned(),
            attrs: None,
            body: vec![],
        }
    }

    pub fn with_attr(mut self, name: &str, value: &str) -> XmlElement {
        let attr = XmlAttribute {
            name: name.to_owned(),
            value: value.to_owned(),
        };

        match &mut self.attrs {
            Some(attrs) => attrs.push(attr),
            None => self.attrs = Some(vec![attr]),
        }

        self
    }

    pub fn with_element(mut self, element: impl XmlAsString + 'static) -> XmlElement {
        self.body.push(Box::new(element));

        self
    }

    pub fn with_text_element(mut self, name: &str, text: &str) -> XmlElement {
        self.body.push(Box::new(XmlElement {
            name: name.to_owned(),
            attrs: None,
            body: vec![Box::new(XmlText {
                text: text.to_owned(),
            })],
        }));

        self
    }

    pub fn with_text(mut self, text: &str) -> XmlElement {
        self.body.push(Box::new(XmlText {
            text: text.to_owned(),
        }));

        self
    }
}

impl XmlAsString for XmlElement {
    fn as_str(&self) -> String {
        let name = xml_escape(self.name.to_owned());
        let mut attrs: String = match &self.attrs {
            Some(attrs) => {
                if attrs.len() < 1 {
                    return "".to_owned();
                }

                let attr_str_vec: Vec<String> = attrs.into_iter().map(|a| a.as_str()).collect();
                attr_str_vec.join(" ")
            }
            None => "".to_owned(),
        };

        let body_str_vec: Vec<String> = (&self.body).into_iter().map(|e| e.as_str()).collect();
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
        assert_eq!(xml_escape("&".to_owned()), "&amp;");
        assert_eq!(xml_escape("\"".to_owned()), "&quot;");
        assert_eq!(xml_escape("'".to_owned()), "&apos;");
        assert_eq!(xml_escape("<".to_owned()), "&lt;");
        assert_eq!(xml_escape(">".to_owned()), "&gt;");
        assert_eq!(xml_escape("&\"".to_owned()), "&amp;&quot;");
        assert_eq!(
            xml_escape("<test foo=\"bar\">baz</test>".to_owned()),
            "&lt;test foo=&quot;bar&quot;&gt;baz&lt;/test&gt;"
        );
    }

    #[test]
    fn generates_xml() {
        assert_eq!(
            XmlElement::new("test")
                .with_attr("foo", "bar")
                .with_text("baz")
                .as_str(),
            "<test foo=\"bar\">baz</test>"
        );
        assert_eq!(
            XmlElement::new("a")
                .with_attr("foo", "bar")
                .with_element(XmlElement::new("b").with_attr("c", "d&e"))
                .as_str(),
            "<a foo=\"bar\"><b c=\"d&amp;e\"></b></a>"
        );
    }
}
