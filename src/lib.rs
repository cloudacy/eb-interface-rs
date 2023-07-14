use std::collections::HashMap;

pub enum DocumentType {
    CreditMemo,
    FinalSettlement,
    Invoice,
    InvoiceForAdvancePayment,
    InvoiceForPartialDelivery,
    SelfBilling,
    SubsequentCredit,
    SubsequentDebit,
}

impl DocumentType {
    fn as_str(&self) -> &str {
        match self {
            DocumentType::CreditMemo => "CreditMemo",
            DocumentType::FinalSettlement => "FinalSettlement",
            DocumentType::Invoice => "Invoice",
            DocumentType::InvoiceForAdvancePayment => "InvoiceForAdvancePayment",
            DocumentType::InvoiceForPartialDelivery => "InvoiceForPartialDelivery",
            DocumentType::SelfBilling => "SelfBilling",
            DocumentType::SubsequentCredit => "SubsequentCredit",
            DocumentType::SubsequentDebit => "SubsequentDebit",
        }
    }
}

pub enum FurtherIdentificationType {
    ARA,
    #[allow(non_camel_case_types)]
    BBG_GZ,
    Consolidator,
    Contract,
    DVR,
    EORI,
    ERSB,
    FN,
    FR,
    HG,
    Payer,
    FASTNR,
    VID,
    VN,
}

impl FurtherIdentificationType {
    fn as_str(&self) -> &str {
        match self {
            FurtherIdentificationType::ARA => "ARA",
            FurtherIdentificationType::BBG_GZ => "BBG_GZ",
            FurtherIdentificationType::Consolidator => "Consolidator",
            FurtherIdentificationType::Contract => "Contract",
            FurtherIdentificationType::DVR => "DVR",
            FurtherIdentificationType::EORI => "EORI",
            FurtherIdentificationType::ERSB => "ERSB",
            FurtherIdentificationType::FN => "FN",
            FurtherIdentificationType::FR => "FR",
            FurtherIdentificationType::HG => "HG",
            FurtherIdentificationType::Payer => "Payer",
            FurtherIdentificationType::FASTNR => "FASTNR",
            FurtherIdentificationType::VID => "VID",
            FurtherIdentificationType::VN => "VN",
        }
    }
}

pub struct FurtherIdentification<'a> {
    pub id: &'a str,
    pub id_type: FurtherIdentificationType,
}

impl FurtherIdentification<'_> {
    fn as_xml(&self) -> String {
        let id = self.id;
        let id_type = self.id_type.as_str();
        format!(
            "<FurtherIdentification IdentificationType=\"{id_type}\">{id}</FurtherIdentification>"
        )
    }
}

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
    fn as_xml(&self) -> String {
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

pub struct Biller<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Vec<FurtherIdentification<'a>>,
    pub address: Option<Address<'a>>,
}

impl Biller<'_> {
    fn as_xml(&self) -> String {
        let vat_identification_number = self.vat_identification_number;
        let further_identification_vec: Vec<String> = (&self.further_identification)
            .into_iter()
            .map(|id| id.as_xml())
            .collect();
        let further_identification = further_identification_vec.join("");
        let address = match &self.address {
            Some(address) => address.as_xml(),
            None => format!(""),
        };
        format!("<Biller><VATIdentificationNumber>{vat_identification_number}</VATIdentificationNumber>{further_identification}{address}</Biller>")
    }
}

pub struct InvoiceRecipient<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Vec<FurtherIdentification<'a>>,
    pub address: Option<Address<'a>>,
}

impl InvoiceRecipient<'_> {
    fn as_xml(&self) -> String {
        let vat_identification_number = self.vat_identification_number;
        let further_identification_vec: Vec<String> = (&self.further_identification)
            .into_iter()
            .map(|id| id.as_xml())
            .collect();
        let further_identification = further_identification_vec.join("");
        let address = match &self.address {
            Some(address) => address.as_xml(),
            None => format!(""),
        };
        format!("<InvoiceRecipient><VATIdentificationNumber>{vat_identification_number}</VATIdentificationNumber>{further_identification}{address}</InvoiceRecipient>")
    }
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
pub enum TaxCategory {
    S,
    AA,
    O,
    D,
    E,
    F,
    G,
    I,
    J,
    K,
    AE,
    Z,
}

