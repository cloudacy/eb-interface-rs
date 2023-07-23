pub mod address;
pub mod biller;
pub mod contact;
pub mod details;
pub mod document;
pub mod identification;
pub mod invoice;
pub mod invoice_recipient;
pub mod order_reference;
pub mod tax;
pub mod xml;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use address::Address;
    use biller::Biller;
    use contact::Contact;
    use details::{Details, DetailsItem};
    use identification::{FurtherIdentification, FurtherIdentificationType};
    use invoice::Invoice;
    use invoice_recipient::InvoiceRecipient;
    use order_reference::OrderReference;
    use tax::{TaxCategory, TaxItem};
    use xml::XmlAsString;

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
            result.as_str(),
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
            result.as_str(),
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
            result.as_str(),
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1234</Quantity><UnitPrice>10.2000</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn it_works() {
        let result = Invoice::new(
            "test",
            "EUR",
            "An invoice",
            "de",
            "993433000298",
            "2020-01-01",
            Biller {
                vat_identification_number: "ATU51507409",
                further_identification: Some(vec![FurtherIdentification {
                    id: "0012345",
                    id_type: FurtherIdentificationType::DVR,
                }]),
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
                further_identification: None,
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
        )
        .as_xml();

        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"test\" DocumentType=\"Invoice\" InvoiceCurrency=\"EUR\" DocumentTitle=\"An invoice\" Language=\"de\"><InvoiceNumber>993433000298</InvoiceNumber><InvoiceDate>2020-01-01</InvoiceDate><Biller><VATIdentificationNumber>ATU51507409</VATIdentificationNumber><FurtherIdentification IdentificationType=\"DVR\">0012345</FurtherIdentification><Address><Name>Schrauben Mustermann</Name><Street>Lassallenstraße 5</Street><Town>Wien</Town><ZIP>1020</ZIP><Country CountryCode=\"AT\">Österreich</Country><Phone>+43 / 1 / 78 56 789</Phone><Email>schrauben@mustermann.at</Email></Address></Biller><InvoiceRecipient><VATIdentificationNumber>ATU18708634</VATIdentificationNumber><OrderReference><OrderID>test</OrderID></OrderReference><Address><Name>Mustermann GmbH</Name><Street>Hauptstraße 10</Street><Town>Graz</Town><ZIP>8010</ZIP><Country CountryCode=\"AT\">Österreich</Country></Address><Contact><Name>Max Mustermann</Name><Email>schrauben@mustermann.at</Email></Contact></InvoiceRecipient><Details><ItemList><ListLineItem><PositionNumber>1</PositionNumber><Description>Schraubenzieher</Description><Quantity Unit=\"STK\">100.0000</Quantity><UnitPrice BaseQuantity=\"1\">10.2000</UnitPrice><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem><LineItemAmount>1020.00</LineItemAmount></ListLineItem><ListLineItem><PositionNumber>2</PositionNumber><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice BaseQuantity=\"1\">5.0000</UnitPrice><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><LineItemAmount>5.00</LineItemAmount></ListLineItem></ItemList></Details><Tax><TaxItem><TaxableAmount>5.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.50</TaxAmount></TaxItem><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem></Tax><TotalGrossAmount>1229.50</TotalGrossAmount><PayableAmount>1229.50</PayableAmount></Invoice>"
        );
    }
}
