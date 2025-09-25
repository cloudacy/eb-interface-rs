use std::sync::LazyLock;

use regex::Regex;

use crate::xml::{ToXml, XmlElement};

#[derive(Default)]
enum PaymentMethodType<'a> {
    #[default]
    NoPayment,
    SEPADirectDebit(PaymentMethodSEPADirectDebit<'a>),
    UniversalBankTransactionBeneficiaryAccount(
        PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a>,
    ),
    UniversalBankTransaction(PaymentMethodUniversalBankTransaction<'a>),
    PaymentCard(PaymentMethodPaymentCard<'a>),
    OtherPayment,
}

impl ToXml for PaymentMethodType<'_> {
    fn to_xml(&self) -> String {
        match self {
            PaymentMethodType::NoPayment => XmlElement::new("NoPayment").to_xml(),
            PaymentMethodType::SEPADirectDebit(p) => p.to_xml(),
            PaymentMethodType::UniversalBankTransactionBeneficiaryAccount(p) => p.to_xml(),
            PaymentMethodType::UniversalBankTransaction(p) => p.to_xml(),
            PaymentMethodType::PaymentCard(p) => p.to_xml(),
            PaymentMethodType::OtherPayment => XmlElement::new("OtherPayment").to_xml(),
        }
    }
}

#[derive(Default)]
pub struct PaymentMethodSEPADirectDebit<'a> {
    direct_debit_type: Option<&'a str>,
    bic: Option<&'a str>,
    iban: Option<&'a str>,
    bank_account_owner: Option<&'a str>,
    creditor_id: Option<&'a str>,
    mandate_reference: Option<&'a str>,
    debit_collection_date: Option<&'a str>,
}

const BIC_REGEX_STR: &str = r"^[0-9A-Za-z]{8}([0-9A-Za-z]{3})?$";

trait MatchBicRegex {
    fn match_bic_regex(bic: &str) -> bool {
        static BIC_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(BIC_REGEX_STR).unwrap());
        !BIC_REGEX.is_match(bic)
    }
}

impl MatchBicRegex for PaymentMethodSEPADirectDebit<'_> {}

impl<'a> PaymentMethodSEPADirectDebit<'a> {
    pub fn new() -> Self {
        PaymentMethodSEPADirectDebit {
            ..Default::default()
        }
    }

    pub fn with_direct_debit_type(mut self, direct_debit_type: &'a str) -> Self {
        self.direct_debit_type = Some(direct_debit_type);
        self
    }

    pub fn with_bic(mut self, bic: &'a str) -> Result<Self, String> {
        if !PaymentMethodSEPADirectDebit::match_bic_regex(bic) {
            return Err(format!("BIC {bic} doesn't match regex {BIC_REGEX_STR}!"));
        }
        self.bic = Some(bic);
        Ok(self)
    }

    pub fn with_iban(mut self, iban: &'a str) -> Result<Self, String> {
        if iban.len() > 34 {
            return Err(format!("IBAN {iban} is too long!"));
        }
        self.iban = Some(iban);
        Ok(self)
    }

    pub fn with_bank_account_owner(mut self, bank_account_owner: &'a str) -> Result<Self, String> {
        if bank_account_owner.len() > 70 {
            return Err(format!(
                "BankAccountOwner {bank_account_owner} is too long!"
            ));
        }
        self.bank_account_owner = Some(bank_account_owner);
        Ok(self)
    }

    pub fn with_creditor_id(mut self, creditor_id: &'a str) -> Result<Self, String> {
        if creditor_id.len() > 35 {
            return Err(format!("CreditorID {creditor_id} is too long!"));
        }
        self.creditor_id = Some(creditor_id);
        Ok(self)
    }

    pub fn with_mandate_reference(mut self, mandate_reference: &'a str) -> Result<Self, String> {
        if mandate_reference.len() > 35 {
            return Err(format!("MandateReference {mandate_reference} is too long!"));
        }
        self.mandate_reference = Some(mandate_reference);
        Ok(self)
    }

    pub fn with_debit_collection_date(
        mut self,
        debit_collection_date: &'a str,
    ) -> Result<Self, String> {
        static DATE_REGEX_STR: &str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$";
        static DATE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(DATE_REGEX_STR).unwrap());
        if !DATE_REGEX.is_match(debit_collection_date) {
            return Err(format!(
                "DebitCollectionDate {debit_collection_date} doesn't match regex {DATE_REGEX_STR}!"
            ));
        }
        self.debit_collection_date = Some(debit_collection_date);
        Ok(self)
    }
}