impl TaxCategory {
    fn as_str(&self) -> &str {
        match self {
            TaxCategory::S => "S",
            TaxCategory::AA => "AA",
            TaxCategory::O => "O",
            TaxCategory::D => "D",
            TaxCategory::E => "E",
            TaxCategory::F => "F",
            TaxCategory::G => "G",
            TaxCategory::I => "I",
            TaxCategory::J => "J",
            TaxCategory::K => "K",
            TaxCategory::AE => "AE",
            TaxCategory::Z => "Z",
        }
    }
}

pub struct TaxItem {
    pub taxable_amount: f64,
    pub tax_percent: i32,
    pub tax_category: TaxCategory,
}

impl TaxItem {
    fn as_xml(&self) -> String {
        let taxable_amount = self.taxable_amount;
        let tax_category_code = self.tax_category.as_str();
        let tax_percent = self.tax_percent;
        let tax_amount = self.taxable_amount * self.tax_percent as f64 / 100.0;
        format!("<TaxItem><TaxableAmount>{taxable_amount:.2}</TaxableAmount><TaxPercent TaxCategoryCode=\"{tax_category_code}\">{tax_percent}</TaxPercent><TaxAmount>{tax_amount:.2}</TaxAmount></TaxItem>")
    }
}

pub struct DetailsItem<'a> {
    pub description: Vec<&'a str>,
    pub quantity: f64,
    pub unit: &'a str,
    pub unit_price: f64,
    pub tax_item: TaxItem,
}

impl DetailsItem<'_> {
    fn line_item_amount(&self) -> f64 {
        self.quantity * self.unit_price /* / self.base_quantity + sum of surcharge_list_line_item.amount + sum of other_vat_able_tax_list_line_item.tax_amount - sum of reduction_list_line_item.amount */
    }

    fn line_item_total_gross_amount(&self) -> f64 {
        self.line_item_amount() * ((self.tax_item.tax_percent + 100) as f64 / 100.0)
    }

    fn as_xml(&self) -> String {
        let description_vec: Vec<String> = (&self.description)
            .into_iter()
            .map(|d| format!("<Description>{d}</Description>"))
            .collect();
        let description = description_vec.join("");
        let quantity = self.quantity;
        let unit = self.unit;
        let unit_price = self.unit_price;
        let line_item_amount = self.line_item_amount();
        let tax_item_xml = self.tax_item.as_xml();
        format!("<ListLineItem>{description}<Quantity Unit=\"{unit}\">{quantity}</Quantity><UnitPrice>{unit_price:.2}</UnitPrice>{tax_item_xml}<LineItemAmount>{line_item_amount:.2}</LineItemAmount></ListLineItem>")
    }
}

pub struct Details<'a> {
    pub items: Vec<DetailsItem<'a>>,
}

impl Details<'_> {
    fn as_xml(&self) -> String {
        let items_xml_vec: Vec<String> = (&self.items).into_iter().map(|l| l.as_xml()).collect();
        let items_xml = items_xml_vec.join("");
        format!("<Details><ItemList>{items_xml}</ItemList></Details>")
    }
}

