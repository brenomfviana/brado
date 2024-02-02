#[cfg(test)]
mod cnh_tests {
    use brado;
    use brado::Document;

    #[test]
    fn validate_cnh_1() {
        let cnh_doc: Document = Document::new("84718735264");
        assert_eq!(true, brado::cnh::validate(&cnh_doc, false, false));
    }

    #[test]
    fn validate_cnh_2() {
        let cnh_doc: Document = Document::new("847 187 352 64");
        assert_eq!(true, brado::cnh::validate(&cnh_doc, true, false));
    }

    #[test]
    fn validate_str_cnh_1() {
        let bare_cnh = "84718735264";
        assert_eq!(true, brado::cnh::validate_str(&bare_cnh, false, false));
    }

    #[test]
    fn validate_str_cnh_2() {
        let bare_cnh = "847 187 352 64";
        assert_eq!(true, brado::cnh::validate_str(&bare_cnh, true, false));
    }
}