impl ToXml for PaymentMethodSEPADirectDebit<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("SEPADirectDebit");

        e = e.with_text_element("Type", self.direct_debit_type.unwrap_or("B2C"));

        if let Some(bic) = self.bic {
            e = e.with_text_element("BIC", bic);
        }

        if let Some(iban) = self.iban {
            e = e.with_text_element("IBAN", iban);
        }

        if let Some(bank_account_owner) = self.bank_account_owner {
            e = e.with_text_element("BankAccountOwner", bank_account_owner);
        }

        if let Some(creditor_id) = self.creditor_id {
            e = e.with_text_element("CreditorID", creditor_id);
        }

        if let Some(mandate_reference) = self.mandate_reference {
            e = e.with_text_element("MandateReference", mandate_reference);
        }

        if let Some(debit_collection_date) = self.debit_collection_date {
            e = e.with_text_element("DebitCollectionDate", debit_collection_date);
        }

        e.to_xml()
    }
}

/// - bank_code_type: ISO 3166-1 Code
pub struct PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a> {
    bank_code: i64,
    bank_code_type: &'a str,
}

impl<'a> PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a> {
    pub fn new(bank_code: i64, bank_code_type: &'a str) -> Self {
        PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode {
            bank_code,
            bank_code_type,
        }
    }
}

#[derive(Default)]
pub struct PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
    bank_name: Option<&'a str>,
    bank_code: Option<PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a>>,
    bic: Option<&'a str>,
    bank_account_number: Option<&'a str>,
    iban: Option<&'a str>,
    bank_account_owner: Option<&'a str>,
}

impl MatchBicRegex for PaymentMethodUniversalBankTransactionBeneficiaryAccount<'_> {}

impl<'a> PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
    pub fn new() -> Self {
        PaymentMethodUniversalBankTransactionBeneficiaryAccount {
            ..Default::default()
        }
    }

    pub fn with_bank_name(mut self, bank_name: &'a str) -> Result<Self, String> {
        if bank_name.len() > 255 {
            return Err(format!("BankName {bank_name} is too long!"));
        }
        self.bank_name = Some(bank_name);
        Ok(self)
    }

    pub fn with_bank_code(
        mut self,
        bank_code: PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a>,
    ) -> Result<Self, String> {
        if bank_code.bank_code_type.len() != 2 {
            return Err(format!(
                "BankCodeType {} is not 2 characters long!",
                bank_code.bank_code_type
            ));
        }
        self.bank_code = Some(bank_code);
        Ok(self)
    }

    pub fn with_bic(mut self, bic: &'a str) -> Result<Self, String> {
        if !PaymentMethodUniversalBankTransactionBeneficiaryAccount::match_bic_regex(bic) {
            return Err(format!("BIC {bic} doesn't match regex {BIC_REGEX_STR}!"));
        }
        self.bic = Some(bic);
        Ok(self)
    }

    pub fn with_bank_account_number(mut self, bank_account_number: &'a str) -> Self {
        self.bank_account_number = Some(bank_account_number);
        self
    }

    pub fn with_iban(mut self, iban: &'a str) -> Result<Self, String> {
        if iban.len() > 34 {
            return Err(format!("IBAN {iban} is too long!"));
        }
        self.iban = Some(iban);
        Ok(self)
    }

    pub fn with_bank_account_owner(mut self, bank_account_owner: &'a str) -> Result<Self, String> {
        if bank_account_owner.len() > 70 {
            return Err(format!(
                "BankAccountOwner {bank_account_owner} is too long!"
            ));
        }
        self.bank_account_owner = Some(bank_account_owner);
        Ok(self)
    }
}

impl ToXml for PaymentMethodUniversalBankTransactionBeneficiaryAccount<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("BeneficiaryAccount");

        if let Some(bank_name) = self.bank_name {
            e = e.with_text_element("BankName", bank_name);
        }

        if let Some(bank_code) = &self.bank_code {
            let bank_code_xml_element = XmlElement::new("BankCode")
                .with_text(format!("{}", bank_code.bank_code))
                .with_attr("BankCodeType", bank_code.bank_code_type);

            e = e.with_element(&bank_code_xml_element);
        }

        if let Some(bic) = self.bic {
            e = e.with_text_element("BIC", bic);
        }

        if let Some(bank_account_number) = self.bank_account_number {
            e = e.with_text_element("BankAccountNr", bank_account_number);
        }

        if let Some(iban) = self.iban {
            e = e.with_text_element("IBAN", iban);
        }

        if let Some(bank_account_owner) = self.bank_account_owner {
            e = e.with_text_element("BankAccountOwner", bank_account_owner);
        }

        e.to_xml()
    }
}

