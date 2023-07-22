pub struct Contact<'a> {
    pub salutation: Option<&'a str>,
    pub name: &'a str,
    pub phone: Option<Vec<&'a str>>,
    pub email: Option<Vec<&'a str>>,
}

impl Contact<'_> {
    pub fn as_xml(&self) -> String {
        let salutation = match self.salutation {
            Some(s) => format!("<Salutation>{s}</Salutation>"),
            None => format!(""),
        };
        let name = self.name;
        let phone = match &self.phone {
            Some(p) => {
                let p_vec: Vec<String> = p
                    .into_iter()
                    .map(|p| format!("<Phone>{p}</Phone>"))
                    .collect();
                p_vec.join("")
            }
            None => format!(""),
        };
        let email = match &self.email {
            Some(e) => {
                let e_vec: Vec<String> = e
                    .into_iter()
                    .map(|e| format!("<Email>{e}</Email>"))
                    .collect();
                e_vec.join("")
            }
            None => format!(""),
        };
        format!("<Contact>{salutation}<Name>{name}</Name>{phone}{email}</Contact>")
    }
}
