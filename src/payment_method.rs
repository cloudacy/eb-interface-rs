use regex::Regex;

use crate::xml::XmlElement;

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

impl<'a> PaymentMethodType<'a> {
    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        match self {
            PaymentMethodType::NoPayment => Ok(XmlElement::new("NoPayment")),
            PaymentMethodType::SEPADirectDebit(p) => p.as_xml(),
            PaymentMethodType::UniversalBankTransactionBeneficiaryAccount(p) => p.as_xml(),
            PaymentMethodType::UniversalBankTransaction(p) => p.as_xml(),
            PaymentMethodType::PaymentCard(p) => p.as_xml(),
            PaymentMethodType::OtherPayment => Ok(XmlElement::new("OtherPayment")),
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

impl<'a> PaymentMethodSEPADirectDebit<'a> {
    pub fn new() -> PaymentMethodSEPADirectDebit<'a> {
        PaymentMethodSEPADirectDebit {
            ..Default::default()
        }
    }

    pub fn with_direct_debit_type(mut self, direct_debit_type: &'a str) -> Self {
        self.direct_debit_type = Some(direct_debit_type);
        self
    }

    pub fn with_bic(mut self, bic: &'a str) -> Self {
        self.bic = Some(bic);
        self
    }

    pub fn with_iban(mut self, iban: &'a str) -> Self {
        self.iban = Some(iban);
        self
    }

    pub fn with_bank_account_owner(mut self, bank_account_owner: &'a str) -> Self {
        self.bank_account_owner = Some(bank_account_owner);
        self
    }

    pub fn with_creditor_id(mut self, creditor_id: &'a str) -> Self {
        self.creditor_id = Some(creditor_id);
        self
    }

    pub fn with_mandate_reference(mut self, mandate_reference: &'a str) -> Self {
        self.mandate_reference = Some(mandate_reference);
        self
    }

    pub fn with_debit_collection_date(mut self, debit_collection_date: &'a str) -> Self {
        self.debit_collection_date = Some(debit_collection_date);
        self
    }

    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        let mut e = XmlElement::new("SEPADirectDebit");

        e = e.with_text_element("Type", self.direct_debit_type.unwrap_or("B2C"));

        if let Some(bic) = self.bic {
            let bic_regex_str = r"^[0-9A-Za-z]{8}([0-9A-Za-z]{3})?$";
            let bic_regex = Regex::new(bic_regex_str).unwrap();
            if bic_regex.is_match(bic) {
                e = e.with_text_element("BIC", bic);
            } else {
                return Err(format!(
                    "BIC {} doesn't match regex {}!",
                    bic, bic_regex_str
                ));
            }
        }

        if let Some(iban) = self.iban {
            if iban.len() <= 34 {
                e = e.with_text_element("IBAN", iban);
            } else {
                return Err(format!("IBAN {} is too long!", iban));
            }
        }

        if let Some(bank_account_owner) = self.bank_account_owner {
            if bank_account_owner.len() <= 70 {
                e = e.with_text_element("BankAccountOwner", bank_account_owner);
            } else {
                return Err(format!(
                    "BankAccountOwner {} is too long!",
                    bank_account_owner
                ));
            }
        }

        if let Some(creditor_id) = self.creditor_id {
            if creditor_id.len() <= 35 {
                e = e.with_text_element("CreditorID", creditor_id);
            } else {
                return Err(format!("CreditorID {} is too long!", creditor_id));
            }
        }

        if let Some(mandate_reference) = self.mandate_reference {
            if mandate_reference.len() <= 35 {
                e = e.with_text_element("MandateReference", mandate_reference);
            } else {
                return Err(format!(
                    "MandateReference {} is too long!",
                    mandate_reference
                ));
            }
        }

        if let Some(debit_collection_date) = self.debit_collection_date {
            let date_regex_str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$";
            let date_regex = Regex::new(date_regex_str).unwrap();
            if date_regex.is_match(debit_collection_date) {
                e = e.with_text_element("DebitCollectionDate", debit_collection_date);
            } else {
                return Err(format!(
                    "DebitCollectionDate {} doesn't match regex {}!",
                    debit_collection_date, date_regex_str
                ));
            }
        }

        Ok(e)
    }
}

/// - bank_code_type: ISO 3166-1 Code
pub struct PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a> {
    bank_code: i64,
    bank_code_type: &'a str,
}

