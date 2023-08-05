# eb-interface-rs

![status](https://github.com/cloudacy/eb-interface-rs/actions/workflows/rust.yml/badge.svg)

[ebInterface](https://www.wko.at/service/netzwerke/was-ist-ebinterface.html) generator

:warning: **This crate is in alpha stage** :warning:

## Notes

- rounding
  - `eb-interface-rs` rounds after all calculations are done
  - `eb-interface-rs` uses the [`MidpointAwayFromZero`](https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointAwayFromZero) rounding strategy

## Feature set

- [x] minimal invoice

## Example

```rust
Invoice::new(
    "generating system",
    "EUR", // currency
    "document title",
    "de", // language
    "993433000298", // invoice number
    "2020-01-01", // invoice date
    Biller {
        vat_identification_number: "ATU51507409",
        further_identification: None,
        order_reference: None,
        address: Some(Address {
            name: "Schrauben Mustermann OG",
            street: None,
            town: "Wien",
            zip: "1020",
            country: "Österreich",
            country_code: Some("AT"),
            phone: None,
            email: None,
        }),
        contact: None,
    },
    InvoiceRecipient {
        vat_identification_number: "ATU18708634",
        further_identification: None,
        order_reference: None,
        address: Some(Address {
            name: "Mustermann GmbH",
            street: None,
            town: "Graz",
            zip: "8010",
            country: "Österreich",
            country_code: Some("AT"),
            phone: None,
            email: None,
        }),
        contact: None,
    },
    Details {
        items: vec![
            DetailsItem {
                position_number: None,
                description: vec!["Schraubenzieher"],
                quantity: dec!(100),
                unit: "STK",
                unit_price: dec!(10.20),
                base_quantity: None,
                reduction_and_surcharge: None,
                tax_item: TaxItem {
                    tax_percent: dec!(20),
                    tax_category: TaxCategory::S,
                },
            },
        ],
    },
)
.as_xml_str(); // returns "<Invoice>...</Invoice>"
```

## Development

Reference: https://www.wko.at/service/netzwerke/ebinterface-aktuelle-version-xml-rechnungsstandard.html

Validate: https://labs.ebinterface.at/labs

```sh
cargo watch -x test
```
