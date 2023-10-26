use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::{
    biller::Biller,
    decimal::CloneAndRescale,
    details::Details,
    invoice_recipient::InvoiceRecipient,
    payment_method::{PaymentMethod, PaymentMethodType},
    tax::{TaxCategory, TaxItem},
    xml::{XmlElement, XmlToString},
};

#[derive(Default)]
pub struct Invoice<'a> {
    pub generating_system: &'a str,
    pub invoice_currency: &'a str,
    pub document_title: &'a str,
    pub language: &'a str,
    pub invoice_number: &'a str,
    pub invoice_date: &'a str,
    pub biller: Biller<'a>,
    pub invoice_recipient: InvoiceRecipient<'a>,
    pub details: Details<'a>,
    pub payment_method: Option<PaymentMethod<'a>>,
}

impl<'a> Invoice<'a> {
    pub fn with_payment_method(
        &mut self,
        payment_method: impl PaymentMethodType<'a> + 'a,
        comment: Option<&'a str>,
    ) -> &Self {
        self.payment_method = Some(PaymentMethod {
            payment_method_type: Box::new(payment_method),
            comment: comment,
            ..Default::default()
        });

        self
    }

    pub fn to_xml_string(&self) -> Result<String, String> {
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
                    tax_percent: e.0 .0,
                    tax_category: e.0 .1,
                }
                .as_xml(&e.1)
            })
            .collect();

        let mut tax = XmlElement::new("Tax");
        for tax_item_xml in tax_item_xmls {
            tax = tax.with_element(tax_item_xml);
        }

        let total_gross_amount = (&self.details.items).into_iter().fold(Decimal::ZERO, |sum, i| sum + i.line_item_total_gross_amount()) /* + sum of surcharges at root + sum of other_vat_able_taxes at root - sum of reductions at root */;
        let payable_amount = total_gross_amount /* - prepaid_amount + rounding_amount + sum of below_the_lines_items */;

        let mut invoice = XmlElement::new("Invoice")
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
            .with_element(self.details.as_xml());

        if let Some(payment_method) = &self.payment_method {
            match payment_method.as_xml() {
                Ok(pm) => {
                    invoice = invoice.with_element(pm);
                }
                Err(e) => return Err(e),
            }
        }

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

        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            invoice.to_string()
        ))
    }
}
