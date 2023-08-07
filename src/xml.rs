fn xml_escape(s: impl AsRef<str>) -> String {
    s.as_ref()
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub trait XmlAsString {
    fn as_str(&self) -> String;
}

struct XmlText<'a> {
    text: Box<dyn AsRef<str> + 'a>,
}

impl<'a> XmlAsString for XmlText<'a> {
    fn as_str(&self) -> String {
        xml_escape(self.text.as_ref())
    }
}

struct XmlAttribute<'a> {
    name: Box<dyn AsRef<str> + 'a>,
    value: Box<dyn AsRef<str> + 'a>,
}

impl XmlAsString for XmlAttribute<'_> {
    fn as_str(&self) -> String {
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
    body: Vec<Box<dyn XmlAsString + 'a>>,
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

    pub fn with_element(mut self, element: impl XmlAsString + 'a) -> Self {
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

impl<'a> XmlAsString for XmlElement<'a> {
    fn as_str(&self) -> String {
        let name = xml_escape(&self.name);
        let mut attrs: String = match &self.attrs {
            Some(attrs) => {
                if attrs.len() < 1 {
                    return "".to_string();
                }

                let attr_str_vec: Vec<String> = attrs.into_iter().map(|a| a.as_str()).collect();
                attr_str_vec.join(" ")
            }
            None => "".to_string(),
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
        assert_eq!(xml_escape("&"), "&amp;");
        assert_eq!(xml_escape("\""), "&quot;");
        assert_eq!(xml_escape("'"), "&apos;");
        assert_eq!(xml_escape("<"), "&lt;");
        assert_eq!(xml_escape(">"), "&gt;");
        assert_eq!(xml_escape("&\""), "&amp;&quot;");
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
