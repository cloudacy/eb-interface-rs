# eb_interface_rs

![status](https://github.com/cloudacy/eb-interface-rs/actions/workflows/rust.yml/badge.svg)

[ebInterface](https://www.wko.at/service/netzwerke/was-ist-ebinterface.html) generator

:warning: **This crate is in alpha stage** :warning:

## Notes

#### rounding

- `eb-interface-rs` rounds after all calculations are done
- `eb-interface-rs` uses the [`MidpointAwayFromZero`](https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointAwayFromZero) rounding strategy

## Feature set

- [x] minimal invoice

## Example

```rust
Invoice::new(
    "test",
    "EUR",
    "993433000298",
    "2020-01-01",
    Biller::new("ATU51507409"),
    InvoiceRecipient::new("ATU18708634"),
)
.with_item(
    DetailsItem::new(
        Decimal::from(100),
        "STK",
        Decimal::new(1020, 2),
        TaxItem::new(Decimal::from(20), TaxCategory::S),
    )
    .with_description("Schraubenzieher"),
)
.with_document_title("An invoice")
.with_language("de")
.with_payment_method(
    PaymentMethod::payment_card(
        PaymentMethodPaymentCard::new("123456*4321")
            .unwrap()
            .with_card_holder_name("Name"),
    )
    .with_comment("Comment"),
)
.to_xml(); // returns "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice>...</Invoice>"
```

## Development

Reference: https://www.wko.at/service/netzwerke/ebinterface-aktuelle-version-xml-rechnungsstandard.html

Validate: https://labs.ebinterface.at/labs

```sh
cargo watch -x test
```
