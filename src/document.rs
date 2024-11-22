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

impl ToString for DocumentType {
    fn to_string(&self) -> String {
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
        .to_owned()
    }
}
