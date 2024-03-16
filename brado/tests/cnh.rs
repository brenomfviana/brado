#[cfg(test)]
mod cnh_tests {
    use brado;

    #[test]
    fn cnh_validate_1_valid_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(brado::cnh::validate(bare_cnh), true);
    }

    #[test]
    fn cnh_validate_2_valid_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        assert_eq!(brado::cnh::validate(masked_cnh), true);
    }

    #[test]
    fn cnh_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::cnh::validate(&document), false);
        }
    }

    #[test]
    fn cnh_validate_4_invalid_bare_cnh() {
        let bare_cnh: &str = "84718735265";
        assert_eq!(brado::cnh::validate(bare_cnh), false);
    }

    #[test]
    fn cnh_validate_5_invalid_masked_cnh() {
        let masked_cnh: &str = "847 187 352 65";
        assert_eq!(brado::cnh::validate(masked_cnh), false);
    }

    #[test]
    fn cnh_validate_6_invalid_mask() {
        let document: &str = "84 718 735 264";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_7_invalid_other_document_1() {
        let document: &str = "847187352";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_8_invalid_other_document_2() {
        let document: &str = "084 718 735 264";
        assert_eq!(brado::cnh::validate(document), false);
    }

    #[test]
    fn cnh_validate_9_invalid_other_document_3() {
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
    fn cnh_is_bare_3_other_document() {
        let bare_document: &str = "84718735265";
        assert_eq!(brado::cnh::is_bare(bare_document), true);
    }

    #[test]
    fn cnh_is_bare_4_other_document() {
        let bare_document: &str = "847 18735264";
        assert_eq!(brado::cnh::is_bare(bare_document), false);
    }

    #[test]
    fn cnh_is_bare_5_other_document() {
        let bare_document: &str = "847187352645";
        assert_eq!(brado::cnh::is_bare(bare_document), false);
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
    fn cnh_is_masked_3_other_document() {
        let masked_document: &str = "8471 187 352 64";
        assert_eq!(brado::cnh::is_masked(masked_document), false);
    }

    #[test]
    fn cnh_is_masked_4_other_document() {
        let masked_document: &str = "847 187 352 645";
        assert_eq!(brado::cnh::is_masked(masked_document), false);
    }

    #[test]
    fn cnh_mask_1_bare_cnh() {
        let bare_cnh: &str = "84718735264";
        assert_eq!(
            brado::cnh::mask(bare_cnh),
            Ok(String::from("847 187 352 64"))
        );
    }

    #[test]
    fn cnh_mask_2_masked_cnh() {
        let masked_cnh: &str = "847 187 352 64";
        let result = brado::cnh::mask(masked_cnh);
        assert_eq!(result, Err("The given string cannot be masked as CNH!"),);
    }

    #[test]
    fn cnh_mask_3_invalid_cnh() {
        let document: &str = "847187352";
        let result = brado::cnh::mask(document);
        assert_eq!(result, Err("The given string cannot be masked as CNH!"),);
    }

    #[test]
    fn cnh_generate_1() {
        let cnh = brado::cnh::generate();
        assert_eq!(brado::cnh::validate(&cnh), true);
        assert_eq!(brado::cnh::is_bare(&cnh), true);
    }

    #[test]
    fn cnh_generate_masked_1() {
        let cnh = match brado::cnh::generate_masked() {
            Ok(doc) => doc,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(brado::cnh::validate(&cnh), true);
        assert_eq!(brado::cnh::is_masked(&cnh), true);
    }
}
