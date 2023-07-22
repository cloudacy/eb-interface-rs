use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

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

pub struct TaxItem {
    pub taxable_amount: Decimal,
    pub tax_percent: Decimal,
    pub tax_category: TaxCategory,
}

impl TaxItem {
    pub fn as_xml(&self) -> String {
        let taxable_amount = self.taxable_amount;
        let tax_category_code = self.tax_category.as_str();
        let tax_percent = self.tax_percent;
        let tax_amount = self.taxable_amount * (self.tax_percent / Decimal::ONE_HUNDRED);
        format!("<TaxItem><TaxableAmount>{:.2}</TaxableAmount><TaxPercent TaxCategoryCode=\"{tax_category_code}\">{tax_percent}</TaxPercent><TaxAmount>{:.2}</TaxAmount></TaxItem>", taxable_amount.round_dp_with_strategy(2, MidpointAwayFromZero), tax_amount.round_dp_with_strategy(2, MidpointAwayFromZero))
    }
}
