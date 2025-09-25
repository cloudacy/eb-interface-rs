use rust_decimal::Decimal;

use crate::{
    decimal::CloneAndRescale,
    xml::{ToXml, XmlElement},
};

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

impl<'a> ReductionAndSurchargeListLineItemBase<'a> {
    fn new(base_amount: Decimal, value: ReductionAndSurchargeValue) -> Self {
        ReductionAndSurchargeListLineItemBase {
            base_amount,
            value,
            comment: None,
        }
    }

    fn sum(&self) -> Decimal {
        match self.value {
            ReductionAndSurchargeValue::Percentage(percentage) => {
                (self.base_amount * percentage / Decimal::ONE_HUNDRED).clone_with_scale(2)
            }
            ReductionAndSurchargeValue::Amount(amount) => amount.clone_with_scale(2),
            ReductionAndSurchargeValue::PercentageAndAmount(_, amount) => {
                amount.clone_with_scale(2)
            }
        }
    }

    fn to_xml_elements(&self) -> Vec<XmlElement> {
        let mut es = vec![
            XmlElement::new("BaseAmount")
                .with_text(self.base_amount.clone_with_scale(2).to_string()),
        ];

        match self.value {
            ReductionAndSurchargeValue::Percentage(percentage) => {
                es.push(
                    XmlElement::new("Percentage")
                        .with_text(percentage.clone_with_scale(2).to_string()),
                );
            }
            ReductionAndSurchargeValue::Amount(amount) => {
                es.push(
                    XmlElement::new("Amount").with_text(amount.clone_with_scale(2).to_string()),
                );
            }
            ReductionAndSurchargeValue::PercentageAndAmount(percentage, amount) => {
                es.push(
                    XmlElement::new("Percentage")
                        .with_text(percentage.clone_with_scale(2).to_string()),
                );
                es.push(
                    XmlElement::new("Amount").with_text(amount.clone_with_scale(2).to_string()),
                );
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

impl<'a> ReductionListLineItem<'a> {
    pub fn new(base_amount: Decimal, value: ReductionAndSurchargeValue) -> Self {
        ReductionListLineItem {
            base: ReductionAndSurchargeListLineItemBase::new(base_amount, value),
        }
    }

    pub fn with_comment(mut self, comment: &'a str) -> Self {
        self.base.comment = Some(comment);
        self
    }

    fn sum(&self) -> Decimal {
        self.base.sum()
    }
}

impl ToXml for ReductionListLineItem<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("ReductionListLineItem");

        for base_element in self.base.to_xml_elements() {
            e = e.with_element(&base_element);
        }

        e.to_xml()
    }
}

pub struct SurchargeListLineItem<'a> {
    base: ReductionAndSurchargeListLineItemBase<'a>,
}

impl<'a> SurchargeListLineItem<'a> {
    pub fn new(base_amount: Decimal, value: ReductionAndSurchargeValue) -> Self {
        SurchargeListLineItem {
            base: ReductionAndSurchargeListLineItemBase::new(base_amount, value),
        }
    }

    pub fn with_comment(mut self, comment: &'a str) -> Self {
        self.base.comment = Some(comment);
        self
    }

    fn sum(&self) -> Decimal {
        self.base.sum()
    }
}

impl ToXml for SurchargeListLineItem<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("SurchargeListLineItem");

        for base_element in self.base.to_xml_elements() {
            e = e.with_element(&base_element);
        }

        e.to_xml()
    }
}

#[derive(Default)]
pub struct ReductionAndSurchargeListLineItemDetails<'a> {
    reduction_list_line_items: Option<Vec<ReductionListLineItem<'a>>>,
    surcharge_list_line_items: Option<Vec<SurchargeListLineItem<'a>>>,
}

impl<'a> ReductionAndSurchargeListLineItemDetails<'a> {
    pub fn new() -> Self {
        ReductionAndSurchargeListLineItemDetails {
            ..Default::default()
        }
    }

    pub fn with_reduction(mut self, reduction: ReductionListLineItem<'a>) -> Self {
        self.reduction_list_line_items
            .get_or_insert_with(Vec::new)
            .push(reduction);
        self
    }

    pub fn with_surcharge(mut self, surcharge: SurchargeListLineItem<'a>) -> Self {
        self.surcharge_list_line_items
            .get_or_insert_with(Vec::new)
            .push(surcharge);
        self
    }

    pub(crate) fn sum(&self) -> Decimal {
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
}

impl ToXml for ReductionAndSurchargeListLineItemDetails<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("ReductionAndSurchargeListLineItemDetails");

        if let Some(reduction_list_line_items) = &self.reduction_list_line_items {
            for reduction_list_line_item in reduction_list_line_items {
                e = e.with_element(reduction_list_line_item);
            }
        }

        if let Some(surcharge_list_line_items) = &self.surcharge_list_line_items {
            for surcharge_list_line_item in surcharge_list_line_items {
                e = e.with_element(surcharge_list_line_item);
            }
        }

        e.to_xml()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::xml::ToXml;

    #[test]
    fn generates_reduction_and_surcharge_list_line_item() {
        let result = ReductionAndSurchargeListLineItemDetails::new()
            .with_reduction(
                ReductionListLineItem::new(
                    Decimal::from(100),
                    ReductionAndSurchargeValue::Percentage(Decimal::from(2)),
                )
                .with_comment("reduction"),
            )
            .with_surcharge(
                SurchargeListLineItem::new(
                    Decimal::from(200),
                    ReductionAndSurchargeValue::Amount(Decimal::from(3)),
                )
                .with_comment("surcharge"),
            )
            .to_xml();

        assert_eq!(
            result,
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Percentage>2.00</Percentage><Comment>reduction</Comment></ReductionListLineItem><SurchargeListLineItem><BaseAmount>200.00</BaseAmount><Amount>3.00</Amount><Comment>surcharge</Comment></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );

        let result = ReductionAndSurchargeListLineItemDetails::new()
            .with_reduction(
                ReductionListLineItem::new(
                    Decimal::from(100),
                    ReductionAndSurchargeValue::Amount(Decimal::from(2)),
                )
                .with_comment("reduction"),
            )
            .with_surcharge(
                SurchargeListLineItem::new(
                    Decimal::from(200),
                    ReductionAndSurchargeValue::Percentage(Decimal::from(3)),
                )
                .with_comment("surcharge"),
            )
            .to_xml();

        assert_eq!(
            result,
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Amount>2.00</Amount><Comment>reduction</Comment></ReductionListLineItem><SurchargeListLineItem><BaseAmount>200.00</BaseAmount><Percentage>3.00</Percentage><Comment>surcharge</Comment></SurchargeListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );

        let result = ReductionAndSurchargeListLineItemDetails::new()
            .with_reduction(
                ReductionListLineItem::new(
                    Decimal::from(100),
                    ReductionAndSurchargeValue::PercentageAndAmount(
                        Decimal::from(2),
                        Decimal::new(34599, 4),
                    ),
                )
                .with_comment("reduction"),
            )
            .to_xml();

        assert_eq!(
            result,
            "<ReductionAndSurchargeListLineItemDetails><ReductionListLineItem><BaseAmount>100.00</BaseAmount><Percentage>2.00</Percentage><Amount>3.46</Amount><Comment>reduction</Comment></ReductionListLineItem></ReductionAndSurchargeListLineItemDetails>"
        );
    }
}
