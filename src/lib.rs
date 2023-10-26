pub mod address;
pub mod biller;
pub mod contact;
pub mod decimal;
pub mod details;
pub mod document;
pub mod identification;
pub mod invoice;
pub mod invoice_recipient;
pub mod order_reference;
pub mod payment_method;
pub mod reduction_and_surcharge;
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
    use payment_method::PaymentMethodPaymentCard;
    use reduction_and_surcharge::{
        ReductionAndSurchargeListLineItemDetails, ReductionAndSurchargeValue, ReductionListLineItem,
    };
    use tax::{TaxCategory, TaxItem};

    #[test]
    fn it_works() {
        let result = Invoice {
            generating_system: "test",
            invoice_currency: "EUR",
            document_title: "An invoice",
            language: "de",
            invoice_number: "993433000298",
            invoice_date: "2020-01-01",
            biller: Biller {
                vat_identification_number: "ATU51507409",
                further_identification: Some(vec![FurtherIdentification {
                    id: "0012345",
                    id_type: FurtherIdentificationType::DVR,
                }]),
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
                ..Default::default()
            },
            invoice_recipient: InvoiceRecipient {
                vat_identification_number: "ATU18708634",
                order_reference: Some(OrderReference {
                    order_id: "test",
                    ..Default::default()
                }),
                address: Some(Address {
                    name: "Mustermann GmbH",
                    street: Some("Hauptstraße 10"),
                    town: "Graz",
                    zip: "8010",
                    country: "Österreich",
                    country_code: Some("AT"),
                    ..Default::default()
                }),
                contact: Some(Contact {
                    name: "Max Mustermann",
                    email: Some(vec!["schrauben@mustermann.at"]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            details: Details {
                items: vec![
                    DetailsItem {
                        position_number: Some(1),
                        description: vec!["Schraubenzieher"],
                        quantity: dec!(100),
                        unit: "STK",
                        unit_price: dec!(10.20),
                        base_quantity: Some(dec!(1)),
                        tax_item: TaxItem {
                            tax_percent: dec!(20),
                            tax_category: TaxCategory::S,
                        },
                        ..Default::default()
                    },
                    DetailsItem {
                        position_number: Some(2),
                        description: vec!["Handbuch zur Schraube"],
                        quantity: dec!(1),
                        unit: "STK",
                        unit_price: dec!(5.00),
                        base_quantity: Some(dec!(1)),
                        reduction_and_surcharge: Some(ReductionAndSurchargeListLineItemDetails {
                            reduction_list_line_items: Some(vec![ReductionListLineItem::new(
                                dec!(5),
                                ReductionAndSurchargeValue::Amount(dec!(2)),
                                Some("reduction"),
                            )]),
                            ..Default::default()
                        }),
                        tax_item: TaxItem {
                            tax_percent: dec!(10),
                            tax_category: TaxCategory::AA,
                        },
                    },
                ],
            },
            ..Default::default()
        }
        .with_payment_method(
            PaymentMethodPaymentCard {
                primary_account_number: "123456*4321",
                card_holder_name: Some("Name"),
            },
            Some("Comment"),
        )
        .to_xml_string()
        .unwrap();

        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"test\" DocumentType=\"Invoice\" InvoiceCurrency=\"EUR\" DocumentTitle=\"An invoice\" Language=\"de\"><InvoiceNumber>993433000298</InvoiceNumber><InvoiceDate>2020-01-01</InvoiceDate><Biller><VATIdentificationNumber>ATU51507409</VATIdentificationNumber><FurtherIdentification IdentificationType=\"DVR\">0012345</FurtherIdentification><Address><Name>Schrauben Mustermann</Name><Street>Lassallenstraße 5</Street><Town>Wien</Town><ZIP>1020</ZIP><Country CountryCode=\"AT\">Österreich</Country><Phone>+43 / 1 / 78 56 789</Phone><Email>schrauben@mustermann.at</Email></Address></Biller><InvoiceRecipient><VATIdentificationNumber>ATU18708634</VATIdentificationNumber><OrderReference><OrderID>test</OrderID></OrderReference><Address><Name>Mustermann GmbH</Name><Street>Hauptstraße 10</Street><Town>Graz</Town><ZIP>8010</ZIP><Country CountryCode=\"AT\">Österreich</Country></Address><Contact><Name>Max Mustermann</Name><Email>schrauben@mustermann.at</Email></Contact></InvoiceRecipient><Details><ItemList><ListLineItem><PositionNumber>1</PositionNumber><Description>Schraubenzieher</Description><Quantity Unit=\"STK\">100.0000</Quantity><UnitPrice BaseQuantity=\"1\">10.2000</UnitPrice><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem><LineItemAmount>1020.00</LineItemAmount></ListLineItem><ListLineItem><PositionNumber>2</PositionNumber><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice BaseQuantity=\"1\">5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.00</Amount><Comment>reduction</Comment></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>3.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.30</TaxAmount></TaxItem><LineItemAmount>3.00</LineItemAmount></ListLineItem></ItemList></Details><PaymentMethod><Comment>Comment</Comment><PaymentCard><PrimaryAccountNumber>123456*4321</PrimaryAccountNumber><CardHolderName>Name</CardHolderName></PaymentCard></PaymentMethod><Tax><TaxItem><TaxableAmount>3.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.30</TaxAmount></TaxItem><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem></Tax><TotalGrossAmount>1227.30</TotalGrossAmount><PayableAmount>1227.30</PayableAmount></Invoice>"
        );
    }
}
