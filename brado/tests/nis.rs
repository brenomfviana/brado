#[cfg(test)]
mod nis_tests {
    use brado;

    #[test]
    fn nis_validate_1_valid_bare_nis() {
        let bare_nis: &str = "40865658047";
        assert_eq!(brado::nis::validate(bare_nis), true);
    }

    #[test]
    fn nis_validate_2_valid_masked_nis() {
        let masked_nis: &str = "408.65658.04-7";
        assert_eq!(brado::nis::validate(masked_nis), true);
    }

    #[test]
    fn nis_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::nis::validate(&document), false);
        }
    }

    #[test]
    fn nis_validate_4_invalid_bare_document() {
        let document: &str = "40865658046";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_validate_5_invalid_masked_document() {
        let document: &str = "408.65658.04-6";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_validate_6_invalid_mask() {
        let document: &str = "408.656.580-47";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_validate_7_invalid_other_document_1() {
        let document: &str = "408656580";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_validate_8_invalid_other_document_2() {
        let document: &str = "040.865.658-047";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_validate_9_invalid_other_document_3() {
        let document: &str = "408.65658.04:6";
        assert_eq!(brado::nis::validate(document), false);
    }

    #[test]
    fn nis_is_bare_1_bare_nis() {
        let bare_nis: &str = "40865658047";
        assert_eq!(brado::nis::is_bare(bare_nis), true);
    }

    #[test]
    fn nis_is_bare_2_masked_nis() {
        let masked_nis: &str = "408.65658.04-7";
        assert_eq!(brado::nis::is_bare(masked_nis), false);
    }

    #[test]
    fn nis_is_bare_3_other_document() {
        let bare_document: &str = "40865658046";
        assert_eq!(brado::nis::is_bare(bare_document), true);
    }

    #[test]
    fn nis_is_bare_4_other_document() {
        let bare_document: &str = "408.65658046";
        assert_eq!(brado::nis::is_bare(bare_document), false);
    }

    #[test]
    fn nis_is_bare_5_other_document() {
        let bare_document: &str = "408656580470";
        assert_eq!(brado::nis::is_bare(bare_document), false);
    }

    #[test]
    fn nis_is_masked_1_masked_nis() {
        let masked_nis: &str = "408.65658.04-7";
        assert_eq!(brado::nis::is_masked(masked_nis), true);
    }

    #[test]
    fn nis_is_masked_2_bare_nis() {
        let bare_nis: &str = "40865658047";
        assert_eq!(brado::nis::is_masked(bare_nis), false);
    }

    #[test]
    fn nis_is_masked_3_other_document() {
        let masked_document: &str = "4086.65658.04-7";
        assert_eq!(brado::nis::is_masked(masked_document), false);
    }

    #[test]
    fn nis_is_masked_4_other_document() {
        let masked_document: &str = "408.65658.04-70";
        assert_eq!(brado::nis::is_masked(masked_document), false);
    }

    #[test]
    fn nis_mask_1_bare_nis() {
        let bare_nis: &str = "40865658047";
        assert_eq!(
            brado::nis::mask(bare_nis),
            Ok(String::from("408.65658.04-7"))
        );
    }

    #[test]
    fn nis_mask_2_masked_nis() {
        let masked_nis: &str = "408.65658.04-7";
        let result = brado::nis::mask(masked_nis);
        assert_eq!(
            result,
            Err("The given string cannot be masked as NIS/NIT/PIS/PASEP!"),
        );
    }

    #[test]
    fn nis_mask_3_invalid_nis() {
        let document: &str = "408656580";
        let result = brado::nis::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as NIS/NIT/PIS/PASEP!"),
        );
    }

    #[test]
    fn nis_generate_1() {
        let nis = brado::nis::generate();
        assert_eq!(brado::nis::validate(&nis), true);
        assert_eq!(brado::nis::is_bare(&nis), true);
    }

    #[test]
    fn nis_generate_masked_1() {
        let nis = brado::nis::generate_masked();
        assert_eq!(brado::nis::validate(&nis), true);
        assert_eq!(brado::nis::is_masked(&nis), true);
    }
}