#[derive(Default)]
pub struct PaymentMethodUniversalBankTransaction<'a> {
    consolidator_payable: Option<bool>,
    beneficiary_account: Option<Vec<PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a>>>,
    payment_reference: Option<&'a str>,
    payment_reference_checksum: Option<&'a str>,
}

impl<'a> PaymentMethodUniversalBankTransaction<'a> {
    pub fn new() -> Self {
        PaymentMethodUniversalBankTransaction {
            ..Default::default()
        }
    }

    pub fn with_consolidator_payable(mut self, consolidator_payable: bool) -> Self {
        self.consolidator_payable = Some(consolidator_payable);
        self
    }

    pub fn with_beneficiary_account(
        mut self,
        beneficiary_account: PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a>,
    ) -> Self {
        self.beneficiary_account
            .get_or_insert_with(Vec::new)
            .push(beneficiary_account);
        self
    }

    pub fn with_payment_reference(mut self, payment_reference: &'a str) -> Result<Self, String> {
        if payment_reference.len() > 35 {
            return Err(format!("PaymentReference {payment_reference} is too long!"));
        }
        self.payment_reference = Some(payment_reference);
        Ok(self)
    }

    pub fn with_payment_reference_checksum(mut self, payment_reference_checksum: &'a str) -> Self {
        self.payment_reference_checksum = Some(payment_reference_checksum);
        self
    }
}

impl ToXml for PaymentMethodUniversalBankTransaction<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("UniversalBankTransaction");

        e = e.with_attr(
            "ConsolidatorPayable",
            format!("{}", self.consolidator_payable.unwrap_or(false)),
        );

        if let Some(beneficiary_account) = &self.beneficiary_account {
            for account in beneficiary_account {
                e = e.with_element(account);
            }
        }

        if let Some(payment_reference) = self.payment_reference {
            let mut payment_reference_xml_element =
                XmlElement::new("PaymentReference").with_text(payment_reference);

            if let Some(payment_reference_checksum) = self.payment_reference_checksum {
                payment_reference_xml_element =
                    payment_reference_xml_element.with_attr("CheckSum", payment_reference_checksum);
            }

            e = e.with_element(&payment_reference_xml_element);
        }

        e.to_xml()
    }
}

/// - primary_account_number: Only provide at most the first 6 and last 4 digits, separated with a "*".
#[derive(Default)]
pub struct PaymentMethodPaymentCard<'a> {
    primary_account_number: &'a str,
    card_holder_name: Option<&'a str>,
}

impl<'a> PaymentMethodPaymentCard<'a> {
    pub fn new(primary_account_number: &'a str) -> Result<Self, String> {
        static PAYMENT_CARD_REGEX_STR: &str = r"^[0-9]{0,6}\*[0-9]{0,4}$";
        static PAYMENT_CARD_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(PAYMENT_CARD_REGEX_STR).unwrap());
        if !PAYMENT_CARD_REGEX.is_match(primary_account_number) {
            return Err(format!(
                "Invalid primary account number \"{primary_account_number}\". Only provide at most the first 6 and last 4 digits, separated with a \"*\".",
            ));
        }
        Ok(PaymentMethodPaymentCard {
            primary_account_number,
            ..Default::default()
        })
    }

    pub fn with_card_holder_name(mut self, card_holder_name: &'a str) -> Self {
        self.card_holder_name = Some(card_holder_name);
        self
    }
}

impl ToXml for PaymentMethodPaymentCard<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("PaymentCard");

        e = e.with_text_element("PrimaryAccountNumber", self.primary_account_number);

        if let Some(card_holder_name) = self.card_holder_name {
            e = e.with_text_element("CardHolderName", card_holder_name);
        }

        e.to_xml()
    }
}

#[derive(Default)]
pub struct PaymentMethod<'a> {
    comment: Option<&'a str>,
    method: PaymentMethodType<'a>,
}

