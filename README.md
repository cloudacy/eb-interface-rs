# eb-interface-rs

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
Invoice {
    generating_system: "test",
    invoice_currency: "EUR",
    document_title: "An invoice",
    language: "de",
    invoice_number: "993433000298",
    invoice_date: "2020-01-01",
    biller: Biller {
        vat_identification_number: "ATU51507409",
        ..Default::default()
    },
    invoice_recipient: InvoiceRecipient {
        vat_identification_number: "ATU18708634",
        ..Default::default()
    },
    details: Details {
        items: vec![
            DetailsItem {
                description: vec!["Schraubenzieher"],
                quantity: dec!(100),
                unit: "STK",
                unit_price: dec!(10.20),
                tax_item: TaxItem {
                    tax_percent: dec!(20),
                    tax_category: TaxCategory::S,
                },
                ..Default::default()
            },
        ],
    },
    ..Default::default()
}
.to_xml_string(); // returns "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice>...</Invoice>"
```

## Development

Reference: https://www.wko.at/service/netzwerke/ebinterface-aktuelle-version-xml-rechnungsstandard.html

Validate: https://labs.ebinterface.at/labs

```sh
cargo watch -x test
```