pub fn generate(
    document_type: DocumentType,
    generating_system: &str,
    invoice_currency: &str,
    document_title: &str,
    language: &str,
    invoice_number: &str,
    invoice_date: &str,
    biller: Biller,
    invoice_recipient: InvoiceRecipient,
    details: Details,
) -> String {
    let document_type_str = document_type.as_str();
    let biller_xml = biller.as_xml();
    let invoice_recipient_xml = invoice_recipient.as_xml();
    let details_xml = details.as_xml();

    // Collect all taxes, grouped by tuples of tax_percent and tax_category.
    let mut tax_items: HashMap<(i32, TaxCategory), f64> = HashMap::new();
    for i in &details.items {
        let k = (i.tax_item.tax_percent, i.tax_item.tax_category.clone());
        let s = match tax_items.get(&k) {
            Some(v) => v.clone(),
            None => 0.0,
        };
        tax_items.insert(k, s + i.line_item_amount());
    }

    // To get consistent results, sort by keys (tax_percent and tax_category).
    let mut sorted_tax_item_entries: Vec<((i32, TaxCategory), f64)> =
        tax_items.into_iter().collect();
    sorted_tax_item_entries.sort_by_key(|k| (k.0 .0, k.0 .1.clone()));

    let tax_items_xml_vec: Vec<String> = sorted_tax_item_entries
        .into_iter()
        .map(|e| {
            TaxItem {
                taxable_amount: e.1,
                tax_percent: e.0 .0,
                tax_category: e.0 .1,
            }
            .as_xml()
        })
        .collect();
    let tax_items_xml = tax_items_xml_vec.join("");

    let total_gross_amount = (&details.items).into_iter().fold(0.0, |sum, i| sum + i.line_item_total_gross_amount()) /* sum of LineItemAmounts + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
    let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

    String::from(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><{document_type_str} xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"{generating_system}\" DocumentType=\"{document_type_str}\" InvoiceCurrency=\"{invoice_currency}\" DocumentTitle=\"{document_title}\" Language=\"{language}\"><InvoiceNumber>{invoice_number}</InvoiceNumber><InvoiceDate>{invoice_date}</InvoiceDate>{biller_xml}{invoice_recipient_xml}{details_xml}<Tax>{tax_items_xml}</Tax><TotalGrossAmount>{total_gross_amount:.2}</TotalGrossAmount><PayableAmount>{payable_amount:.2}</PayableAmount></{document_type_str}>"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate(
            DocumentType::Invoice,
            "test",
            "EUR",
            "An invoice",
            "de",
            "993433000298",
            "2020-01-01",
            Biller {
                vat_identification_number: "ATU51507409",
                further_identification: vec![FurtherIdentification {
                    id: "0012345",
                    id_type: FurtherIdentificationType::DVR,
                }],
                address: Some(Address {
                    name: "Schrauben Mustermann",
                    street: Some("Lassallenstraße 5"),
                    town: "Wien",
                    zip: "1020",
                    country: "Österreich",
                    country_code: Some("AT"),
                    phone: Some(vec!["+43 / 1 / 78 56 789"]),
                    email: Some(vec!["schrauben@mustermann.at"]),
                }),
            },
            InvoiceRecipient {
                vat_identification_number: "ATU18708634",
                further_identification: vec![],
                address: Some(Address {
                    name: "Mustermann GmbH",
                    street: Some("Hauptstraße 10"),
                    town: "Graz",
                    zip: "8010",
                    country: "Österreich",
                    country_code: Some("AT"),
                    phone: None,
                    email: None,
                }),
            },
            Details {
                items: vec![
                    DetailsItem {
                        description: vec!["Schraubenzieher"],
                        quantity: 100.0,
                        unit: "C62",
                        unit_price: 10.20,
                        tax_item: TaxItem {
                            taxable_amount: 1020.0,
                            tax_percent: 20,
                            tax_category: TaxCategory::S,
                        },
                    },
                    DetailsItem {
                        description: vec!["Handbuch zur Schraube"],
                        quantity: 1.0,
                        unit: "C62",
                        unit_price: 5.00,
                        tax_item: TaxItem {
                            taxable_amount: 5.0,
                            tax_percent: 10,
                            tax_category: TaxCategory::S,
                        },
                    },
                ],
            },
        );
        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"test\" DocumentType=\"Invoice\" InvoiceCurrency=\"EUR\" DocumentTitle=\"An invoice\" Language=\"de\"><InvoiceNumber>993433000298</InvoiceNumber><InvoiceDate>2020-01-01</InvoiceDate><Biller><VATIdentificationNumber>ATU51507409</VATIdentificationNumber><FurtherIdentification IdentificationType=\"DVR\">0012345</FurtherIdentification><Address><Name>Schrauben Mustermann</Name><Street>Lassallenstraße 5</Street><Town>Wien</Town><ZIP>1020</ZIP><Country CountryCode=\"AT\">Österreich</Country><Phone>+43 / 1 / 78 56 789</Phone><Email>schrauben@mustermann.at</Email></Address></Biller><InvoiceRecipient><VATIdentificationNumber>ATU18708634</VATIdentificationNumber><Address><Name>Mustermann GmbH</Name><Street>Hauptstraße 10</Street><Town>Graz</Town><ZIP>8010</ZIP><Country CountryCode=\"AT\">Österreich</Country></Address></InvoiceRecipient><Details><ItemList><ListLineItem><Description>Schraubenzieher</Description><Quantity Unit=\"C62\">100</Quantity><UnitPrice>10.20</UnitPrice><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem><LineItemAmount>1020.00</LineItemAmount></ListLineItem><ListLineItem><Description>Handbuch zur Schraube</Description><Quantity Unit=\"C62\">1</Quantity><UnitPrice>5.00</UnitPrice><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><LineItemAmount>5.00</LineItemAmount></ListLineItem></ItemList></Details><Tax><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem></Tax><TotalGrossAmount>1229.50</TotalGrossAmount><PayableAmount>1229.50</PayableAmount></Invoice>"
        );
    }
}
