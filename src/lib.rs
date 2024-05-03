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
pub mod utils;
pub mod xml;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use address::Address;
    use biller::Biller;
    use contact::Contact;
    use details::DetailsItem;
    use identification::{FurtherIdentification, FurtherIdentificationType};
    use invoice::Invoice;
    use invoice_recipient::InvoiceRecipient;
    use order_reference::OrderReference;
    use payment_method::{PaymentMethod, PaymentMethodPaymentCard};
    use reduction_and_surcharge::{ReductionAndSurchargeValue, ReductionListLineItem};
    use tax::{TaxCategory, TaxItem};

    #[test]
    fn it_works() {
        let result = Invoice::new(
            "test",
            "EUR",
            "993433000298",
            "2020-01-01",
            Biller::new("ATU51507409")
                .with_further_identification(FurtherIdentification::new(
                    "0012345",
                    FurtherIdentificationType::DVR,
                ))
                .with_address(
                    Address::new("Schrauben Mustermann", "Wien", "1020", "Österreich")
                        .with_street("Lassallenstraße 5")
                        .with_country_code("AT")
                        .with_phone("+43 / 1 / 78 56 789")
                        .with_email("schrauben@mustermann.at"),
                ),
            InvoiceRecipient::new("ATU18708634")
                .with_order_reference(OrderReference::new("test"))
                .with_address(
                    Address::new("Mustermann GmbH", "Graz", "8010", "Österreich")
                        .with_street("Hauptstraße 10")
                        .with_country_code("AT"),
                )
                .with_contact(Contact::new("Max Mustermann").with_email("schrauben@mustermann.at")),
        )
        .with_item(
            DetailsItem::new(
                dec!(100),
                "STK",
                dec!(10.20),
                TaxItem::new(dec!(20), TaxCategory::S),
            )
            .with_position_number(1)
            .with_description("Schraubenzieher")
            .with_base_quantity(dec!(1)),
        )
        .with_item(
            DetailsItem::new(
                dec!(1),
                "STK",
                dec!(5.00),
                TaxItem::new(dec!(10), TaxCategory::AA),
            )
            .with_position_number(2)
            .with_description("Handbuch zur Schraube")
            .with_base_quantity(dec!(1))
            .with_reduction(
                ReductionListLineItem::new(dec!(5), ReductionAndSurchargeValue::Amount(dec!(2)))
                    .with_comment("reduction"),
            ),
        )
        .with_document_title("An invoice")
        .with_language("de")
        .with_payment_method(
            PaymentMethod::payment_card(
                PaymentMethodPaymentCard::new("123456*4321").with_card_holder_name("Name"),
            )
            .with_comment("Comment"),
        )
        .to_xml_string()
        .unwrap();

        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"test\" DocumentType=\"Invoice\" InvoiceCurrency=\"EUR\" DocumentTitle=\"An invoice\" Language=\"de\"><InvoiceNumber>993433000298</InvoiceNumber><InvoiceDate>2020-01-01</InvoiceDate><Biller><VATIdentificationNumber>ATU51507409</VATIdentificationNumber><FurtherIdentification IdentificationType=\"DVR\">0012345</FurtherIdentification><Address><Name>Schrauben Mustermann</Name><Street>Lassallenstraße 5</Street><Town>Wien</Town><ZIP>1020</ZIP><Country CountryCode=\"AT\">Österreich</Country><Phone>+43 / 1 / 78 56 789</Phone><Email>schrauben@mustermann.at</Email></Address></Biller><InvoiceRecipient><VATIdentificationNumber>ATU18708634</VATIdentificationNumber><OrderReference><OrderID>test</OrderID></OrderReference><Address><Name>Mustermann GmbH</Name><Street>Hauptstraße 10</Street><Town>Graz</Town><ZIP>8010</ZIP><Country CountryCode=\"AT\">Österreich</Country></Address><Contact><Name>Max Mustermann</Name><Email>schrauben@mustermann.at</Email></Contact></InvoiceRecipient><Details><ItemList><ListLineItem><PositionNumber>1</PositionNumber><Description>Schraubenzieher</Description><Quantity Unit=\"STK\">100.0000</Quantity><UnitPrice BaseQuantity=\"1\">10.2000</UnitPrice><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem><LineItemAmount>1020.00</LineItemAmount></ListLineItem><ListLineItem><PositionNumber>2</PositionNumber><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice BaseQuantity=\"1\">5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.00</Amount><Comment>reduction</Comment></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>3.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.30</TaxAmount></TaxItem><LineItemAmount>3.00</LineItemAmount></ListLineItem></ItemList></Details><Tax><TaxItem><TaxableAmount>3.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.30</TaxAmount></TaxItem><TaxItem><TaxableAmount>1020.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.00</TaxAmount></TaxItem></Tax><TotalGrossAmount>1227.30</TotalGrossAmount><PayableAmount>1227.30</PayableAmount><PaymentMethod><Comment>Comment</Comment><PaymentCard><PrimaryAccountNumber>123456*4321</PrimaryAccountNumber><CardHolderName>Name</CardHolderName></PaymentCard></PaymentMethod></Invoice>"
        );
    }
}
