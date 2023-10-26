use regex::Regex;

use crate::xml::XmlElement;

pub trait PaymentMethodType<'a> {
    fn as_xml(&self) -> Result<XmlElement<'a>, String>;
}

pub struct PaymentMethodNoPayment {}

impl<'a> PaymentMethodType<'a> for PaymentMethodNoPayment {
    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        Ok(XmlElement::new("NoPayment"))
    }
}

#[derive(Default)]
pub struct PaymentMethodSEPADirectDebit<'a> {
    pub direct_debit_type: Option<&'a str>,
    pub bic: Option<&'a str>,
    pub iban: Option<&'a str>,
    pub bank_account_owner: Option<&'a str>,
    pub creditor_id: Option<&'a str>,
    pub mandate_reference: Option<&'a str>,
    pub debit_collection_date: Option<&'a str>,
}

impl<'a> PaymentMethodType<'a> for PaymentMethodSEPADirectDebit<'a> {
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
    pub bank_code: i64,
    pub bank_code_type: &'a str,
}

#[derive(Default)]
pub struct PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
    pub bank_name: Option<&'a str>,
    pub bank_code: Option<PaymentMethodUniversalBankTransactionBeneficiaryAccountBankCode<'a>>,
    pub bic: Option<&'a str>,
    pub bank_account_number: Option<&'a str>,
    pub iban: Option<&'a str>,
    pub bank_account_owner: Option<&'a str>,
}

impl<'a> PaymentMethodType<'a> for PaymentMethodUniversalBankTransactionBeneficiaryAccount<'a> {
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

impl<'a> PaymentMethodType<'a> for PaymentMethodUniversalBankTransaction<'a> {
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
    pub primary_account_number: &'a str,
    pub card_holder_name: Option<&'a str>,
}

impl<'a> PaymentMethodType<'a> for PaymentMethodPaymentCard<'a> {
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

pub struct PaymentMethodOtherPayment {}

impl<'a> PaymentMethodType<'a> for PaymentMethodOtherPayment {
    fn as_xml(&self) -> Result<XmlElement<'a>, String> {
        Ok(XmlElement::new("OtherPayment"))
    }
}

#[derive(Default)]
pub struct PaymentMethod<'a> {
    pub comment: Option<&'a str>,
    pub payment_method_type: Box<dyn PaymentMethodType<'a> + 'a>,
}

impl<'a> Default for Box<dyn PaymentMethodType<'a>> {
    fn default() -> Box<(dyn PaymentMethodType<'a>)> {
        Box::new(PaymentMethodNoPayment {})
    }
}

impl<'a> PaymentMethod<'a> {
    pub fn with_method_type(
        mut self,
        payment_method_type: impl PaymentMethodType<'a> + 'a,
    ) -> Self {
        self.payment_method_type = Box::new(payment_method_type);
        self
    }

    pub fn as_xml(&self) -> Result<XmlElement, String> {
        let mut e = XmlElement::new("PaymentMethod");

        if let Some(comment) = self.comment {
            e = e.with_text_element("Comment", comment);
        }

        match self.payment_method_type.as_xml() {
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
            PaymentMethod {
                ..Default::default()
            }
            .with_method_type(PaymentMethodNoPayment {})
            .as_xml()
            .unwrap()
            .to_string(),
            "<PaymentMethod><NoPayment></NoPayment></PaymentMethod>"
        )
    }

    #[test]
    fn sepa_direct_debit() {
        assert_eq!(
            PaymentMethod {
                ..Default::default()
            }
            .with_method_type(PaymentMethodSEPADirectDebit {
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
            PaymentMethod {
                ..Default::default()
            }
            .with_method_type(PaymentMethodUniversalBankTransaction {
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
            PaymentMethod {
                ..Default::default()
            }
            .with_method_type(PaymentMethodPaymentCard {
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
            PaymentMethod {
                ..Default::default()
            }
            .with_method_type(PaymentMethodOtherPayment {})
            .as_xml()
            .unwrap()
            .to_string(),
            "<PaymentMethod><OtherPayment></OtherPayment></PaymentMethod>"
        )
    }
}
