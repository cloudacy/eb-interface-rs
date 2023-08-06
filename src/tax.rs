use crate::xml::XmlElement;
use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Default)]
pub enum TaxCategory {
    #[default]
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
    pub fn as_str(&self) -> &str {
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

#[derive(Default)]
pub struct TaxItem {
    pub tax_percent: Decimal,
    pub tax_category: TaxCategory,
}

impl TaxItem {
    pub fn as_xml(&self, taxable_amount: &Decimal) -> XmlElement {
        let tax_amount = taxable_amount * (self.tax_percent / Decimal::ONE_HUNDRED);

        XmlElement::new("TaxItem")
            .with_text_element(
                "TaxableAmount",
                &format!(
                    "{:.2}",
                    taxable_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                ),
            )
            .with_element(
                XmlElement::new("TaxPercent")
                    .with_attr("TaxCategoryCode", self.tax_category.as_str())
                    .with_text(&format!("{}", self.tax_percent)),
            )
            .with_text_element(
                "TaxAmount",
                &format!(
                    "{:.2}",
                    tax_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                ),
            )
    }
}
