#[cfg(test)]
mod cns_tests {
    use brado;

    #[test]
    fn cns_validate_1_valid_bare_cns_1() {
        let bare_cns: &str = "144082627260004";
        assert_eq!(brado::cns::validate(bare_cns), true);
    }

    #[test]
    fn cns_validate_2_valid_bare_cns_2() {
        let bare_cns: &str = "812297346500000";
        assert_eq!(brado::cns::validate(bare_cns), true);
    }

    #[test]
    fn cns_validate_3_valid_masked_cns() {
        let masked_cns: &str = "144 0826 2726 0004";
        assert_eq!(brado::cns::validate(masked_cns), true);
    }

    #[test]
    fn cns_validate_4_invalid_bare_cns() {
        let bare_cns: &str = "144082627260005";
        assert_eq!(brado::cns::validate(bare_cns), false);
    }

    #[test]
    fn cns_validate_5_invalid_masked_cns() {
        let masked_cns: &str = "144 0826 2726 0005";
        assert_eq!(brado::cns::validate(masked_cns), false);
    }

    #[test]
    fn cns_validate_6_invalid_mask() {
        let document: &str = "14 4082 6272 60004";
        assert_eq!(brado::cns::validate(document), false);
    }

    #[test]
    fn cns_validate_7_invalid_other_document_1() {
        let document: &str = "1440826272600";
        assert_eq!(brado::cns::validate(document), false);
    }

    #[test]
    fn cns_validate_8_invalid_other_document_2() {
        let document: &str = "0144 0826 2726 0004";
        assert_eq!(brado::cns::validate(document), false);
    }

    #[test]
    fn cns_validate_9_invalid_other_document_3() {
        let document: &str = "144 0826 2726 00:04";
        assert_eq!(brado::cns::validate(document), false);
    }

    #[test]
    fn cns_is_bare_1_bare_cns() {
        let bare_cns: &str = "144082627260004";
        assert_eq!(brado::cns::is_bare(bare_cns), true);
    }

    #[test]
    fn cns_is_bare_2_masked_cns() {
        let masked_cns: &str = "144 0826 2726 0004";
        assert_eq!(brado::cns::is_bare(masked_cns), false);
    }

    #[test]
    fn cns_is_bare_3_other_document() {
        let bare_document: &str = "144082627260005";
        assert_eq!(brado::cns::is_bare(bare_document), true);
    }

    #[test]
    fn cns_is_bare_4_other_document() {
        let bare_document: &str = "144 082627260004";
        assert_eq!(brado::cns::is_bare(bare_document), false);
    }

    #[test]
    fn cns_is_bare_5_other_document() {
        let bare_document: &str = "1440826272600045";
        assert_eq!(brado::cns::is_bare(bare_document), false);
    }

    #[test]
    fn cns_is_masked_1_masked_cns() {
        let masked_cns: &str = "144 0826 2726 0004";
        assert_eq!(brado::cns::is_masked(masked_cns), true);
    }

    #[test]
    fn cns_is_masked_2_bare_cns() {
        let bare_cns: &str = "144082627260004";
        assert_eq!(brado::cns::is_masked(bare_cns), false);
    }

    #[test]
    fn cns_is_masked_3_other_document() {
        let masked_document: &str = "1440 0826 2726 0004";
        assert_eq!(brado::cns::is_masked(masked_document), false);
    }

    #[test]
    fn cns_is_masked_4_other_document() {
        let masked_document: &str = "144 0826 2726 00045";
        assert_eq!(brado::cns::is_masked(masked_document), false);
    }

    #[test]
    fn cns_mask_1_bare_cns() {
        let bare_cns: &str = "144082627260004";
        assert_eq!(
            brado::cns::mask(bare_cns),
            Ok(String::from("144 0826 2726 0004"))
        );
    }

    #[test]
    fn cns_mask_2_masked_cns() {
        let masked_cns: &str = "144 0826 2726 0004";
        let result = brado::cns::mask(masked_cns);
        assert_eq!(result, Err("The given string cannot be masked as CNS!"),);
    }

    #[test]
    fn cns_mask_3_invalid_cns() {
        let document: &str = "1440826272600";
        let result = brado::cns::mask(document);
        assert_eq!(result, Err("The given string cannot be masked as CNS!"),);
    }

    #[test]
    fn cns_generate_1() {
        let cns = brado::cns::generate();
        assert_eq!(brado::cns::validate(&cns), true);
        assert_eq!(brado::cns::is_bare(&cns), true);
    }

    #[test]
    fn cns_generate_masked_1() {
        let cns = brado::cns::generate_masked();
        assert_eq!(brado::cns::validate(&cns), true);
        assert_eq!(brado::cns::is_masked(&cns), true);
    }
}
