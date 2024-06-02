use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::{
    biller::Biller,
    decimal::CloneAndRescale,
    details::{Details, DetailsItem},
    invoice_recipient::InvoiceRecipient,
    payment_method::PaymentMethod,
    tax::{TaxCategory, TaxItem},
    xml::{XmlElement, XmlToString},
};

#[derive(Default)]
pub struct Invoice<'a> {
    generating_system: &'a str,
    invoice_currency: &'a str,
    document_title: Option<&'a str>,
    language: Option<&'a str>,
    invoice_number: &'a str,
    invoice_date: &'a str,
    biller: Biller<'a>,
    invoice_recipient: InvoiceRecipient<'a>,
    details: Details<'a>,
    payment_method: Option<PaymentMethod<'a>>,
}

impl<'a> Invoice<'a> {
    pub fn new(
        generating_system: &'a str,
        invoice_currency: &'a str,
        invoice_number: &'a str,
        invoice_date: &'a str,
        biller: Biller<'a>,
        invoice_recipient: InvoiceRecipient<'a>,
    ) -> Self {
        Self {
            generating_system,
            invoice_currency,
            invoice_number,
            invoice_date,
            biller,
            invoice_recipient,
            ..Default::default()
        }
    }

    pub fn with_document_title(mut self, document_title: &'a str) -> Self {
        self.document_title = Some(document_title);
        self
    }

    pub fn with_language(mut self, language: &'a str) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_items(mut self, items: Vec<DetailsItem<'a>>) -> Self {
        self.details.items = items;
        self
    }

    pub fn with_item(mut self, item: DetailsItem<'a>) -> Self {
        self.details.items.push(item);
        self
    }

    pub fn with_payment_method(&mut self, payment_method: PaymentMethod<'a>) -> &Self {
        self.payment_method = Some(payment_method);

        self
    }

    fn invoice_tax_items(&self) -> Vec<((Decimal, TaxCategory), Decimal)> {
        // Collect all taxes, grouped by tuples of tax_percent and tax_category.
        let mut tax_items: HashMap<(Decimal, TaxCategory), Decimal> = HashMap::new();
        for i in &self.details.items {
            let k = i.tax_item_tuple();
            let s = tax_items.get(&k).unwrap_or(&Decimal::ZERO);
            tax_items.insert(k, s + i.line_item_amount());
        }

        // To get consistent results, sort by keys (tax_percent and tax_category).
        let mut sorted_tax_item_entries: Vec<((Decimal, TaxCategory), Decimal)> =
            tax_items.into_iter().collect();
        sorted_tax_item_entries.sort_by_key(|k| (k.0 .0, k.0 .1));

        sorted_tax_item_entries
    }

    pub fn to_xml_string(&self) -> Result<String, String> {
        let tax_item_xmls = self
            .invoice_tax_items()
            .iter()
            .map(|e| TaxItem::new(e.0 .0, e.0 .1).as_xml(&e.1))
            .collect::<Vec<XmlElement>>();

        let mut tax = XmlElement::new("Tax");
        for tax_item_xml in tax_item_xmls {
            tax = tax.with_element(tax_item_xml);
        }

        let total_gross_amount = self.details.items.iter().fold(Decimal::ZERO, |sum, i| sum + i.line_item_total_gross_amount()) /* + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
        let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

        let mut invoice = XmlElement::new("Invoice")
            .with_attr("xmlns", "http://www.ebinterface.at/schema/6p1/")
            .with_attr("GeneratingSystem", self.generating_system)
            .with_attr("DocumentType", "Invoice")
            .with_attr("InvoiceCurrency", self.invoice_currency);

        if let Some(document_title) = self.document_title {
            invoice = invoice.with_attr("DocumentTitle", document_title);
        }

        if let Some(language) = self.language {
            invoice = invoice.with_attr("Language", language);
        }

        invoice = invoice
            .with_text_element("InvoiceNumber", self.invoice_number)
            .with_text_element("InvoiceDate", self.invoice_date)
            .with_element(self.biller.as_xml())
            .with_element(self.invoice_recipient.as_xml())
            .with_element(self.details.as_xml());

        invoice = invoice
            .with_element(tax)
            .with_text_element(
                "TotalGrossAmount",
                total_gross_amount.clone_with_scale(2).to_string(),
            )
            .with_text_element(
                "PayableAmount",
                payable_amount.clone_with_scale(2).to_string(),
            );

        if let Some(payment_method) = &self.payment_method {
            match payment_method.as_xml() {
                Ok(pm) => {
                    invoice = invoice.with_element(pm);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            invoice.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn correctly_calculates_tax() {
        let invoice = Invoice::new(
            "test",
            "EUR",
            "0000",
            "2024-06-02",
            Biller::new("ATU00000000"),
            InvoiceRecipient::new("ATU000000000"),
        )
        .with_items(vec![
            DetailsItem::new(
                dec!(5.19),
                "h",
                dec!(106.23),
                TaxItem::new(dec!(20), TaxCategory::S),
            ),
            DetailsItem::new(
                dec!(3.2),
                "h",
                dec!(106.23),
                TaxItem::new(dec!(20), TaxCategory::S),
            ),
            DetailsItem::new(
                dec!(3.00),
                "h",
                dec!(106.23),
                TaxItem::new(dec!(20), TaxCategory::S),
            ),
            DetailsItem::new(
                dec!(0.84),
                "h",
                dec!(106.23),
                TaxItem::new(dec!(20), TaxCategory::S),
            ),
            DetailsItem::new(
                dec!(14.62),
                "h",
                dec!(106.23),
                TaxItem::new(dec!(20), TaxCategory::S),
            ),
        ]);

        let tax_items = invoice.invoice_tax_items();

        assert_eq!(tax_items.first().map_or(dec!(0), |i| i.1), dec!(2852.27))
    }
}
