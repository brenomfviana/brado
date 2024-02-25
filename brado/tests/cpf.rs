#[cfg(test)]
mod cpf_tests {
    use brado;

    #[test]
    fn cpf_validate_1_valid_bare_cpf() {
        let bare_cpf = String::from("63929247011");
        assert_eq!(brado::cpf::validate(&bare_cpf), true);
    }

    #[test]
    fn cpf_validate_2_valid_masked_cpf() {
        let masked_cpf = String::from("639.292.470-11");
        assert_eq!(brado::cpf::validate(&masked_cpf), true);
    }

    #[test]
    fn cpf_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::cpf::validate(&document), false);
        }
    }

    #[test]
    fn cpf_validate_4_invalid_mask() {
        let document = String::from("63.929.247-011");
        assert_eq!(brado::cpf::validate(&document), false);
    }

    #[test]
    fn cpf_validate_5_invalid_other_document_1() {
        let document = String::from("639292470");
        assert_eq!(brado::cpf::validate(&document), false);
    }

    #[test]
    fn cpf_validate_6_invalid_other_document_2() {
        let document = String::from("063.929.247-011");
        assert_eq!(brado::cpf::validate(&document), false);
    }

    #[test]
    fn cpf_validate_7_invalid_other_document_3() {
        let document = String::from("639.292.470-1:1");
        assert_eq!(brado::cpf::validate(&document), false);
    }

    #[test]
    fn cpf_is_bare_1_bare_cpf() {
        let bare_cpf = String::from("63929247011");
        assert_eq!(brado::cpf::is_bare(&bare_cpf), true);
    }

    #[test]
    fn cpf_is_bare_2_masked_cpf() {
        let masked_cpf = String::from("639.292.470-11");
        assert_eq!(brado::cpf::is_bare(&masked_cpf), false);
    }

    #[test]
    fn cpf_is_masked_1_masked_cpf() {
        let masked_cpf = String::from("639.292.470-11");
        assert_eq!(brado::cpf::is_masked(&masked_cpf), true);
    }

    #[test]
    fn cpf_is_masked_2_bare_cpf() {
        let bare_cpf = String::from("63929247011");
        assert_eq!(brado::cpf::is_masked(&bare_cpf), false);
    }

    #[test]
    fn cpf_mask_1_bare_cpf() {
        let bare_cpf = String::from("63929247011");
        assert_eq!(brado::cpf::mask(&bare_cpf), String::from("639.292.470-11"),);
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CPF")]
    fn cpf_mask_2_masked_cpf() {
        let masked_cpf = String::from("639.292.470-11");
        brado::cpf::mask(&masked_cpf);
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CPF")]
    fn cpf_mask_3_invalid_cpf() {
        let document = String::from("639292470");
        brado::cpf::mask(&document);
    }

    #[test]
    fn cpf_generate_1() {
        let cpf = brado::cpf::generate();
        assert_eq!(brado::cpf::validate(&cpf), true);
    }
}
