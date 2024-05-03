use rust_decimal::Decimal;

use crate::{decimal::CloneAndRescale, xml::XmlElement};

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
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
    tax_percent: Decimal,
    tax_category: TaxCategory,
}

impl TaxItem {
    pub fn new(tax_percent: Decimal, tax_category: TaxCategory) -> TaxItem {
        TaxItem {
            tax_percent,
            tax_category,
        }
    }

    pub fn percent(&self) -> Decimal {
        self.tax_percent
    }

    pub fn category(&self) -> TaxCategory {
        self.tax_category
    }

    pub fn tax_item_tuple(&self) -> (Decimal, TaxCategory) {
        (self.tax_percent, self.tax_category)
    }

    pub fn as_xml<'a>(&self, taxable_amount: &Decimal) -> XmlElement<'a> {
        let tax_amount = taxable_amount * (self.tax_percent / Decimal::ONE_HUNDRED);

        XmlElement::new("TaxItem")
            .with_text_element(
                "TaxableAmount",
                taxable_amount.clone_with_scale(2).to_string(),
            )
            .with_element(
                XmlElement::new("TaxPercent")
                    .with_attr("TaxCategoryCode", self.tax_category.as_str().to_string())
                    .with_text(self.tax_percent.to_string()),
            )
            .with_text_element("TaxAmount", tax_amount.clone_with_scale(2).to_string())
    }
}
