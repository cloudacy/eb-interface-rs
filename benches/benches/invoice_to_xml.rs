use criterion::{Criterion, criterion_group, criterion_main};
use eb_interface_rs::{
    address::Address,
    biller::Biller,
    contact::Contact,
    details::DetailsItem,
    identification::{FurtherIdentification, FurtherIdentificationType},
    invoice::Invoice,
    invoice_recipient::InvoiceRecipient,
    order_reference::OrderReference,
    payment_method::{PaymentMethod, PaymentMethodPaymentCard},
    reduction_and_surcharge::{ReductionAndSurchargeValue, ReductionListLineItem},
    tax::{TaxCategory, TaxItem},
};
use rust_decimal::Decimal;

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);
criterion_main!(benches);

pub fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("invoice_to_xml");
    g.sample_size(1000);
    g.warm_up_time(core::time::Duration::from_millis(500));
    g.measurement_time(core::time::Duration::from_millis(1000));

    let invoice = Invoice::new(
        "test",
        "EUR",
        "993433000298",
        "2020-01-01",
        Biller::new("ATU51507409")
            .with_further_identification(FurtherIdentification::new("0012345", FurtherIdentificationType::DVR))
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
            Decimal::from(100),
            "STK",
            Decimal::new(1020, 2),
            TaxItem::new(Decimal::from(20), TaxCategory::S),
        )
        .with_position_number(1)
        .with_description("Schraubenzieher")
        .with_base_quantity(Decimal::from(1)),
    )
    .with_item(
        DetailsItem::new(Decimal::from(1), "STK", Decimal::from(5), TaxItem::new(Decimal::from(10), TaxCategory::AA))
            .with_position_number(2)
            .with_description("Handbuch zur Schraube")
            .with_base_quantity(Decimal::from(1))
            .with_reduction(
                ReductionListLineItem::new(Decimal::from(5), ReductionAndSurchargeValue::Amount(Decimal::from(2)))
                    .with_comment("reduction"),
            ),
    )
    .with_document_title("An invoice")
    .with_language("de")
    .with_payment_method(
        PaymentMethod::payment_card(
            PaymentMethodPaymentCard::new("123456*4321")
                .and_then(|s| Ok(s.with_card_holder_name("Name")))
                .unwrap_or_else(|e| panic!("{e}")),
        )
        .with_comment("Comment"),
    );

    g.bench_function("invoice_to_xml", |b| b.iter(|| invoice.to_xml()));
}
