#[cfg(test)]
mod cnpj_tests {
    use brado;

    #[test]
    fn cnpj_validate_1_valid_bare_cnpj() {
        let bare_cnpj: String = String::from("05200851000100");
        assert_eq!(true, brado::cnpj::validate(&bare_cnpj));
    }

    #[test]
    fn cnpj_validate_2_valid_masked_cnpj() {
        let masked_cnpj: String = String::from("05.200.851/0001-00");
        assert_eq!(true, brado::cnpj::validate(&masked_cnpj));
    }

    #[test]
    fn cnpj_validate_3_invalid_mask() {
        let document = String::from("0.520.085/1000-100");
        assert_eq!(brado::cnpj::validate(&document), false);
    }

    #[test]
    fn cnpj_validate_4_invalid_other_document_1() {
        let document = String::from("052008510001");
        assert_eq!(brado::cnpj::validate(&document), false);
    }

    #[test]
    fn cnpj_validate_5_invalid_other_document_2() {
        let document = String::from("00.520.085/1000-100");
        assert_eq!(brado::cnpj::validate(&document), false);
    }

    #[test]
    fn cnpj_validate_6_invalid_other_document_3() {
        let document = String::from("05.200.851/0001-0:0");
        assert_eq!(brado::cnpj::validate(&document), false);
    }

    #[test]
    fn cnpj_is_bare_1_bare_cnpj() {
        let bare_cnpj = String::from("05200851000100");
        assert_eq!(brado::cnpj::is_bare(&bare_cnpj), true);
    }

    #[test]
    fn cnpj_is_bare_2_masked_cnpj() {
        let masked_cnpj = String::from("05.200.851/0001-00");
        assert_eq!(brado::cnpj::is_bare(&masked_cnpj), false);
    }

    #[test]
    fn cnpj_is_masked_1_masked_cnpj() {
        let masked_cnpj = String::from("05.200.851/0001-00");
        assert_eq!(brado::cnpj::is_masked(&masked_cnpj), true);
    }

    #[test]
    fn cnpj_is_masked_2_bare_cnpj() {
        let bare_cnpj = String::from("05200851000100");
        assert_eq!(brado::cnpj::is_masked(&bare_cnpj), false);
    }

    #[test]
    fn cnpj_mask_1_bare_cnpj() {
        let bare_cnpj = String::from("05200851000100");
        assert_eq!(
            brado::cnpj::mask(&bare_cnpj),
            String::from("05.200.851/0001-00"),
        );
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CNPJ")]
    fn cnpj_mask_2_masked_cnpj() {
        let masked_cnpj = String::from("05.200.851/0001-00");
        brado::cnpj::mask(&masked_cnpj);
    }

    #[test]
    #[should_panic(expected = "The given string cannot be masked as CNPJ")]
    fn cnpj_mask_3_invalid_cnpj() {
        let document = String::from("052008510001");
        brado::cnpj::mask(&document);
    }

    #[test]
    fn cnpj_generate_1() {
        let cnpj = brado::cnpj::generate();
        assert_eq!(brado::cnpj::validate(&cnpj), true);
    }
}
