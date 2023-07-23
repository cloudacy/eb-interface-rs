use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};
use std::collections::HashMap;

use crate::{
    biller::Biller,
    details::Details,
    invoice_recipient::InvoiceRecipient,
    tax::{TaxCategory, TaxItem},
    xml::{XmlAsString, XmlElement},
};

pub struct Invoice<'a> {
    generating_system: &'a str,
    invoice_currency: &'a str,
    document_title: &'a str,
    language: &'a str,
    invoice_number: &'a str,
    invoice_date: &'a str,
    biller: Biller<'a>,
    invoice_recipient: InvoiceRecipient<'a>,
    details: Details<'a>,
}

impl Invoice<'_> {
    pub fn new<'a>(
        generating_system: &'a str,
        invoice_currency: &'a str,
        document_title: &'a str,
        language: &'a str,
        invoice_number: &'a str,
        invoice_date: &'a str,
        biller: Biller<'a>,
        invoice_recipient: InvoiceRecipient<'a>,
        details: Details<'a>,
    ) -> Invoice<'a> {
        Invoice {
            generating_system,
            invoice_currency,
            document_title,
            language,
            invoice_number,
            invoice_date,
            biller,
            invoice_recipient,
            details,
        }
    }

    pub fn as_xml(&self) -> String {
        // Collect all taxes, grouped by tuples of tax_percent and tax_category.
        let mut tax_items: HashMap<(Decimal, TaxCategory), Decimal> = HashMap::new();
        for i in &self.details.items {
            let k = (i.tax_item.tax_percent, i.tax_item.tax_category.clone());
            let s = match tax_items.get(&k) {
                Some(v) => v.clone(),
                None => Decimal::ZERO,
            };
            tax_items.insert(k, s + i.line_item_amount());
        }

        // To get consistent results, sort by keys (tax_percent and tax_category).
        let mut sorted_tax_item_entries: Vec<((Decimal, TaxCategory), Decimal)> =
            tax_items.into_iter().collect();
        sorted_tax_item_entries.sort_by_key(|k| (k.0 .0, k.0 .1.clone()));

        let tax_item_xmls: Vec<XmlElement> = sorted_tax_item_entries
            .into_iter()
            .map(|e| {
                TaxItem {
                    taxable_amount: e.1,
                    tax_percent: e.0 .0,
                    tax_category: e.0 .1,
                }
                .as_xml()
            })
            .collect();

        let mut tax = XmlElement::new("Tax");
        for tax_item_xml in tax_item_xmls {
            tax = tax.with_element(tax_item_xml);
        }

        let total_gross_amount = (&self.details.items).into_iter().fold(Decimal::ZERO, |sum, i| sum + i.line_item_total_gross_amount()) /* + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
        let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

        let invoice = XmlElement::new("Invoice")
            .with_attr("xmlns", "http://www.ebinterface.at/schema/6p1/")
            .with_attr("GeneratingSystem", self.generating_system)
            .with_attr("DocumentType", "Invoice")
            .with_attr("InvoiceCurrency", self.invoice_currency)
            .with_attr("DocumentTitle", self.document_title)
            .with_attr("Language", self.language)
            .with_text_element("InvoiceNumber", self.invoice_number)
            .with_text_element("InvoiceDate", self.invoice_date)
            .with_element(self.biller.as_xml())
            .with_element(self.invoice_recipient.as_xml())
            .with_element(self.details.as_xml())
            .with_element(tax)
            .with_text_element(
                "TotalGrossAmount",
                &format!(
                    "{:.2}",
                    total_gross_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                ),
            )
            .with_text_element(
                "PayableAmount",
                &format!(
                    "{:.2}",
                    payable_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
                ),
            );

        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            invoice.as_str()
        )
    }
}