impl<'a> PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a> {
    pub fn new(
        bank_code: i64,
        bank_code_type: &'a str,
    ) -> PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a> {
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

impl<'a> PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
    pub fn new() -> PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
        PaymentMethodUniversalBankTransactionBeneficiaryAccount {
            ..Default::default()
        }
    }

    pub fn with_bank_name(mut self, bank_name: &'a str) -> Self {
        self.bank_name = Some(bank_name);
        self
    }

    pub fn with_bank_code(
        mut self,
        bank_code: PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a>,
    ) -> Self {
        self.bank_code = Some(bank_code);
        self
    }

    pub fn with_bic(mut self, bic: &'a str) -> Self {
        self.bic = Some(bic);
        self
    }

    pub fn with_bank_account_number(mut self, bank_account_number: &'a str) -> Self {
        self.bank_account_number = Some(bank_account_number);
        self
    }

    pub fn with_iban(mut self, iban: &'a str) -> Self {
        self.iban = Some(iban);
        self
    }

    pub fn with_bank_account_owner(mut self, bank_account_owner: &'a str) -> Self {
        self.bank_account_owner = Some(bank_account_owner);
        self
    }

    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        let mut e = XmlElement::new("BeneficiaryAccount");

        if let Some(bank_name) = self.bank_name {
            if bank_name.len() <= 255 {
                e = e.with_text_element("BankName", bank_name);
            } else {
                return Err(format!("BankName {} is too long!", bank_name));
            }
        }

        if let Some(bank_code) = &self.bank_code {
            if bank_code.bank_code_type.len() != 2 {
                return Err(format!(
                    "BankCodeType {} is not 2 characters long!",
                    bank_code.bank_code_type
                ));
            }

            let bank_code_xml_element = XmlElement::new("BankCode")
                .with_text(format!("{}", bank_code.bank_code))
                .with_attr("BankCodeType", bank_code.bank_code_type);

            e = e.with_element(bank_code_xml_element);
        }

        if let Some(bic) = self.bic {
            let bic_regex_str = r"^[0-9A-Za-z]{8}([0-9A-Za-z]{3})?$";
            let bic_regex = Regex::new(bic_regex_str).unwrap();
            if bic_regex.is_match(bic) {
                e = e.with_text_element("BIC", bic);
            } else {
                return Err(format!(
                    "BIC {} doesn't match regex {}!",
                    bic, bic_regex_str
                ));
            }
        }

        if let Some(bank_account_number) = self.bank_account_number {
            e = e.with_text_element("BankAccountNr", bank_account_number);
        }

        if let Some(iban) = self.iban {
            if iban.len() <= 34 {
                e = e.with_text_element("IBAN", iban);
            } else {
                return Err(format!("IBAN {} is too long!", iban));
            }
        }

        if let Some(bank_account_owner) = self.bank_account_owner {
            if bank_account_owner.len() <= 70 {
                e = e.with_text_element("BankAccountOwner", bank_account_owner);
            } else {
                return Err(format!(
                    "BankAccountOwner {} is too long!",
                    bank_account_owner
                ));
            }
        }

        Ok(e)
    }
}

#[derive(Default)]
pub struct PaymentMethodUniversalBankTransaction<'a> {
    pub consolidator_payable: Option<bool>,
    pub beneficiary_account:
        Option<Vec<PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a>>>,
    pub payment_reference: Option<&'a str>,
    pub payment_reference_checksum: Option<&'a str>,
}

impl<'a> PaymentMethodUniversalBankTransaction<'a> {
    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        let mut e = XmlElement::new("UniversalBankTransaction");

        e = e.with_attr(
            "ConsolidatorPayable",
            format!("{}", self.consolidator_payable.unwrap_or(false)),
        );

        if let Some(beneficiary_account) = &self.beneficiary_account {
            for account in beneficiary_account {
                e = e.with_element(match account.as_xml() {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                });
            }
        }

        if let Some(payment_reference) = self.payment_reference {
            if payment_reference.len() <= 35 {
                let mut payment_reference_xml_element =
                    XmlElement::new("PaymentReference").with_text(payment_reference);

                if let Some(payment_reference_checksum) = self.payment_reference_checksum {
                    payment_reference_xml_element = payment_reference_xml_element
                        .with_attr("CheckSum", payment_reference_checksum);
                }

                e = e.with_element(payment_reference_xml_element);
            } else {
                return Err(format!(
                    "PaymentReference {} is too long!",
                    payment_reference
                ));
            }
        }

