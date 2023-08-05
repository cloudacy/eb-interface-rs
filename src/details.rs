use crate::{
    reduction_and_surcharge::ReductionAndSurchargeListLineItemDetails, tax::TaxItem,
    xml::XmlElement,
};
use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

pub struct DetailsItem<'a> {
    pub position_number: Option<u64>,
    pub description: Vec<&'a str>,
    pub quantity: Decimal,
    pub unit: &'a str,
    pub unit_price: Decimal,
    pub base_quantity: Option<Decimal>,
    pub reduction_and_surcharge: Option<ReductionAndSurchargeListLineItemDetails<'a>>,
    pub tax_item: TaxItem,
}

impl DetailsItem<'_> {
    pub fn line_item_amount(&self) -> Decimal {
        let base_quantity = match self.base_quantity {
            Some(bq) => bq,
            None => Decimal::ONE,
        };

        let reduction_and_surcharge_sum = match &self.reduction_and_surcharge {
            Some(rs) => rs.sum(),
            None => Decimal::ZERO,
        };

        self.quantity * self.unit_price / base_quantity + reduction_and_surcharge_sum
        /* + sum of other_vat_able_tax_list_line_item.tax_amount */
    }

    pub fn line_item_total_gross_amount(&self) -> Decimal {
        self.line_item_amount()
            * ((self.tax_item.tax_percent + Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED)
    }

    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("ListLineItem");

        if let Some(pn) = self.position_number {
            e = e.with_text_element("PositionNumber", &format!("{pn}"));
        }

        for description in &self.description {
            e = e.with_text_element("Description", description);
        }

        e = e.with_element(
            XmlElement::new("Quantity")
                .with_attr("Unit", self.unit)
                .with_text(&format!(
                    "{:.4}",
                    self.quantity
                        .round_dp_with_strategy(4, MidpointAwayFromZero)
                )),
        );

        let mut up = XmlElement::new("UnitPrice").with_text(&format!(
            "{:.4}",
            self.unit_price
                .round_dp_with_strategy(4, MidpointAwayFromZero)
        ));
        if let Some(bq) = &self.base_quantity {
            up = up.with_attr("BaseQuantity", &format!("{bq}"))
        }
        e = e.with_element(up);

        if let Some(reduction_and_surcharge) = &self.reduction_and_surcharge {
            e = e.with_element(reduction_and_surcharge.as_xml());
        }

        e = e.with_element(self.tax_item.as_xml());

        e = e.with_text_element(
            "LineItemAmount",
            &format!(
                "{:.2}",
                self.line_item_amount()
                    .round_dp_with_strategy(2, MidpointAwayFromZero)
            ),
        );

        e
    }
}

pub struct Details<'a> {
    pub items: Vec<DetailsItem<'a>>,
}

impl Details<'_> {
    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("Details");

        let mut ie = XmlElement::new("ItemList");
        for item in &self.items {
            ie = ie.with_element(item.as_xml());
        }
        e = e.with_element(ie);

        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use crate::{
        reduction_and_surcharge::{
            ReductionAndSurchargeValue, ReductionListLineItem, SurchargeListLineItem,
        },
        tax::TaxCategory,
        xml::XmlAsString,
    };

    #[test]
    fn rounds_line_item_amount_result_after_calculation() {
        let quantity = dec!(0.005);
        let unit_price = dec!(0.005);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            reduction_and_surcharge: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">0.0050</Quantity><UnitPrice>0.0050</UnitPrice><TaxItem><TaxableAmount>0.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>0.00</TaxAmount></TaxItem><LineItemAmount>0.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_up() {
        let quantity = dec!(100.123456);
        let unit_price = dec!(10.20005);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            reduction_and_surcharge: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1235</Quantity><UnitPrice>10.2001</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn calculates_reduction_correctly() {
        let result = DetailsItem {
            position_number: None,
            description: vec!["Handbuch zur Schraube"],
            quantity: dec!(1),
            unit: "STK",
            unit_price: dec!(5.00),
            base_quantity: None,
            reduction_and_surcharge: Some(ReductionAndSurchargeListLineItemDetails {
                reduction_list_line_items: Some(vec![ReductionListLineItem::new(
                    dec!(5),
                    ReductionAndSurchargeValue::Amount(dec!(2)),
                    None,
                )]),
                surcharge_list_line_items: None,
            }),
            tax_item: TaxItem {
                taxable_amount: dec!(3.00),
                tax_percent: dec!(10),
                tax_category: TaxCategory::AA,
            },
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice>5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.00</Amount></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>3.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.30</TaxAmount></TaxItem><LineItemAmount>3.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn calculates_surcharge_correctly() {
        let result = DetailsItem {
            position_number: None,
            description: vec!["Handbuch zur Schraube"],
            quantity: dec!(1),
            unit: "STK",
            unit_price: dec!(5.00),
            base_quantity: None,
            reduction_and_surcharge: Some(ReductionAndSurchargeListLineItemDetails {
                reduction_list_line_items: None,
                surcharge_list_line_items: Some(vec![SurchargeListLineItem::new(
                    dec!(5),
                    ReductionAndSurchargeValue::Amount(dec!(2)),
                    None,
                )]),
            }),
            tax_item: TaxItem {
                taxable_amount: dec!(7.00),
                tax_percent: dec!(10),
                tax_category: TaxCategory::AA,
            },
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice>5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><SurchargeListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.00</Amount></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>7.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.70</TaxAmount></TaxItem><LineItemAmount>7.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_down() {
        let quantity = dec!(100.12344);
        let unit_price = dec!(10.20001);
        let taxable_amount = quantity * unit_price;

        let result = DetailsItem {
            position_number: None,
            description: vec!["Sand"],
            quantity: quantity,
            unit: "KGM",
            unit_price: unit_price,
            base_quantity: None,
            reduction_and_surcharge: None,
            tax_item: TaxItem {
                taxable_amount: taxable_amount,
                tax_percent: dec!(20),
                tax_category: TaxCategory::S,
            },
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1234</Quantity><UnitPrice>10.2000</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }
}
