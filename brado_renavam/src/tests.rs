//! Testes de integração do módulo RENAVAM.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Renavam`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Renavam;

#[test]
fn renavam_validate_1_valid_renavams() {
    let valid_renavams = ["79072338363", "7907233836-3"];
    for valid_renavam in valid_renavams {
        assert!(Renavam::validate(valid_renavam));
    }
}

#[test]
fn renavam_validate_2_invalid_renavams() {
    let invalid_renavams = [
        "7907233836",
        "79072338362",
        "790723383632",
        "7907233836-",
        "7907233836-2",
        "7907233836-32",
        "790723383-63",
        "79072338363-",
        "7907233836:3",
        "AAAAAAAAAAA",
        "AAAAAAAAAA-A",
    ];
    for invalid_renavam in invalid_renavams {
        assert!(!Renavam::validate(invalid_renavam));
    }
}

#[test]
fn renavam_is_bare_1_valid_bare() {
    let documents = [
        "79072338363", // Valid RENAVAM
        "79072338362", // Invalid RENAVAM
    ];
    for document in documents {
        assert!(Renavam::is_bare(document));
    }
}

#[test]
fn renavam_is_bare_2_invalid_bare() {
    let documents = [
        "7907233836-3", // Valid RENAVAM
        "7907233836-2", // Invalid RENAVAM
        "7907233836",   // Invalid RENAVAM
        "790723383632", // Invalid RENAVAM
    ];
    for document in documents {
        assert!(!Renavam::is_bare(document));
    }
}

#[test]
fn renavam_is_masked_1_valid_masked() {
    let documents = [
        "7907233836-3", // Valid RENAVAM
        "7907233836-2", // Invalid RENAVAM
    ];
    for document in documents {
        assert!(Renavam::is_masked(document));
    }
}

#[test]
fn renavam_is_masked_2_invalid_masked() {
    let documents = [
        "79072338363",   // Valid RENAVAM
        "79072338362",   // Invalid RENAVAM
        "7907233836",    // Invalid RENAVAM
        "790723383632",  // Invalid RENAVAM
        "7907233836-",   // Invalid RENAVAM
        "7907233836-32", // Invalid RENAVAM
        "790723383-63",  // Invalid RENAVAM
        "79072338363-",  // Invalid RENAVAM
        "7907233836:3",  // Invalid RENAVAM
    ];
    for document in documents {
        assert!(!Renavam::is_masked(document));
    }
}

#[test]
fn renavam_mask_1_valid_mask() {
    let valid_renavam: &str = "79072338363";
    assert_eq!(
        Renavam::mask(valid_renavam),
        Ok(String::from("7907233836-3"))
    );
    let invalid_renavam: &str = "79072338362";
    assert_eq!(
        Renavam::mask(invalid_renavam),
        Ok(String::from("7907233836-2"))
    );
}

#[test]
fn renavam_mask_2_invalid_mask() {
    let documents = [
        "7907233836-3", // Valid RENAVAM
        "7907233836-2", // Invalid RENAVAM
        "7907233836",   // Invalid RENAVAM
        "790723383632", // Invalid RENAVAM
    ];
    for document in documents {
        let result = Renavam::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as RENAVAM!"),
        );
    }
}

#[test]
fn renavam_generate_1() {
    for _ in 0..1000 {
        let renavam = Renavam::generate();
        assert!(Renavam::validate(&renavam));
        assert!(Renavam::is_bare(&renavam));
    }
}

#[test]
fn renavam_generate_masked_1() {
    for _ in 0..1000 {
        let renavam = Renavam::generate_masked();
        assert!(Renavam::validate(&renavam));
        assert!(Renavam::is_masked(&renavam));
    }
}

#[test]
fn renavam_document_1_constants() {
    assert_eq!(Renavam::NAME, "RENAVAM");
    assert_eq!(Renavam::SIZE, 11);
}
