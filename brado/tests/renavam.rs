#[cfg(test)]
mod renavam_tests {
    use brado;

    #[test]
    fn renavam_validate_1_valid_bare_renavam() {
        let bare_renavam: &str = "79072338363";
        assert_eq!(brado::renavam::validate(bare_renavam), true);
    }

    #[test]
    fn renavam_validate_2_valid_masked_renavam() {
        let masked_renavam: &str = "7907233836-3";
        assert_eq!(brado::renavam::validate(masked_renavam), true);
    }

    #[test]
    fn renavam_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::renavam::validate(&document), false);
        }
    }

    #[test]
    fn renavam_validate_4_invalid_bare_document() {
        let document: &str = "79072338362";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_validate_5_invalid_masked_document() {
        let document: &str = "7907233836-2";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_validate_6_invalid_mask() {
        let document: &str = "790723383-63";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_validate_7_invalid_other_document_1() {
        let document: &str = "790723383";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_validate_8_invalid_other_document_2() {
        let document: &str = "07907233836-3";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_validate_9_invalid_other_document_3() {
        let document: &str = "7907233836:3";
        assert_eq!(brado::renavam::validate(document), false);
    }

    #[test]
    fn renavam_is_bare_1_bare_renavam() {
        let bare_renavam: &str = "79072338363";
        assert_eq!(brado::renavam::is_bare(bare_renavam), true);
    }

    #[test]
    fn renavam_is_bare_2_masked_renavam() {
        let masked_renavam: &str = "7907233836-3";
        assert_eq!(brado::renavam::is_bare(masked_renavam), false);
    }

    #[test]
    fn renavam_is_bare_3_other_document() {
        let bare_document: &str = "79072338362";
        assert_eq!(brado::renavam::is_bare(bare_document), true);
    }

    #[test]
    fn renavam_is_bare_4_other_document() {
        let bare_document: &str = "790-72338362";
        assert_eq!(brado::renavam::is_bare(bare_document), false);
    }

    #[test]
    fn renavam_is_bare_5_other_document() {
        let bare_document: &str = "790723383630";
        assert_eq!(brado::renavam::is_bare(bare_document), false);
    }

    #[test]
    fn renavam_is_masked_1_masked_renavam() {
        let masked_renavam: &str = "7907233836-3";
        assert_eq!(brado::renavam::is_masked(masked_renavam), true);
    }

    #[test]
    fn renavam_is_masked_2_bare_renavam() {
        let bare_renavam: &str = "79072338363";
        assert_eq!(brado::renavam::is_masked(bare_renavam), false);
    }

    #[test]
    fn renavam_is_masked_3_other_document() {
        let masked_document: &str = "79072338-363";
        assert_eq!(brado::renavam::is_masked(masked_document), false);
    }

    #[test]
    fn renavam_is_masked_4_other_document() {
        let masked_document: &str = "7907233836-30";
        assert_eq!(brado::renavam::is_masked(masked_document), false);
    }

    #[test]
    fn renavam_mask_1_bare_renavam() {
        let bare_renavam: &str = "79072338363";
        assert_eq!(
            brado::renavam::mask(bare_renavam),
            Ok(String::from("7907233836-3"))
        );
    }

    #[test]
    fn renavam_mask_2_masked_renavam() {
        let masked_renavam: &str = "7907233836-3";
        let result = brado::renavam::mask(masked_renavam);
        assert_eq!(
            result,
            Err("The given string cannot be masked as RENAVAM!"),
        );
    }

    #[test]
    fn renavam_mask_3_invalid_renavam() {
        let document: &str = "790723383";
        let result = brado::renavam::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as RENAVAM!"),
        );
    }

    #[test]
    fn renavam_generate_1() {
        let renavam = brado::renavam::generate();
        assert_eq!(brado::renavam::validate(&renavam), true);
        assert_eq!(brado::renavam::is_bare(&renavam), true);
    }

    #[test]
    fn renavam_generate_masked_1() {
        let renavam = brado::renavam::generate_masked();
        assert_eq!(brado::renavam::validate(&renavam), true);
        assert_eq!(brado::renavam::is_masked(&renavam), true);
    }
}
