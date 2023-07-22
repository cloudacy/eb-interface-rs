use rust_decimal::{Decimal, RoundingStrategy::MidpointAwayFromZero};
use std::collections::HashMap;

use crate::{
    biller::Biller,
    details::Details,
    invoice_recipient::InvoiceRecipient,
    tax::{TaxCategory, TaxItem},
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
            generating_system: generating_system,
            invoice_currency: invoice_currency,
            document_title: document_title,
            language: language,
            invoice_number: invoice_number,
            invoice_date: invoice_date,
            biller: biller,
            invoice_recipient: invoice_recipient,
            details: details,
        }
    }

    pub fn as_xml(&self) -> String {
        let biller_xml = self.biller.as_xml();
        let invoice_recipient_xml = self.invoice_recipient.as_xml();
        let details_xml = self.details.as_xml();

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

        let tax_items_xml_vec: Vec<String> = sorted_tax_item_entries
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
        let tax_items_xml = tax_items_xml_vec.join("");

        let total_gross_amount = (&self.details.items).into_iter().fold(Decimal::ZERO, |sum, i| sum + i.line_item_total_gross_amount()) /* + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
        let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

        format!(
          "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Invoice xmlns=\"http://www.ebinterface.at/schema/6p1/\" GeneratingSystem=\"{}\" DocumentType=\"Invoice\" InvoiceCurrency=\"{}\" DocumentTitle=\"{}\" Language=\"{}\"><InvoiceNumber>{}</InvoiceNumber><InvoiceDate>{}</InvoiceDate>{biller_xml}{invoice_recipient_xml}{details_xml}<Tax>{tax_items_xml}</Tax><TotalGrossAmount>{:.2}</TotalGrossAmount><PayableAmount>{:.2}</PayableAmount></Invoice>", self.generating_system, self.invoice_currency, self.document_title, self.language, self.invoice_number, self.invoice_date, total_gross_amount.round_dp_with_strategy(2, MidpointAwayFromZero), payable_amount.round_dp_with_strategy(2, MidpointAwayFromZero)
      )
    }
}
