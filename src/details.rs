use rust_decimal::{Decimal, RoundingStrategy};

use crate::{
    decimal::CloneAndRescale,
    reduction_and_surcharge::{
        ReductionAndSurchargeListLineItemDetails, ReductionListLineItem, SurchargeListLineItem,
    },
    tax::TaxItem,
    xml::{ToXml, XmlElement},
};

#[derive(Default)]
pub struct DetailsItem<'a> {
    position_number: Option<u64>,
    description: Vec<&'a str>,
    quantity: Decimal,
    unit: &'a str,
    unit_price: Decimal,
    base_quantity: Option<Decimal>,
    reduction_and_surcharge: Option<ReductionAndSurchargeListLineItemDetails<'a>>,
    pub(crate) tax_item: TaxItem,
}

impl<'a> DetailsItem<'a> {
    pub fn new(
        quantity: Decimal,
        unit: &'a str,
        unit_price: Decimal,
        tax_item: TaxItem,
    ) -> DetailsItem {
        DetailsItem {
            quantity,
            unit,
            unit_price,
            tax_item,
            ..Default::default()
        }
    }

    pub fn with_position_number(mut self, position_number: u64) -> Self {
        self.position_number = Some(position_number);
        self
    }

    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description.push(description);
        self
    }

    pub fn with_base_quantity(mut self, base_quantity: Decimal) -> Self {
        self.base_quantity = Some(base_quantity);
        self
    }

    pub fn with_reduction(mut self, reduction: ReductionListLineItem<'a>) -> Self {
        let mut ras = match self.reduction_and_surcharge {
            Some(ras) => ras,
            None => ReductionAndSurchargeListLineItemDetails::new(),
        };
        ras = ras.with_reduction(reduction);
        self.reduction_and_surcharge = Some(ras);
        self
    }

    pub fn with_surcharge(mut self, surcharge: SurchargeListLineItem<'a>) -> Self {
        let mut ras = match self.reduction_and_surcharge {
            Some(ras) => ras,
            None => ReductionAndSurchargeListLineItemDetails::new(),
        };
        ras = ras.with_surcharge(surcharge);
        self.reduction_and_surcharge = Some(ras);
        self
    }

    pub(crate) fn line_item_amount(&self) -> Decimal {
        let base_quantity = self.base_quantity.unwrap_or(Decimal::ONE);

        let reduction_and_surcharge_sum = match &self.reduction_and_surcharge {
            Some(rs) => rs.sum(),
            None => Decimal::ZERO,
        };

        (self.quantity * self.unit_price / base_quantity + reduction_and_surcharge_sum)
            .round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
        /* + sum of other_vat_able_tax_list_line_item.tax_amount */
    }

    pub(crate) fn line_item_total_gross_amount(&self) -> Decimal {
        self.line_item_amount()
            * ((self.tax_item.tax_percent + Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED)
    }
}

impl ToXml for DetailsItem<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("ListLineItem");

        // PositionNumber.
        if let Some(pn) = self.position_number {
            e = e.with_text_element("PositionNumber", pn.to_string());
        }

        // Description(s).
        for description in &self.description {
            e = e.with_text_element("Description", description);
        }

        // Quantity.
        e = e.with_element(
            &XmlElement::new("Quantity")
                .with_attr("Unit", self.unit)
                .with_text(self.quantity.clone_with_scale(4).to_string()),
        );

        // UnitPrice and BaseQuantity.
        let mut up =
            XmlElement::new("UnitPrice").with_text(self.unit_price.clone_with_scale(4).to_string());
        if let Some(bq) = &self.base_quantity {
            up = up.with_attr("BaseQuantity", bq.to_string())
        }
        e = e.with_element(&up);

        // ReductionListLineItem(s) and SurchargeListLineItem(s).
        let mut reduction_and_surcharge_sum = Decimal::ZERO;
        if let Some(reduction_and_surcharge) = &self.reduction_and_surcharge {
            reduction_and_surcharge_sum = reduction_and_surcharge.sum();
            e = e.with_element(reduction_and_surcharge);
        }

        // TaxItem.
        let taxable_amount = self.quantity * self.unit_price + reduction_and_surcharge_sum;
        e = e.with_element(&self.tax_item.taxable_amount(taxable_amount));

        // LineItemAmount.
        e = e.with_text_element("LineItemAmount", self.line_item_amount().to_string());

        e.to_xml()
    }
}

