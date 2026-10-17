#[cfg(test)]
mod cns_tests {
    #[test]
    fn cns_validate_1_valid_cnss() {
        let valid_cnss = [
            "144082627260004",
            "812297346500000",
            "144 0826 2726 0004",
            "812 2973 4650 0000",
        ];
        for valid_cns in valid_cnss {
            assert!(brado::cns::validate(valid_cns));
        }
    }

    #[test]
    fn cns_validate_2_invalid_cnss() {
        let invalid_cnss = [
            "14408262726000",
            "144082627260003",
            "1440826272600043",
            "144 0826 2726 000",
            "144 0826 2726 0003",
            "144 0826 2726 00043",
            "14 4082 6272 60004",
            "1440 8262 7260 004",
            "144:0826 2726 0004",
            "144 0826:2726 0004",
            "144 0826 2726:0004",
            "AAAAAAAAAAAAAAA",
            "AAA AAAA AAAA AAAA",
        ];
        for invalid_cns in invalid_cnss {
            assert!(!brado::cns::validate(invalid_cns));
        }
    }

    #[test]
    fn cns_is_bare_1_valid_bare() {
        let documents = [
            "144082627260004", // Valid CNS
            "144082627260003", // Invalid CNS
        ];
        for document in documents {
            assert!(brado::cns::is_bare(document));
        }
    }

    #[test]
    fn cns_is_bare_2_invalid_bare() {
        let documents = [
            "144 0826 2726 0004", // Valid CNS
            "144 0826 2726 0003", // Invalid CNS
            "144 082627260004",   // Invalid CNS
            "1440826 27260004",   // Invalid CNS
            "14408262726 0004",   // Invalid CNS
            "14408262726000",     // Invalid CNS
            "1440826272600043",   // Invalid CNS
        ];
        for document in documents {
            assert!(!brado::cns::is_bare(document));
        }
    }

    #[test]
    fn cns_is_masked_1_valid_masked() {
        let documents = [
            "144 0826 2726 0004", // Valid CNS
            "144 0826 2726 0003", // Invalid CNS
        ];
        for document in documents {
            assert!(brado::cns::is_masked(document));
        }
    }

    #[test]
    fn cns_is_masked_2_invalid_masked() {
        let documents = [
            "144082627260004",     // Valid CNS
            "144082627260003",     // Invalid CNS
            "144 082627260004",    // Invalid CNS
            "1440826 27260004",    // Invalid CNS
            "14408262726 0004",    // Invalid CNS
            "14408262726000",      // Invalid CNS
            "1440826272600043",    // Invalid CNS
            "144 0826 2726 000",   // Invalid CNS
            "144 0826 2726 00043", // Invalid CNS
            "14 4082 6272 60004",  // Invalid CNS
            "1440 8262 7260 004",  // Invalid CNS
            "144:0826 2726 0004",  // Invalid CNS
            "144 0826:2726 0004",  // Invalid CNS
            "144 0826 2726:0004",  // Invalid CNS
        ];
        for document in documents {
            assert!(!brado::cns::is_masked(document));
        }
    }

    #[test]
    fn cns_mask_1_valid_mask() {
        let valid_cns: &str = "144082627260004";
        assert_eq!(
            brado::cns::mask(valid_cns),
            Ok(String::from("144 0826 2726 0004"))
        );
        let invalid_cns: &str = "144082627260003";
        assert_eq!(
            brado::cns::mask(invalid_cns),
            Ok(String::from("144 0826 2726 0003"))
        );
    }

    #[test]
    fn cns_mask_2_invalid_mask() {
        let documents = [
            "144 0826 2726 0004", // Valid CNS
            "144 0826 2726 0003", // Invalid CNS
            "14408262726000",     // Invalid CNS
            "1440826272600043",   // Invalid CNS
        ];
        for document in documents {
            let result = brado::cns::mask(document);
            assert_eq!(
                result,
                Err("The given string cannot be masked as CNS!"),
            );
        }
    }

    #[test]
    fn cns_generate_1() {
        for _ in 0..1000 {
            let cns = brado::cns::generate();
            assert!(brado::cns::validate(&cns));
            assert!(brado::cns::is_bare(&cns));
        }
    }

    #[test]
    fn cns_generate_masked_1() {
        for _ in 0..1000 {
            let cns = brado::cns::generate_masked();
            assert!(brado::cns::validate(&cns));
            assert!(brado::cns::is_masked(&cns));
        }
    }
}
