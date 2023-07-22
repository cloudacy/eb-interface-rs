use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

use crate::tax::TaxItem;

pub struct DetailsItem<'a> {
    pub position_number: Option<u64>,
    pub description: Vec<&'a str>,
    pub quantity: Decimal,
    pub unit: &'a str,
    pub unit_price: Decimal,
    pub base_quantity: Option<Decimal>,
    pub tax_item: TaxItem,
}

impl DetailsItem<'_> {
    pub fn line_item_amount(&self) -> Decimal {
        let base_quantity = match self.base_quantity {
            Some(bq) => bq,
            None => Decimal::ONE,
        };
        self.quantity * self.unit_price / base_quantity /* + sum of surcharge_list_line_item.amount + sum of other_vat_able_tax_list_line_item.tax_amount - sum of reduction_list_line_item.amount */
    }

    pub fn line_item_total_gross_amount(&self) -> Decimal {
        self.line_item_amount()
            * ((self.tax_item.tax_percent + Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED)
    }

    pub fn as_xml(&self) -> String {
        let position_number = match self.position_number {
            Some(pn) => format!("<PositionNumber>{pn}</PositionNumber>"),
            None => format!(""),
        };
        let description_vec: Vec<String> = (&self.description)
            .into_iter()
            .map(|d| format!("<Description>{d}</Description>"))
            .collect();
        let description = description_vec.join("");
        let unit: &str = self.unit;
        let base_quantity = match self.base_quantity {
            Some(bq) => format!(" BaseQuantity=\"{bq}\""),
            None => format!(""),
        };
        let tax_item_xml = self.tax_item.as_xml();
        format!("<ListLineItem>{position_number}{description}<Quantity Unit=\"{unit}\">{:.4}</Quantity><UnitPrice{base_quantity}>{:.4}</UnitPrice>{tax_item_xml}<LineItemAmount>{:.2}</LineItemAmount></ListLineItem>", self.quantity.round_dp_with_strategy(4, MidpointAwayFromZero), self.unit_price.round_dp_with_strategy(4, MidpointAwayFromZero), self.line_item_amount().round_dp_with_strategy(2, MidpointAwayFromZero))
    }
}

pub struct Details<'a> {
    pub items: Vec<DetailsItem<'a>>,
}

impl Details<'_> {
    pub fn as_xml(&self) -> String {
        let items_xml_vec: Vec<String> = (&self.items).into_iter().map(|l| l.as_xml()).collect();
        let items_xml = items_xml_vec.join("");
        format!("<Details><ItemList>{items_xml}</ItemList></Details>")
    }
}
