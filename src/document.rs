pub enum DocumentType {
    CreditMemo,
    FinalSettlement,
    Invoice,
    InvoiceForAdvancePayment,
    InvoiceForPartialDelivery,
    SelfBilling,
    SubsequentCredit,
    SubsequentDebit,
}

impl DocumentType {
    pub fn as_str(&self) -> &str {
        match self {
            DocumentType::CreditMemo => "CreditMemo",
            DocumentType::FinalSettlement => "FinalSettlement",
            DocumentType::Invoice => "Invoice",
            DocumentType::InvoiceForAdvancePayment => "InvoiceForAdvancePayment",
            DocumentType::InvoiceForPartialDelivery => "InvoiceForPartialDelivery",
            DocumentType::SelfBilling => "SelfBilling",
            DocumentType::SubsequentCredit => "SubsequentCredit",
            DocumentType::SubsequentDebit => "SubsequentDebit",
        }
    }
}
