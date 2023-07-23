use crate::{tax::TaxItem, xml::XmlElement};
use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

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