impl<'a> PaymentMethod<'a> {
    pub fn no_payment() -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::NoPayment,
            ..Default::default()
        }
    }

    pub fn sepa_direct_debit(
        sepa_direct_debit: PaymentMethodSEPADirectDebit<'a>,
    ) -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::SEPADirectDebit(sepa_direct_debit),
            ..Default::default()
        }
    }

    pub fn universal_bank_transaction(
        universal_bank_transaction: PaymentMethodUniversalBankTransaction<'a>,
    ) -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::UniversalBankTransaction(universal_bank_transaction),
            ..Default::default()
        }
    }

    pub fn universal_bank_transaction_beneficiary_account(
        universal_bank_transaction_beneficiary_account: PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a>,
    ) -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::UniversalBankTransactionBeneficiaryAccount(
                universal_bank_transaction_beneficiary_account,
            ),
            ..Default::default()
        }
    }

    pub fn payment_card(payment_card: PaymentMethodPaymentCard<'a>) -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::PaymentCard(payment_card),
            ..Default::default()
        }
    }

    pub fn other_payment() -> PaymentMethod<'a> {
        PaymentMethod {
            method: PaymentMethodType::OtherPayment,
            ..Default::default()
        }
    }

    pub fn with_comment(mut self, comment: &'a str) -> Self {
        self.comment = Some(comment);
        self
    }
}

impl ToXml for PaymentMethod<'_> {
    fn to_xml(&self) -> String {
        let mut e = XmlElement::new("PaymentMethod");

        if let Some(comment) = self.comment {
            e = e.with_text_element("Comment", comment);
        }

        e = e.with_element(&self.method);

        e.to_xml()
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::ToXml;

    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            PaymentMethod {
                ..Default::default()
            }
            .to_xml(),
            "<PaymentMethod><NoPayment></NoPayment></PaymentMethod>"
        )
    }

    #[test]
    fn no_payment() {
        assert_eq!(
            PaymentMethod::no_payment().to_xml(),
            "<PaymentMethod><NoPayment></NoPayment></PaymentMethod>"
        )
    }

    #[test]
    fn sepa_direct_debit() {
        assert_eq!(
            PaymentMethod::sepa_direct_debit(PaymentMethodSEPADirectDebit {
                direct_debit_type: Some("B2B"),
                bic: Some("BKAUATWW"),
                iban: Some("AT491200011111111111"),
                bank_account_owner: Some("Test"),
                creditor_id: Some("AT12ZZZ00000000001"),
                mandate_reference: Some("123"),
                debit_collection_date: Some("2020-01-01"),
                ..Default::default()
            })
            .to_xml(),
            "<PaymentMethod><SEPADirectDebit><Type>B2B</Type><BIC>BKAUATWW</BIC><IBAN>AT491200011111111111</IBAN><BankAccountOwner>Test</BankAccountOwner><CreditorID>AT12ZZZ00000000001</CreditorID><MandateReference>123</MandateReference><DebitCollectionDate>2020-01-01</DebitCollectionDate></SEPADirectDebit></PaymentMethod>"
        )
    }

    #[test]
    fn universal_bank_transaction() {
        assert_eq!(
            PaymentMethod::universal_bank_transaction(PaymentMethodUniversalBankTransaction {
                consolidator_payable: Some(true),
                beneficiary_account: Some(vec![
                    PaymentMethodUniversalBankTransactionBeneficiaryAccount {
                        bank_name: Some("Bank"),
                        bank_code: Some(
                            PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode {
                                bank_code: 12000,
                                bank_code_type: "AT",
                            }
                        ),
                        bic: Some("BKAUATWW"),
                        bank_account_number: Some("11111111111"),
                        iban: Some("AT491200011111111111"),
                        bank_account_owner: Some("Name"),
                    }
                ]),
                payment_reference: Some("123456789012"),
                payment_reference_checksum: Some("X"),
            })
            .to_xml(),
            "<PaymentMethod><UniversalBankTransaction ConsolidatorPayable=\"true\"><BeneficiaryAccount><BankName>Bank</BankName><BankCode BankCodeType=\"AT\">12000</BankCode><BIC>BKAUATWW</BIC><BankAccountNr>11111111111</BankAccountNr><IBAN>AT491200011111111111</IBAN><BankAccountOwner>Name</BankAccountOwner></BeneficiaryAccount><PaymentReference CheckSum=\"X\">123456789012</PaymentReference></UniversalBankTransaction></PaymentMethod>"
        )
    }

    #[test]
    fn payment_card() {
        assert_eq!(
            PaymentMethod::payment_card(PaymentMethodPaymentCard {
                primary_account_number: "123456*4321",
                card_holder_name: Some("Name"),
            })
            .to_xml(),
            "<PaymentMethod><PaymentCard><PrimaryAccountNumber>123456*4321</PrimaryAccountNumber><CardHolderName>Name</CardHolderName></PaymentCard></PaymentMethod>"
        )
    }

    #[test]
    fn other_payment() {
        assert_eq!(
            PaymentMethod::other_payment().to_xml(),
            "<PaymentMethod><OtherPayment></OtherPayment></PaymentMethod>"
        )
    }
}
