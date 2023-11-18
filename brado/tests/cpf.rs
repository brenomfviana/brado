#[cfg(test)]
mod cpf_tests {
    use brado;
    use brado::Document;

    #[test]
    fn validate_cpf_1() {
        let cpf_doc: Document = Document::new("11111111111");
        assert_eq!(false, brado::cpf::validate(&cpf_doc, false, false));
    }

    #[test]
    fn validate_cpf_2() {
        let cpf_doc: Document = Document::new("111.111.111-11");
        assert_eq!(false, brado::cpf::validate(&cpf_doc, true, false));
    }

    #[test]
    fn validate_cpf_3() {
        let cpf_doc: Document = Document::new("63929247011");
        assert_eq!(true, brado::cpf::validate(&cpf_doc, false, true));
    }

    #[test]
    fn validate_cpf_4() {
        let cpf_doc: Document = Document::new("639.292.470-11");
        assert_eq!(true, brado::cpf::validate(&cpf_doc, true, false));
    }

    #[test]
    fn validate_str_cpf_1() {
        let bare_cpf = "11111111111";
        assert_eq!(false, brado::cpf::validate_str(bare_cpf, false, false));
    }

    #[test]
    fn validate_str_cpf_2() {
        let bare_cpf = "111.111.111-11";
        assert_eq!(false, brado::cpf::validate_str(bare_cpf, true, false));
    }

    #[test]
    fn validate_str_cpf_3() {
        let bare_cpf = "63929247011";
        assert_eq!(true, brado::cpf::validate_str(bare_cpf, false, true));
    }

    #[test]
    fn validate_str_cpf_4() {
        let bare_cpf = "639.292.470-11";
        assert_eq!(true, brado::cpf::validate_str(&bare_cpf, true, false));
    }
}
