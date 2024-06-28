#[cfg(test)]
mod eleitoral_tests {
    use brado;

    #[test]
    fn eleitoral_validate_1_valid_eleitorais() {
        let valid_eleitorais = ["773537801651", "7735 3780 1651"];
        for valid_eleitoral in valid_eleitorais {
            assert_eq!(brado::eleitoral::validate(valid_eleitoral), true);
        }
    }

    #[test]
    fn eleitoral_validate_2_invalid_eleitorais() {
        let invalid_eleitorais = [
            "77353780165",
            "773537801650",
            "7735378016510",
            "7735 3780 165",
            "7735 3780 1650",
            "7735 3780 16510",
            "773 5378 01651",
            "77353 7801 651",
            "7735:3780 1651",
            "7735 3780:1651",
            "AAAAAAAAAAAA",
            "AAAA AAAA AAAA",
        ];
        for invalid_eleitoral in invalid_eleitorais {
            assert_eq!(brado::eleitoral::validate(invalid_eleitoral), false);
        }
    }

    #[test]
    fn eleitoral_is_bare_1_valid_bare() {
        let documents = [
            "773537801651", // Valid Título Eleitoral
            "773537801650", // Invalid Título Eleitoral
        ];
        for document in documents {
            assert_eq!(brado::eleitoral::is_bare(document), true);
        }
    }

    #[test]
    fn eleitoral_is_bare_2_invalid_bare() {
        let documents = [
            "7735 3780 1651", // Valid Título Eleitoral
            "7735 3780 1650", // Invalid Título Eleitoral
            "773 5378 01651", // Invalid Título Eleitoral
            "77353 7801 651", // Invalid Título Eleitoral
            "77353780165",    // Invalid Título Eleitoral
            "7735378016510",  // Invalid Título Eleitoral
        ];
        for document in documents {
            assert_eq!(brado::eleitoral::is_bare(document), false);
        }
    }

    #[test]
    fn eleitoral_is_masked_1_valid_masked() {
        let documents = [
            "7735 3780 1651", // Valid Título Eleitoral
            "7735 3780 1650", // Invalid Título Eleitoral
        ];
        for document in documents {
            assert_eq!(brado::eleitoral::is_masked(document), true);
        }
    }

    #[test]
    fn eleitoral_is_masked_2_invalid_masked() {
        let documents = [
            "773537801651", // Valid Título Eleitoral
            "773537801650", // Invalid Título Eleitoral
            "7735 37801651", // Invalid Título Eleitoral
            "77353780 1651", // Invalid Título Eleitoral
            "773 5378 01651", // Invalid Título Eleitoral
            "77353 7801 651", // Invalid Título Eleitoral
            "77353780165",  // Invalid Título Eleitoral
            "7735378016510", // Invalid Título Eleitoral
            "7735:37801651", // Invalid Título Eleitoral
            "77353780:1651", // Invalid Título Eleitoral
        ];
        for document in documents {
            assert_eq!(brado::eleitoral::is_masked(document), false);
        }
    }

    #[test]
    fn eleitoral_mask_1_valid_mask() {
        let valid_eleitoral: &str = "773537801651";
        assert_eq!(
            brado::eleitoral::mask(valid_eleitoral),
            Ok(String::from("7735 3780 1651"))
        );
        let invalid_eleitoral: &str = "773537801650";
        assert_eq!(
            brado::eleitoral::mask(invalid_eleitoral),
            Ok(String::from("7735 3780 1650"))
        );
    }

    #[test]
    fn eleitoral_mask_2_invalid_mask() {
        let documents = [
            "7735 3780 1651", // Valid Título Eleitoral
            "7735 3780 1650", // Invalid Título Eleitoral
            "77353780165",    // Invalid Título Eleitoral
            "7735378016510",  // Invalid Título Eleitoral
        ];
        for document in documents {
            let result = brado::eleitoral::mask(document);
            assert_eq!(
                result,
                Err("The given string cannot be masked as Título Eleitoral!"),
            );
        }
    }

    #[test]
    fn eleitoral_generate_1() {
        for _ in 0..1000 {
            let eleitoral = brado::eleitoral::generate();
            assert_eq!(brado::eleitoral::validate(&eleitoral), true);
            assert_eq!(brado::eleitoral::is_bare(&eleitoral), true);
        }
    }

    #[test]
    fn eleitoral_generate_masked_1() {
        for _ in 0..1000 {
            let eleitoral = brado::eleitoral::generate_masked();
            assert_eq!(brado::eleitoral::validate(&eleitoral), true);
            assert_eq!(brado::eleitoral::is_masked(&eleitoral), true);
        }
    }
}
