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

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
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
        )
    }
}
