#[cfg(test)]
mod cnh_tests {
    #[test]
    fn cnh_validate_1_valid_cnhs() {
        let valid_cnhs = ["84718735264", "847 187 352 64"];
        for valid_cnh in valid_cnhs {
            assert!(brado::cnh::validate(valid_cnh));
        }
    }

    #[test]
    fn cnh_validate_2_invalid_cnhs() {
        let invalid_cnhs = [
            "8471873526",
            "84718735263",
            "847187352643",
            "847 187 352 6",
            "847 187 352 63",
            "847 187 352 643",
            "84 718 735 264",
            "8471 873 526 4",
            "847:187 352 64",
            "847 187:352 64",
            "847 187 352:64",
            "AAAAAAAAAAA",
            "AAA AAA AAA AA",
        ];
        for invalid_cnh in invalid_cnhs {
            assert!(!brado::cnh::validate(invalid_cnh));
        }
    }

    #[test]
    fn cnh_validate_3_invalid_cnhs_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).concat();
            assert!(!brado::cnh::validate(&document));
        }
    }

    #[test]
    fn cnh_is_bare_1_valid_bare() {
        let documents = [
            "84718735264", // Valid CNH
            "84718735263", // Invalid CNH
        ];
        for document in documents {
            assert!(brado::cnh::is_bare(document));
        }
    }

    #[test]
    fn cnh_is_bare_2_invalid_bare() {
        let documents = [
            "847 187 352 64", // Valid CNH
            "847 187 352 63", // Invalid CNH
            "847 18735264",   // Invalid CNH
            "847187 35264",   // Invalid CNH
            "847187352 64",   // Invalid CNH
            "8471873526",     // Invalid CNH
            "847187352643",   // Invalid CNH
        ];
        for document in documents {
            assert!(!brado::cnh::is_bare(document));
        }
    }

    #[test]
    fn cnh_is_masked_1_valid_masked() {
        let documents = [
            "847 187 352 64", // Valid CNH
            "847 187 352 63", // Invalid CNH
        ];
        for document in documents {
            assert!(brado::cnh::is_masked(document));
        }
    }

    #[test]
    fn cnh_is_masked_2_invalid_masked() {
        let documents = [
            "84718735264",     // Valid CNH
            "84718735263",     // Invalid CNH
            "847 18735264",    // Invalid CNH
            "847187 35264",    // Invalid CNH
            "847187352 64",    // Invalid CNH
            "8471873526",      // Invalid CNH
            "847187352643",    // Invalid CNH
            "847 187 352 6",   // Invalid CNH
            "847 187 352 643", // Invalid CNH
            "84 718 735 264",  // Invalid CNH
            "8471 873 526 4",  // Invalid CNH
            "847:187 352 64",  // Invalid CNH
            "847 187:352 64",  // Invalid CNH
            "847 187 352:64",  // Invalid CNH
        ];
        for document in documents {
            assert!(!brado::cnh::is_masked(document));
        }
    }

    #[test]
    fn cnh_mask_1_valid_mask() {
        let valid_cnh: &str = "84718735264";
        assert_eq!(
            brado::cnh::mask(valid_cnh),
            Ok(String::from("847 187 352 64"))
        );
        let invalid_cnh: &str = "84718735263";
        assert_eq!(
            brado::cnh::mask(invalid_cnh),
            Ok(String::from("847 187 352 63"))
        );
    }

    #[test]
    fn cnh_mask_2_invalid_mask() {
        let documents = [
            "847 187 352 64", // Valid CNH
            "847 187 352 63", // Invalid CNH
            "8471873526",     // Invalid CNH
            "847187352643",   // Invalid CNH
        ];
        for document in documents {
            let result = brado::cnh::mask(document);
            assert_eq!(
                result,
                Err("The given string cannot be masked as CNH!"),
            );
        }
    }

    #[test]
    fn cnh_generate_1() {
        for _ in 0..1000 {
            let cnh = brado::cnh::generate();
            assert!(brado::cnh::validate(&cnh));
            assert!(brado::cnh::is_bare(&cnh));
        }
    }

    #[test]
    fn cnh_generate_masked_1() {
        for _ in 0..1000 {
            let cnh = brado::cnh::generate_masked();
            assert!(brado::cnh::validate(&cnh));
            assert!(brado::cnh::is_masked(&cnh));
        }
    }
}