#[derive(Default)]
pub(crate) struct Details<'a> {
    pub(crate) items: Vec<DetailsItem<'a>>,
}

impl ToXml for Details<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("Details");

        let mut ie = XmlElement::new("ItemList");
        for item in &self.items {
            ie = ie.with_element(item);
        }
        e = e.with_element(&ie);

        e.to_xml()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use crate::{
        reduction_and_surcharge::{ReductionAndSurchargeValue, SurchargeListLineItem},
        tax::TaxCategory,
        xml::ToXml,
    };

    #[test]
    fn rounds_line_item_amount_result_after_calculation() {
        let quantity = dec!(0.005);
        let unit_price = dec!(0.005);

        let result = DetailsItem::new(
            quantity,
            "KGM",
            unit_price,
            TaxItem::new(dec!(20), TaxCategory::S),
        )
        .with_description("Sand")
        .to_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">0.0050</Quantity><UnitPrice>0.0050</UnitPrice><TaxItem><TaxableAmount>0.00</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>0.00</TaxAmount></TaxItem><LineItemAmount>0.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_up() {
        let quantity = dec!(100.123456);
        let unit_price = dec!(10.20005);

        let result = DetailsItem::new(
            quantity,
            "KGM",
            unit_price,
            TaxItem::new(dec!(20), TaxCategory::S),
        )
        .with_description("Sand")
        .to_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1235</Quantity><UnitPrice>10.2001</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn calculates_reduction_correctly() {
        let result = DetailsItem::new(
            dec!(1),
            "STK",
            dec!(5.00),
            TaxItem::new(dec!(10), TaxCategory::AA),
        )
        .with_description("Handbuch zur Schraube")
        .with_reduction(ReductionListLineItem::new(
            dec!(5),
            ReductionAndSurchargeValue::Amount(dec!(2.3399)),
        ))
        .to_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice>5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.34</Amount></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>2.66</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.27</TaxAmount></TaxItem><LineItemAmount>2.66</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn calculates_surcharge_correctly() {
        let result = DetailsItem::new(
            dec!(1),
            "STK",
            dec!(5.00),
            TaxItem::new(dec!(10), TaxCategory::AA),
        )
        .with_description("Handbuch zur Schraube")
        .with_surcharge(SurchargeListLineItem::new(
            dec!(5),
            ReductionAndSurchargeValue::Amount(dec!(2)),
        ))
        .to_xml();

        assert_eq!(
            result,
            "<ListLineItem><Description>Handbuch zur Schraube</Description><Quantity Unit=\"STK\">1.0000</Quantity><UnitPrice>5.0000</UnitPrice><ReductionAndSurchargeListLineItemDetails><SurchargeListLineItem><BaseAmount>5.00</BaseAmount><Amount>2.00</Amount></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails><TaxItem><TaxableAmount>7.00</TaxableAmount><TaxPercent TaxCategoryCode=\"AA\">10</TaxPercent><TaxAmount>0.70</TaxAmount></TaxItem><LineItemAmount>7.00</LineItemAmount></ListLineItem>"
        );
    }

    #[test]
    fn rounds_correctly_down() {
        let quantity = dec!(100.12344);
        let unit_price = dec!(10.20001);

        let result = DetailsItem::new(
            quantity,
            "KGM",
            unit_price,
            TaxItem::new(dec!(20), TaxCategory::S),
        )
        .with_description("Sand")
        .to_xml();

        assert_eq!(
            result.as_str(),
            "<ListLineItem><Description>Sand</Description><Quantity Unit=\"KGM\">100.1234</Quantity><UnitPrice>10.2000</UnitPrice><TaxItem><TaxableAmount>1021.26</TaxableAmount><TaxPercent TaxCategoryCode=\"S\">20</TaxPercent><TaxAmount>204.25</TaxAmount></TaxItem><LineItemAmount>1021.26</LineItemAmount></ListLineItem>"
        );
    }
}
