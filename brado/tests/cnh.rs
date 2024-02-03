#[cfg(test)]
mod cnh_tests {
    use brado;

    #[test]
    fn cnh_validate_1_valid_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(true, brado::cnh::validate(bare_cnh));
    }

    #[test]
    fn cnh_validate_2_valid_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        assert_eq!(true, brado::cnh::validate(masked_cnh));
    }

    #[test]
    fn cnh_validate_3_invalid_mask() {
        let document: &str = "84 718 735 264";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_4_invalid_other_document_1() {
        let document: &str = "847187352";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_5_invalid_other_document_2() {
        let document: &str = "084 718 735 264";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_6_invalid_other_document_3() {
        let document: &str = "847 187 352 6:4";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_is_bare_1_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(brado::cnh::is_bare(bare_cnh), true);
    }

    #[test]
    fn cnh_is_bare_2_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        assert_eq!(brado::cnh::is_bare(masked_cnh), false);
    }

    #[test]
    fn cnh_is_masked_1_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        assert_eq!(brado::cnh::is_masked(masked_cnh), true);
    }

    #[test]
    fn cnh_is_masked_2_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(brado::cnh::is_masked(bare_cnh), false);
    }

    #[test]
    fn cnh_mask_1_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(brado::cnh::mask(bare_cnh), String::from("847 187 352 64"),);
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CNH")]
    fn cnh_mask_2_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        brado::cnh::mask(masked_cnh);
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CNH")]
    fn cnh_mask_3_invalid_cnh() {
        let document: &str = "847187352";
        brado::cnh::mask(document);
    }
}
