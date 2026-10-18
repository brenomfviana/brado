//! Testes de integração do módulo Certidão.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Certidao`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Certidao;

#[test]
fn certidao_validate_1_valid_certidaos() {
    let valid_certidaos = [
        "21924201552023106304243115818536",
        "219242 01 55 2023 1 06304 243 1158185-36",
    ];
    for valid_certidao in valid_certidaos {
        assert!(Certidao::validate(valid_certidao));
    }
}

#[test]
fn certidao_validate_2_invalid_certidaos() {
    let invalid_certidaos = [
        "2192420155202310630424311581853",
        "21924201552023106304243115818535",
        "219242015520231063042431158185365",
        "219242 01 55 2023 1 06304 243 1158185-3",
        "219242 01 55 2023 1 06304 243 1158185-35",
        "219242 01 55 2023 1 06304 243 1158185-365",
        "219242 01552023106304243115818536",
        "21924201 552023106304243115818536",
        "2192420155 2023106304243115818536",
        "21924201552023 106304243115818536",
        "219242015520231 06304243115818536",
        "21924201552023106304 243115818536",
        "21924201552023106304243 115818536",
        "219242015520231063042431158185-36",
        "21924 20 15 5202 3 10630 424 3115818-536",
        "2192420 15 52 0231 0 63042 431 1581853-6",
        "219242:01 55 2023 1 06304 243 1158185-36",
        "219242 01:55 2023 1 06304 243 1158185-36",
        "219242 01 55:2023 1 06304 243 1158185-36",
        "219242 01 55 2023:1 06304 243 1158185-36",
        "219242 01 55 2023 1:06304 243 1158185-36",
        "219242 01 55 2023 1 06304:243 1158185-36",
        "219242 01 55 2023 1 06304 243:1158185-36",
        "219242 01 55 2023 1 06304 243 1158185:36",
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "AAAAAA AA AA AAAA A AAAAA AAA AAAAAAA-AA",
    ];
    for invalid_certidao in invalid_certidaos {
        assert!(!Certidao::validate(invalid_certidao));
    }
}

#[test]
fn certidao_is_bare_1_valid_bare() {
    let documents = [
        "21924201552023106304243115818536", // Valid Certidão
        "21924201552023106304243115818535", // Invalid Certidão
    ];
    for document in documents {
        assert!(Certidao::is_bare(document));
    }
}

#[test]
fn certidao_is_bare_2_invalid_bare() {
    let documents = [
        "219242 01 55 2023 1 06304 243 1158185-36", // Valid Certidão
        "219242 01 55 2023 1 06304 243 1158185-35", // Invalid Certidão
        "219242 01552023106304243115818536",        // Invalid Certidão
        "21924201 552023106304243115818536",        // Invalid Certidão
        "2192420155 2023106304243115818536",        // Invalid Certidão
        "21924201552023 106304243115818536",        // Invalid Certidão
        "219242015520231 06304243115818536",        // Invalid Certidão
        "21924201552023106304 243115818536",        // Invalid Certidão
        "21924201552023106304243 115818536",        // Invalid Certidão
        "219242015520231063042431158185-36",        // Invalid Certidão
        "2192420155202310630424311581853",          // Invalid Certidão
        "219242015520231063042431158185365",        // Invalid Certidão
    ];
    for document in documents {
        assert!(!Certidao::is_bare(document));
    }
}

#[test]
fn certidao_is_masked_1_valid_masked() {
    let documents = [
        "219242 01 55 2023 1 06304 243 1158185-36", // Valid Certidão
        "219242 01 55 2023 1 06304 243 1158185-35", // Invalid Certidão
    ];
    for document in documents {
        assert!(Certidao::is_masked(document));
    }
}

#[test]
fn certidao_is_masked_2_invalid_masked() {
    let documents = [
        "21924201552023106304243115818536", // Valid Certidão
        "21924201552023106304243115818535", // Invalid Certidão
        "219242 01552023106304243115818536", // Invalid Certidão
        "21924201 552023106304243115818536", // Invalid Certidão
        "2192420155 2023106304243115818536", // Invalid Certidão
        "21924201552023 106304243115818536", // Invalid Certidão
        "219242015520231 06304243115818536", // Invalid Certidão
        "21924201552023106304 243115818536", // Invalid Certidão
        "21924201552023106304243 115818536", // Invalid Certidão
        "219242015520231063042431158185-36", // Invalid Certidão
        "2192420155202310630424311581853",  // Invalid Certidão
        "219242015520231063042431158185365", // Invalid Certidão
        "219242 01 55 2023 1 06304 243 1158185-3", // Invalid Certidão
        "219242 01 55 2023 1 06304 243 1158185-365", // Invalid Certidão
        "219242:01 55 2023 1 06304 243 1158185-36", // Invalid Certidão
        "219242 01:55 2023 1 06304 243 1158185-36", // Invalid Certidão
        "219242 01 55:2023 1 06304 243 1158185-36", // Invalid Certidão
        "219242 01 55 2023:1 06304 243 1158185-36", // Invalid Certidão
        "219242 01 55 2023 1:06304 243 1158185-36", // Invalid Certidão
        "219242 01 55 2023 1 06304:243 1158185-36", // Invalid Certidão
        "219242 01 55 2023 1 06304 243:1158185-36", // Invalid Certidão
        "219242 01 55 2023 1 06304 243 1158185:36", // Invalid Certidão
    ];
    for document in documents {
        assert!(!Certidao::is_masked(document));
    }
}

#[test]
fn certidao_mask_1_valid_mask() {
    let valid_certidao: &str = "21924201552023106304243115818536";
    assert_eq!(
        Certidao::mask(valid_certidao),
        Ok(String::from("219242 01 55 2023 1 06304 243 1158185-36"))
    );
    let invalid_certidao: &str = "21924201552023106304243115818535";
    assert_eq!(
        Certidao::mask(invalid_certidao),
        Ok(String::from("219242 01 55 2023 1 06304 243 1158185-35"))
    );
}

#[test]
fn certidao_mask_2_invalid_mask() {
    let documents = [
        "219242 01 55 2023 1 06304 243 1158185-36", // Valid Certidão
        "219242 01 55 2023 1 06304 243 1158185-35", // Invalid Certidão
        "2192420155202310630424311581853",          // Invalid Certidão
        "219242015520231063042431158185365",        // Invalid Certidão
    ];
    for document in documents {
        let result = Certidao::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Certidão!"),
        );
    }
}

#[test]
fn certidao_generate_1() {
    for _ in 0..1000 {
        let certidao = Certidao::generate();
        assert!(Certidao::validate(&certidao));
        assert!(Certidao::is_bare(&certidao));
    }
}

#[test]
fn certidao_generate_masked_1() {
    for _ in 0..1000 {
        let certidao = Certidao::generate_masked();
        assert!(Certidao::validate(&certidao));
        assert!(Certidao::is_masked(&certidao));
    }
}

#[test]
fn certidao_document_1_constants() {
    assert_eq!(Certidao::NAME, "Certidão");
    assert_eq!(Certidao::SIZE, 32);
}
