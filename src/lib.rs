pub mod tax;

use std::collections::HashMap;

use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

use tax::{TaxCategory, TaxItem};

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

pub struct Contact<'a> {
    pub salutation: Option<&'a str>,
    pub name: &'a str,
    pub phone: Option<Vec<&'a str>>,
    pub email: Option<Vec<&'a str>>,
}

impl Contact<'_> {
    fn as_xml(&self) -> String {
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

pub struct OrderReference<'a> {
    pub order_id: &'a str,
    pub reference_date: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl OrderReference<'_> {
    fn as_xml(&self) -> String {
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

pub struct Biller<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Vec<FurtherIdentification<'a>>,
    pub order_reference: Option<OrderReference<'a>>,
    pub address: Option<Address<'a>>,
    pub contact: Option<Contact<'a>>,
}

impl Biller<'_> {
    fn as_xml(&self) -> String {
        let vat_identification_number = self.vat_identification_number;
        let further_identification_vec: Vec<String> = (&self.further_identification)
            .into_iter()
            .map(|id| id.as_xml())
            .collect();
        let further_identification = further_identification_vec.join("");
        let order_reference = match &self.order_reference {
            Some(order_reference) => order_reference.as_xml(),
            None => format!(""),
        };
        let address = match &self.address {
            Some(address) => address.as_xml(),
            None => format!(""),
        };
        let contact = match &self.contact {
            Some(contact) => contact.as_xml(),
            None => format!(""),
        };
        format!("<Biller><VATIdentificationNumber>{vat_identification_number}</VATIdentificationNumber>{further_identification}{order_reference}{address}{contact}</Biller>")
    }
}

pub struct InvoiceRecipient<'a> {
    pub vat_identification_number: &'a str,
    pub further_identification: Vec<FurtherIdentification<'a>>,
    pub order_reference: Option<OrderReference<'a>>,
    pub address: Option<Address<'a>>,
    pub contact: Option<Contact<'a>>,
}

impl InvoiceRecipient<'_> {
    fn as_xml(&self) -> String {
        let vat_identification_number = self.vat_identification_number;
        let further_identification_vec: Vec<String> = (&self.further_identification)
            .into_iter()
            .map(|id| id.as_xml())
            .collect();
        let further_identification = further_identification_vec.join("");
        let order_reference = match &self.order_reference {
            Some(order_reference) => order_reference.as_xml(),
            None => format!(""),
        };
        let address = match &self.address {
            Some(address) => address.as_xml(),
            None => format!(""),
        };
        let contact = match &self.contact {
            Some(contact) => contact.as_xml(),
            None => format!(""),
        };
        format!("<InvoiceRecipient><VATIdentificationNumber>{vat_identification_number}</VATIdentificationNumber>{further_identification}{order_reference}{address}{contact}</InvoiceRecipient>")
    }
}

pub struct DetailsItem<'a> {
    pub position_number: Option<u64>,
    pub description: Vec<&'a str>,
    pub quantity: Decimal,
    pub unit: &'a str,
    pub unit_price: Decimal,
    pub base_quantity: Option<Decimal>,
    pub tax_item: TaxItem,
}

