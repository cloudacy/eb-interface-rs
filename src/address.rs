pub struct Address<'a> {
    pub name: &'a str,
    pub street: Option<&'a str>,
    pub town: &'a str,
    pub zip: &'a str,
    pub country: &'a str,
    pub country_code: Option<&'a str>,
    pub phone: Option<Vec<&'a str>>,
    pub email: Option<Vec<&'a str>>,
}

impl Address<'_> {
    pub fn as_xml(&self) -> String {
        let name = self.name;
        let street = match self.street {
            Some(s) => s,
            None => "",
        };
        let town = self.town;
        let zip = self.zip;
        let country = self.country;
        let country_code = match self.country_code {
            Some(cc) => format!(" CountryCode=\"{cc}\""),
            None => format!(""),
        };
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
        format!("<Address><Name>{name}</Name><Street>{street}</Street><Town>{town}</Town><ZIP>{zip}</ZIP><Country{country_code}>{country}</Country>{phone}{email}</Address>")
    }
}
