use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};

use crate::xml::XmlElement;

pub enum ReductionAndSurchargeValue {
    Percentage(Decimal),
    Amount(Decimal),
    PercentageAndAmount(Decimal, Decimal),
}

struct ReductionAndSurchargeListLineItemBase<'a> {
    base_amount: Decimal,
    value: ReductionAndSurchargeValue,
    comment: Option<&'a str>,
}

impl ReductionAndSurchargeListLineItemBase<'_> {
    fn sum(&self) -> Decimal {
        match self.value {
            ReductionAndSurchargeValue::Percentage(percentage) => {
                self.base_amount * percentage / Decimal::ONE_HUNDRED
            }
            ReductionAndSurchargeValue::Amount(amount) => amount,
            ReductionAndSurchargeValue::PercentageAndAmount(_, amount) => amount,
        }
    }

    fn as_xml(&self) -> Vec<XmlElement> {
        let mut es = vec![XmlElement::new("BaseAmount").with_text(&format!(
            "{:.2}",
            self.base_amount
                .round_dp_with_strategy(2, MidpointAwayFromZero)
        ))];

        match self.value {
            ReductionAndSurchargeValue::Percentage(percentage) => {
                es.push(XmlElement::new("Percentage").with_text(&format!(
                    "{:.2}",
                    percentage.round_dp_with_strategy(2, MidpointAwayFromZero)
                )));
            }
            ReductionAndSurchargeValue::Amount(amount) => {
                es.push(XmlElement::new("Amount").with_text(&format!(
                    "{:.2}",
                    amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                )));
            }
            ReductionAndSurchargeValue::PercentageAndAmount(percentage, amount) => {
                es.push(XmlElement::new("Percentage").with_text(&format!(
                    "{:.2}",
                    percentage.round_dp_with_strategy(2, MidpointAwayFromZero)
                )));
                es.push(XmlElement::new("Amount").with_text(&format!(
                    "{:.2}",
                    amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                )));
            }
        }

        if let Some(comment) = self.comment {
            es.push(XmlElement::new("Comment").with_text(comment))
        }

        es
    }
}

pub struct ReductionListLineItem<'a> {
    base: ReductionAndSurchargeListLineItemBase<'a>,
}

impl ReductionListLineItem<'_> {
    pub fn new(
        base_amount: Decimal,
        value: ReductionAndSurchargeValue,
        comment: Option<&'static str>,
    ) -> Self {
        ReductionListLineItem {
            base: ReductionAndSurchargeListLineItemBase {
                base_amount,
                value,
                comment,
            },
        }
    }

    fn sum(&self) -> Decimal {
        self.base.sum()
    }

    fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("ReductionListLineItem");

        for base_element in self.base.as_xml() {
            e = e.with_element(base_element);
        }

        e
    }
}

pub struct SurchargeListLineItem<'a> {
    base: ReductionAndSurchargeListLineItemBase<'a>,
}

impl SurchargeListLineItem<'_> {
    pub fn new(
        base_amount: Decimal,
        value: ReductionAndSurchargeValue,
        comment: Option<&'static str>,
    ) -> Self {
        SurchargeListLineItem {
            base: ReductionAndSurchargeListLineItemBase {
                base_amount,
                value,
                comment,
            },
        }
    }

    fn sum(&self) -> Decimal {
        self.base.sum()
    }

    fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("SurchargeListLineItem");

        for base_element in self.base.as_xml() {
            e = e.with_element(base_element);
        }

        e
    }
}

#[derive(Default)]
pub struct ReductionAndSurchargeListLineItemDetails<'a> {
    pub reduction_list_line_items: Option<Vec<ReductionListLineItem<'a>>>,
    pub surcharge_list_line_items: Option<Vec<SurchargeListLineItem<'a>>>,
}

impl ReductionAndSurchargeListLineItemDetails<'_> {
    pub fn sum(&self) -> Decimal {
        let surcharge_sum = match &self.surcharge_list_line_items {
            Some(s) => s.iter().fold(Decimal::ZERO, |sum, s| sum + s.sum()),
            None => Decimal::ZERO,
        };
        let reduction_sum = match &self.reduction_list_line_items {
            Some(r) => r.iter().fold(Decimal::ZERO, |sum, r| sum + r.sum()),
            None => Decimal::ZERO,
        };
        surcharge_sum - reduction_sum
    }

    pub fn as_xml(&self) -> XmlElement {
        let mut e = XmlElement::new("ReductionAndSurchargeListLineItemDetails");

        if let Some(reduction_list_line_items) = &self.reduction_list_line_items {
            for reduction_list_line_item in reduction_list_line_items {
                e = e.with_element(reduction_list_line_item.as_xml());
            }
        }

        if let Some(surcharge_list_line_items) = &self.surcharge_list_line_items {
            for surcharge_list_line_item in surcharge_list_line_items {
                e = e.with_element(surcharge_list_line_item.as_xml());
            }
        }

        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use crate::xml::XmlAsString;

    #[test]
    fn generates_reduction_and_surcharge_list_line_item() {
        let result = ReductionAndSurchargeListLineItemDetails {
            reduction_list_line_items: Some(vec![ReductionListLineItem::new(
                dec!(100),
                ReductionAndSurchargeValue::Percentage(dec!(2)),
                Some("reduction"),
            )]),
            surcharge_list_line_items: Some(vec![SurchargeListLineItem::new(
                dec!(200),
                ReductionAndSurchargeValue::Amount(dec!(3)),
                Some("surcharge"),
            )]),
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Percentage>2.00</Percentage><Comment>reduction</Comment></ReductionListLineItem><SurchargeListLineItem><BaseAmount>200.00</BaseAmount><Amount>3.00</Amount><Comment>surcharge</Comment></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );

        let result = ReductionAndSurchargeListLineItemDetails {
            reduction_list_line_items: Some(vec![ReductionListLineItem::new(
                dec!(100),
                ReductionAndSurchargeValue::Amount(dec!(2)),
                Some("reduction"),
            )]),
            surcharge_list_line_items: Some(vec![SurchargeListLineItem::new(
                dec!(200),
                ReductionAndSurchargeValue::Percentage(dec!(3)),
                Some("surcharge"),
            )]),
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Amount>2.00</Amount><Comment>reduction</Comment></ReductionListLineItem><SurchargeListLineItem><BaseAmount>200.00</BaseAmount><Percentage>3.00</Percentage><Comment>surcharge</Comment></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );

        let result = ReductionAndSurchargeListLineItemDetails {
            reduction_list_line_items: Some(vec![ReductionListLineItem::new(
                dec!(100),
                ReductionAndSurchargeValue::PercentageAndAmount(dec!(2), dec!(3)),
                Some("reduction"),
            )]),
            ..Default::default()
        }
        .as_xml();

        assert_eq!(
            result.as_str(),
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Percentage>2.00</Percentage><Amount>3.00</Amount><Comment>reduction</Comment></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );
    }
}
