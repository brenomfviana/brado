#[cfg(test)]
mod cnpj_tests {
    use brado;

    #[test]
    fn validate_cnpj_1() {
        let cnpj_doc: String = String::from("05200851000100");
        assert_eq!(true, brado::cnpj::validate(&cnpj_doc, false));
    }

    #[test]
    fn validate_cnpj_2() {
        let cnpj_doc: String = String::from("05.200.851/0001-00");
        assert_eq!(true, brado::cnpj::validate(&cnpj_doc, true));
    }

    #[test]
    fn validate_str_cnpj_1() {
        let bare_cnpj: &str = "05200851000100";
        assert_eq!(true, brado::cnpj::validate_str(&bare_cnpj, false));
    }

    #[test]
    fn validate_str_cnpj_2() {
        let bare_cnpj: &str = "05.200.851/0001-00";
        assert_eq!(true, brado::cnpj::validate_str(&bare_cnpj, true));
    }
}
