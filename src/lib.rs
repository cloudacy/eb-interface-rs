pub fn generate(invoice_number: &str) -> String {
    String::from(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?> {invoice_number}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate("993433000298");
        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?> 993433000298"
        );
    }
}
