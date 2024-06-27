#[cfg(test)]
mod eleitoral_tests {
    use brado;

    #[test]
    fn eleitoral_validate_1_valid_bare_eleitoral() {
        let bare_eleitoral: &str = "773537801651";
        assert_eq!(brado::eleitoral::validate(bare_eleitoral), true);
    }

    #[test]
    fn eleitoral_validate_2_valid_masked_eleitoral() {
        let masked_eleitoral: &str = "7735 3780 1651";
        assert_eq!(brado::eleitoral::validate(masked_eleitoral), true);
    }

    #[test]
    fn eleitoral_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::eleitoral::validate(&document), false);
        }
    }

    #[test]
    fn eleitoral_validate_4_invalid_bare_document() {
        let document: &str = "773537801650";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_validate_5_invalid_masked_document() {
        let document: &str = "7735 3780 1650";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_validate_6_invalid_mask() {
        let document: &str = "773 537 801 651";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_validate_7_invalid_other_document_1() {
        let document: &str = "7735378016";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_validate_8_invalid_other_document_2() {
        let document: &str = "07735 3780 1650";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_validate_9_invalid_other_document_3() {
        let document: &str = "7735 3780:1650";
        assert_eq!(brado::eleitoral::validate(document), false);
    }

    #[test]
    fn eleitoral_is_bare_1_bare_eleitoral() {
        let bare_eleitoral: &str = "773537801651";
        assert_eq!(brado::eleitoral::is_bare(bare_eleitoral), true);
    }

    #[test]
    fn eleitoral_is_bare_2_masked_eleitoral() {
        let masked_eleitoral: &str = "7735 3780 1651";
        assert_eq!(brado::eleitoral::is_bare(masked_eleitoral), false);
    }

    #[test]
    fn eleitoral_is_bare_3_other_document() {
        let bare_document: &str = "773537801650";
        assert_eq!(brado::eleitoral::is_bare(bare_document), true);
    }

    #[test]
    fn eleitoral_is_bare_4_other_document() {
        let bare_document: &str = "7735 37801650";
        assert_eq!(brado::eleitoral::is_bare(bare_document), false);
    }

    #[test]
    fn eleitoral_is_bare_5_other_document() {
        let bare_document: &str = "7735378016510";
        assert_eq!(brado::eleitoral::is_bare(bare_document), false);
    }

    #[test]
    fn eleitoral_is_masked_1_masked_eleitoral() {
        let masked_eleitoral: &str = "7735 3780 1651";
        assert_eq!(brado::eleitoral::is_masked(masked_eleitoral), true);
    }

    #[test]
    fn eleitoral_is_masked_2_bare_eleitoral() {
        let bare_eleitoral: &str = "773537801651";
        assert_eq!(brado::eleitoral::is_masked(bare_eleitoral), false);
    }

    #[test]
    fn eleitoral_is_masked_3_other_document() {
        let masked_document: &str = "77353 3780 1651";
        assert_eq!(brado::eleitoral::is_masked(masked_document), false);
    }

    #[test]
    fn eleitoral_is_masked_4_other_document() {
        let masked_document: &str = "7735 3780 16510";
        assert_eq!(brado::eleitoral::is_masked(masked_document), false);
    }

    #[test]
    fn eleitoral_mask_1_bare_eleitoral() {
        let bare_eleitoral: &str = "773537801651";
        assert_eq!(
            brado::eleitoral::mask(bare_eleitoral),
            Ok(String::from("7735 3780 1651"))
        );
    }

    #[test]
    fn eleitoral_mask_2_masked_eleitoral() {
        let masked_eleitoral: &str = "7735 3780 1651";
        let result = brado::eleitoral::mask(masked_eleitoral);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Título Eleitoral!"),
        );
    }

    #[test]
    fn eleitoral_mask_3_invalid_eleitoral() {
        let document: &str = "7735378016";
        let result = brado::eleitoral::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Título Eleitoral!"),
        );
    }

    #[test]
    fn eleitoral_generate_1() {
        let eleitoral = brado::eleitoral::generate();
        assert_eq!(brado::eleitoral::validate(&eleitoral), true);
        assert_eq!(brado::eleitoral::is_bare(&eleitoral), true);
    }

    #[test]
    fn eleitoral_generate_masked_1() {
        let eleitoral = brado::eleitoral::generate_masked();
        assert_eq!(brado::eleitoral::validate(&eleitoral), true);
        assert_eq!(brado::eleitoral::is_masked(&eleitoral), true);
    }
}
