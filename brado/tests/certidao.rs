#[cfg(test)]
mod certidao_tests {
    use brado;

    #[test]
    fn certidao_validate_1_valid_bare_certidao() {
        let bare_certidao: &str = "21924201552023106304243115818536";
        assert_eq!(brado::certidao::validate(bare_certidao), true);
    }

    #[test]
    fn certidao_validate_2_valid_masked_certidao() {
        let masked_certidao: &str = "219242 01 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::validate(masked_certidao), true);
    }

    #[test]
    fn certidao_validate_3_invalid_repeated_numbers() {
        for i in 1..=9 {
            let document: String = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::certidao::validate(&document), false);
        }
    }

    #[test]
    fn certidao_validate_4_invalid_bare_document() {
        let document: &str = "21924201552023106304243115818535";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_validate_5_invalid_masked_document() {
        let document: &str = "219242 01 55 2023 1 06304 243 1158185-35";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_validate_6_invalid_mask() {
        let document: &str = "21924 201 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_validate_7_invalid_other_document_1() {
        let document: &str = "219242015520231063042431158185";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_validate_8_invalid_other_document_2() {
        let document: &str = "021924 201 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_validate_9_invalid_other_document_3() {
        let document: &str = "219242 01 55 2023 1 06304 243 1158185:36";
        assert_eq!(brado::certidao::validate(document), false);
    }

    #[test]
    fn certidao_is_bare_1_bare_certidao() {
        let bare_certidao: &str = "21924201552023106304243115818536";
        assert_eq!(brado::certidao::is_bare(bare_certidao), true);
    }

    #[test]
    fn certidao_is_bare_2_masked_certidao() {
        let masked_certidao: &str = "219242 01 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::is_bare(masked_certidao), false);
    }

    #[test]
    fn certidao_is_bare_3_other_document() {
        let bare_document: &str = "21924201552023106304243115818535";
        assert_eq!(brado::certidao::is_bare(bare_document), true);
    }

    #[test]
    fn certidao_is_bare_4_other_document() {
        let bare_document: &str = "2192420 1552023106304243115818536";
        assert_eq!(brado::certidao::is_bare(bare_document), false);
    }

    #[test]
    fn certidao_is_bare_5_other_document() {
        let bare_document: &str = "219242015520231063042431158185360";
        assert_eq!(brado::certidao::is_bare(bare_document), false);
    }

    #[test]
    fn certidao_is_masked_1_masked_certidao() {
        let masked_certidao: &str = "219242 01 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::is_masked(masked_certidao), true);
    }

    #[test]
    fn certidao_is_masked_2_bare_certidao() {
        let bare_certidao: &str = "21924201552023106304243115818536";
        assert_eq!(brado::certidao::is_masked(bare_certidao), false);
    }

    #[test]
    fn certidao_is_masked_3_other_document() {
        let masked_document: &str = "2192420 01 55 2023 1 06304 243 1158185-36";
        assert_eq!(brado::certidao::is_masked(masked_document), false);
    }

    #[test]
    fn certidao_is_masked_4_other_document() {
        let masked_document: &str = "219242 01 55 2023 1 06304 243 1158185-360";
        assert_eq!(brado::certidao::is_masked(masked_document), false);
    }

    #[test]
    fn certidao_mask_1_bare_certidao() {
        let bare_certidao: &str = "21924201552023106304243115818536";
        assert_eq!(
            brado::certidao::mask(bare_certidao),
            Ok(String::from("219242 01 55 2023 1 06304 243 1158185-36"))
        );
    }

    #[test]
    fn certidao_mask_2_masked_certidao() {
        let masked_certidao: &str = "219242 01 55 2023 1 06304 243 1158185-36";
        let result = brado::certidao::mask(masked_certidao);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Certidão!"),
        );
    }

    #[test]
    fn certidao_mask_3_invalid_certidao() {
        let document: &str = "219242015520231063042431158185";
        let result = brado::certidao::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Certidão!"),
        );
    }

    #[test]
    fn certidao_generate_1() {
        let certidao = brado::certidao::generate();
        assert_eq!(brado::certidao::validate(&certidao), true);
        assert_eq!(brado::certidao::is_bare(&certidao), true);
    }

    #[test]
    fn certidao_generate_masked_1() {
        let certidao = brado::certidao::generate_masked();
        println!("{:?}", certidao);
        assert_eq!(brado::certidao::validate(&certidao), true);
        assert_eq!(brado::certidao::is_masked(&certidao), true);
    }
}