        Ok(e)
    }
}

/// - primary_account_number: Only provide at most the first 6 and last 4 digits, separated with a "*".
#[derive(Default)]
pub struct PaymentMethodPaymentCard<'a> {
    primary_account_number: &'a str,
    card_holder_name: Option<&'a str>,
}

impl<'a> PaymentMethodPaymentCard<'a> {
    pub fn new(primary_account_number: &'a str) -> PaymentMethodPaymentCard {
        PaymentMethodPaymentCard {
            primary_account_number,
            ..Default::default()
        }
    }

    pub fn with_card_holder_name(mut self, card_holder_name: &'a str) -> Self {
        self.card_holder_name = Some(card_holder_name);
        self
    }

    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        let mut e = XmlElement::new("PaymentCard");

        let payment_card_regex_str = r"^[0-9]{0,6}\*[0-9]{0,4}$";
        let payment_card_regex = Regex::new(payment_card_regex_str).unwrap();
        if payment_card_regex.is_match(self.primary_account_number) {
            e = e.with_text_element("PrimaryAccountNumber", self.primary_account_number);
        } else {
            return Err(format!(
                "Invalid primary account number \"{}\". Only provide at most the first 6 and last 4 digits, separated with a \"*\".", self.primary_account_number
            ));
        }

        if let Some(card_holder_name) = self.card_holder_name {
            e = e.with_text_element("CardHolderName", card_holder_name);
        }

        Ok(e)
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

    pub fn as_xml(&self) -> Result<XmlElement, String> {
        let mut e = XmlElement::new("PaymentMethod");

        if let Some(comment) = self.comment {
            e = e.with_text_element("Comment", comment);
        }

        match self.method.as_xml() {
            Ok(pmt) => {
                e = e.with_element(pmt);
            }
            Err(e) => return Err(e),
        }

        Ok(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::XmlToString;

    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            PaymentMethod {
                ..Default::default()
            }
            .as_xml()
            .unwrap()
            .to_string(),
            "<PaymentMethod><NoPayment></NoPayment></PaymentMethod>"
        )
    }

    #[test]
    fn no_payment() {
        assert_eq!(
            PaymentMethod::no_payment().as_xml().unwrap().to_string(),
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
            .as_xml()
            .unwrap()
            .to_string(),
            "<PaymentMethod><SEPADirectDebit><Type>B2B</Type><BIC>BKAUATWW</BIC><IBAN>AT491200011111111111</IBAN><BankAccountOwner>Test</BankAccountOwner><CreditorID>AT12ZZZ00000000001</CreditorID><MandateReference>123</MandateReference><DebitCollectionDate>2020-01-01</DebitCollectionDate></SEPADirectDebit></PaymentMethod>"
        )
    }

    #[test]
    fn universal_bank_transaction() {
        assert_eq!(
            PaymentMethod::universal_bank_transaction(PaymentMethodUniversalBankTransaction {
                consolidator_payable: Some(true),
                beneficiary_account: Some(vec![PaymentMethodUniversalBankTransactionBeneficiaryAccount {
                    bank_name: Some("Bank"),
                    bank_code: Some(PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode {
                        bank_code: 12000,
                        bank_code_type: "AT",
                    }),
                    bic: Some("BKAUATWW"),
                    bank_account_number: Some("11111111111"),
                    iban: Some("AT491200011111111111"),
                    bank_account_owner: Some("Name"),
                }]),
                payment_reference: Some("123456789012"),
                payment_reference_checksum: Some("X"),
            })
            .as_xml()
            .unwrap()
            .to_string(),
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
            .as_xml()
            .unwrap()
            .to_string(),
            "<PaymentMethod><PaymentCard><PrimaryAccountNumber>123456*4321</PrimaryAccountNumber><CardHolderName>Name</CardHolderName></PaymentCard></PaymentMethod>"
        )
    }

    #[test]
    fn other_payment() {
        assert_eq!(
            PaymentMethod::other_payment().as_xml().unwrap().to_string(),
            "<PaymentMethod><OtherPayment></OtherPayment></PaymentMethod>"
        )
    }
}
