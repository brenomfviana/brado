#[cfg(test)]
mod cnpj_tests {
    use brado;

    #[test]
    fn cnpj_validate_1_valid_bare_cnpj() {
        let bare_cnpj: &str = "05200851000100";
        assert_eq!(brado::cnpj::validate(bare_cnpj), true);
    }

    #[test]
    fn cnpj_validate_2_valid_masked_cnpj() {
        let masked_cnpj: &str = "05.200.851/0001-00";
        assert_eq!(brado::cnpj::validate(masked_cnpj), true);
    }

    #[test]
    fn cnpj_validate_3_invalid_bare_cnpj() {
        let bare_cnpj: &str = "05200851000101";
        assert_eq!(brado::cnpj::validate(bare_cnpj), false);
    }

    #[test]
    fn cnpj_validate_4_invalid_masked_cnpj() {
        let masked_cnpj: &str = "05.200.851/0001-01";
        assert_eq!(brado::cnpj::validate(masked_cnpj), false);
    }

    #[test]
    fn cnpj_validate_5_invalid_mask() {
        let document: &str = "0.520.085/1000-100";
        assert_eq!(brado::cnpj::validate(document), false);
    }

    #[test]
    fn cnpj_validate_6_invalid_other_document_1() {
        let document: &str = "052008510001";
        assert_eq!(brado::cnpj::validate(document), false);
    }

    #[test]
    fn cnpj_validate_7_invalid_other_document_2() {
        let document: &str = "00.520.085/1000-100";
        assert_eq!(brado::cnpj::validate(document), false);
    }

    #[test]
    fn cnpj_validate_8_invalid_other_document_3() {
        let document: &str = "05.200.851/0001-0:0";
        assert_eq!(brado::cnpj::validate(document), false);
    }

    #[test]
    fn cnpj_is_bare_1_bare_cnpj() {
        let bare_cnpj: &str = "05200851000100";
        assert_eq!(brado::cnpj::is_bare(bare_cnpj), true);
    }

    #[test]
    fn cnpj_is_bare_2_masked_cnpj() {
        let masked_cnpj: &str = "05.200.851/0001-00";
        assert_eq!(brado::cnpj::is_bare(masked_cnpj), false);
    }

    #[test]
    fn cnpj_is_bare_3_other_document() {
        let bare_document: &str = "05200851000101";
        assert_eq!(brado::cnpj::is_bare(bare_document), true);
    }

    #[test]
    fn cnpj_is_bare_4_other_document() {
        let bare_document: &str = "052.00851000100";
        assert_eq!(brado::cnpj::is_bare(bare_document), false);
    }

    #[test]
    fn cnpj_is_bare_5_other_document() {
        let bare_document: &str = "052008510001001";
        assert_eq!(brado::cnpj::is_bare(bare_document), false);
    }

    #[test]
    fn cnpj_is_masked_1_masked_cnpj() {
        let masked_cnpj: &str = "05.200.851/0001-00";
        assert_eq!(brado::cnpj::is_masked(masked_cnpj), true);
    }

    #[test]
    fn cnpj_is_masked_2_bare_cnpj() {
        let bare_cnpj: &str = "05200851000100";
        assert_eq!(brado::cnpj::is_masked(bare_cnpj), false);
    }

    #[test]
    fn cnpj_is_masked_3_other_document() {
        let masked_document: &str = "0.520.085/1000-100";
        assert_eq!(brado::cnpj::is_masked(masked_document), false);
    }

    #[test]
    fn cnpj_is_masked_4_other_document() {
        let masked_document: &str = "00.520.085/1000-100";
        assert_eq!(brado::cnpj::is_masked(masked_document), false);
    }

    #[test]
    fn cnpj_mask_1_bare_cnpj() {
        let bare_cnpj: &str = "05200851000100";
        assert_eq!(
            brado::cnpj::mask(bare_cnpj),
            Ok(String::from("05.200.851/0001-00")),
        );
    }

    #[test]
    fn cnpj_mask_2_masked_cnpj() {
        let masked_cnpj: &str = "05.200.851/0001-00";
        let result = brado::cnpj::mask(masked_cnpj);
        assert_eq!(result, Err("The given string cannot be masked as CNPJ!"),);
    }

    #[test]
    fn cnpj_mask_3_invalid_cnpj() {
        let document: &str = "052008510001";
        let result = brado::cnpj::mask(document);
        assert_eq!(result, Err("The given string cannot be masked as CNPJ!"),);
    }

    #[test]
    fn cnpj_generate_1() {
        let cnpj = brado::cnpj::generate();
        assert_eq!(brado::cnpj::validate(&cnpj), true);
        assert_eq!(brado::cnpj::is_bare(&cnpj), true);
    }

    #[test]
    fn cnpj_generate_masked_1() {
        let cnpj = brado::cnpj::generate_masked();
        assert_eq!(brado::cnpj::validate(&cnpj), true);
        assert_eq!(brado::cnpj::is_masked(&cnpj), true);
    }
}
