//! Testes de integração do módulo Eleitoral.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Eleitoral`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Eleitoral;

#[test]
fn eleitoral_validate_1_valid_eleitorais() {
    let valid_eleitorais = ["773537801651", "7735 3780 1651"];
    for valid_eleitoral in valid_eleitorais {
        assert!(Eleitoral::validate(valid_eleitoral));
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
        assert!(!Eleitoral::validate(invalid_eleitoral));
    }
}

#[test]
fn eleitoral_is_bare_1_valid_bare() {
    let documents = [
        "773537801651", // Valid Título Eleitoral
        "773537801650", // Invalid Título Eleitoral
    ];
    for document in documents {
        assert!(Eleitoral::is_bare(document));
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
        assert!(!Eleitoral::is_bare(document));
    }
}

#[test]
fn eleitoral_is_masked_1_valid_masked() {
    let documents = [
        "7735 3780 1651", // Valid Título Eleitoral
        "7735 3780 1650", // Invalid Título Eleitoral
    ];
    for document in documents {
        assert!(Eleitoral::is_masked(document));
    }
}

#[test]
fn eleitoral_is_masked_2_invalid_masked() {
    let documents = [
        "773537801651",    // Valid Título Eleitoral
        "773537801650",    // Invalid Título Eleitoral
        "7735 37801651",   // Invalid Título Eleitoral
        "77353780 1651",   // Invalid Título Eleitoral
        "77353780165",     // Invalid Título Eleitoral
        "7735378016510",   // Invalid Título Eleitoral
        "7735 3780 165",   // Invalid Título Eleitoral
        "7735 3780 16510", // Invalid Título Eleitoral
        "773 5378 01651",  // Invalid Título Eleitoral
        "77353 7801 651",  // Invalid Título Eleitoral
        "7735:37801651",   // Invalid Título Eleitoral
        "77353780:1651",   // Invalid Título Eleitoral
    ];
    for document in documents {
        assert!(!Eleitoral::is_masked(document));
    }
}

#[test]
fn eleitoral_mask_1_valid_mask() {
    let valid_eleitoral: &str = "773537801651";
    assert_eq!(
        Eleitoral::mask(valid_eleitoral),
        Ok(String::from("7735 3780 1651"))
    );
    let invalid_eleitoral: &str = "773537801650";
    assert_eq!(
        Eleitoral::mask(invalid_eleitoral),
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
        let result = Eleitoral::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as Título Eleitoral!"),
        );
    }
}

#[test]
fn eleitoral_generate_1() {
    for _ in 0..1000 {
        let eleitoral = Eleitoral::generate();
        assert!(Eleitoral::validate(&eleitoral));
        assert!(Eleitoral::is_bare(&eleitoral));
    }
}

#[test]
fn eleitoral_generate_masked_1() {
    for _ in 0..1000 {
        let eleitoral = Eleitoral::generate_masked();
        assert!(Eleitoral::validate(&eleitoral));
        assert!(Eleitoral::is_masked(&eleitoral));
    }
}

#[test]
fn eleitoral_document_1_constants() {
    assert_eq!(Eleitoral::NAME, "Título Eleitoral");
    assert_eq!(Eleitoral::SIZE, 12);
}