impl DetailsItem<'_> {
    fn line_item_amount(&self) -> Decimal {
        let base_quantity = match self.base_quantity {
            Some(bq) => bq,
            None => Decimal::ONE,
        };
        self.quantity * self.unit_price / base_quantity /* + sum of surcharge_list_line_item.amount + sum of other_vat_able_tax_list_line_item.tax_amount - sum of reduction_list_line_item.amount */
    }

    fn line_item_total_gross_amount(&self) -> Decimal {
        self.line_item_amount()
            * ((self.tax_item.tax_percent + Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED)
    }

    fn as_xml(&self) -> String {
        let position_number = match self.position_number {
            Some(pn) => format!("<PositionNumber>{pn}</PositionNumber>"),
            None => format!(""),
        };
        let description_vec: Vec<String> = (&self.description)
            .into_iter()
            .map(|d| format!("<Description>{d}</Description>"))
            .collect();
        let description = description_vec.join("");
        let unit: &str = self.unit;
        let base_quantity = match self.base_quantity {
            Some(bq) => format!(" BaseQuantity=\"{bq}\""),
            None => format!(""),
        };
        let tax_item_xml = self.tax_item.as_xml();
        format!("<ListLineItem>{position_number}{description}<Quantity Unit=\"{unit}\">{:.4}</Quantity><UnitPrice{base_quantity}>{:.4}</UnitPrice>{tax_item_xml}<LineItemAmount>{:.2}</LineItemAmount></ListLineItem>", self.quantity.round_dp_with_strategy(4, MidpointAwayFromZero), self.unit_price.round_dp_with_strategy(4, MidpointAwayFromZero), self.line_item_amount().round_dp_with_strategy(2, MidpointAwayFromZero))
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
    let mut tax_items: HashMap<(Decimal, TaxCategory), Decimal> = HashMap::new();
    for i in &details.items {
        let k = (i.tax_item.tax_percent, i.tax_item.tax_category.clone());
        let s = match tax_items.get(&k) {
            Some(v) => v.clone(),
            None => Decimal::ZERO,
        };
        tax_items.insert(k, s + i.line_item_amount());
    }

    // To get consistent results, sort by keys (tax_percent and tax_category).
    let mut sorted_tax_item_entries: Vec<((Decimal, TaxCategory), Decimal)> =
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

    let total_gross_amount = (&details.items).into_iter().fold(Decimal::ZERO, |sum, i| sum + i.line_item_total_gross_amount()) /* + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
    let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

    String::from(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><{document_type_str} xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"{generating_system}\" DocumentType=\"{document_type_str}\" InvoiceCurrency=\"{invoice_currency}\" DocumentTitle=\"{document_title}\" Language=\"{language}\"><InvoiceNumber>{invoice_number}</InvoiceNumber><InvoiceDate>{invoice_date}</InvoiceDate>{biller_xml}{invoice_recipient_xml}{details_xml}<Tax>{tax_items_xml}</Tax><TotalGrossAmount>{:.2}</TotalGrossAmount><PayableAmount>{:.2}</PayableAmount></{document_type_str}>", total_gross_amount.round_dp_with_strategy(2, MidpointAwayFromZero), payable_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn round_line_item_amount_result_after_calculation() {
        let quantity = dec!(0.005);
        let unit_price = dec!(0.005);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">0.0050</Quantity><UnitPrice>0.0050</UnitPrice><TaxItem><TaxableAmount>0.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>0.00</TaxAmount></TaxItem><LineItemAmount>0.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_up() {
        let quantity = dec!(100.123456);
        let unit_price = dec!(10.20005);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1235</Quantity><UnitPrice>10.2001</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_down() {
        let quantity = dec!(100.12344);
        let unit_price = dec!(10.20001);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1234</Quantity><UnitPrice>10.2000</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }

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
                order_reference: None,
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
                contact: None,
            },
            InvoiceRecipient {
                vat_identification_number: "ATU18708634",
                further_identification: vec![],
                order_reference: Some(OrderReference {
                    order_id: "test",
                    reference_date: None,
                    description: None,
                }),
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
                contact: Some(Contact {
                    salutation: None,
                    name: "Max Mustermann",
                    phone: None,
                    email: Some(vec!["schrauben@mustermann.at"]),
                }),
            },
            Details {
                items: vec![
                    DetailsItem {
                        position_number: Some(1),
                        description: vec!["Schraubenzieher"],
                        quantity: dec!(100),
                        unit: "STK",
                        unit_price: dec!(10.20),
                        base_quantity: Some(dec!(1)),
                        tax_item: TaxItem {
                            taxable_amount: dec!(1020.00),
                            tax_percent: dec!(20),
                            tax_category: TaxCategory::S,
                        },
                    },
                    DetailsItem {
                        position_number: Some(2),
                        description: vec!["Handbuch zur Schraube"],
                        quantity: dec!(1),
                        unit: "STK",
                        unit_price: dec!(5.00),
                        base_quantity: Some(dec!(1)),
                        tax_item: TaxItem {
                            taxable_amount: dec!(5.00),
                            tax_percent: dec!(10),
                            tax_category: TaxCategory::AA,
                        },
                    },
                ],
            },
        );

        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"test\" DocumentType=\"Invoice\" InvoiceCurrency=\"EUR\" DocumentTitle=\"An invoice\" Language=\"de\"><InvoiceNumber>993433000298</InvoiceNumber><InvoiceDate>2020-01-01</InvoiceDate><Biller><VATIdentificationNumber>ATU51507409</VATIdentificationNumber><FurtherIdentification IdentificationType=\"DVR\">0012345</FurtherIdentification><Address><Name>Schrauben Mustermann</Name><Street>Lassallenstraße 5</Street><Town>Wien</Town><ZIP>1020</ZIP><Country CountryCode=\"AT\">Österreich</Country><Phone>+43 / 1 / 78 56 789</Phone><Email>schrauben@mustermann.at</Email></Address></Biller><InvoiceRecipient><VATIdentificationNumber>ATU18708634</VATIdentificationNumber><OrderReference><OrderID>test</OrderID></OrderReference><Address><Name>Mustermann GmbH</Name><Street>Hauptstraße 10</Street><Town>Graz</Town><ZIP>8010</ZIP><Country CountryCode=\"AT\">Österreich</Country></Address><Contact><Name>Max Mustermann</Name><Email>schrauben@mustermann.at</Email></Contact></InvoiceRecipient><Details><ItemList><ListLineItem><PositionNumber>1</PositionNumber><Description>Schraubenzieher</Description><Quantity Unit=\"STK\">100.0000</Quantity><UnitPrice BaseQuantity=\"1\">10.2000</UnitPrice><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem><LineItemAmount>1020.00</LineItemAmount></ListLineItem><ListLineItem><PositionNumber>2</PositionNumber><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice BaseQuantity=\"1\">5.0000</UnitPrice><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><LineItemAmount>5.00</LineItemAmount></ListLineItem></ItemList></Details><Tax><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem></Tax><TotalGrossAmount>1229.50</TotalGrossAmount><PayableAmount>1229.50</PayableAmount></Invoice>"
        );
    }
}
