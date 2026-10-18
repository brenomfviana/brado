//! Testes de integração do módulo CPF.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Cpf`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Cpf;

#[test]
fn cpf_validate_1_valid_cpfs() {
    let valid_cpfs = ["63929247011", "639.292.470-11"];
    for valid_cpf in valid_cpfs {
        assert!(Cpf::validate(valid_cpf));
    }
}

#[test]
fn cpf_validate_2_invalid_cpfs() {
    let invalid_cpfs = [
        "6392924701",
        "63929247010",
        "639292470110",
        "639.292.470-1",
        "639.292.470-10",
        "639.292.470-110",
        "63.929.247-011",
        "6392.924.701-1",
        "639:292.470-11",
        "639.292:470-11",
        "639.292.470:11",
        "AAAAAAAAAAA",
        "AAA.AAA.AAA-AA",
    ];
    for invalid_cpf in invalid_cpfs {
        assert!(!Cpf::validate(invalid_cpf));
    }
}

#[test]
fn cpf_validate_3_invalid_cpfs_repeated_numbers() {
    for i in 1..=9 {
        let cpf_rep_num = (vec![i.to_string(); 11]).concat();
        assert!(!Cpf::validate(&cpf_rep_num));
    }
}

#[test]
fn cpf_is_bare_1_valid_bare() {
    let documents = [
        "63929247011", // Valid CPF
        "63929247010", // Invalid CPF
    ];
    for document in documents {
        assert!(Cpf::is_bare(document));
    }
}

#[test]
fn cpf_is_bare_2_invalid_bare() {
    let documents = [
        "639.292.470-11", // Valid CPF
        "639.292.470-10", // Invalid CPF
        "639.29247011",   // Invalid CPF
        "639292.47011",   // Invalid CPF
        "639292470-11",   // Invalid CPF
        "6392924701",     // Invalid CPF
        "639292470110",   // Invalid CPF
    ];
    for document in documents {
        assert!(!Cpf::is_bare(document));
    }
}

#[test]
fn cpf_is_masked_1_valid_masked() {
    let documents = [
        "639.292.470-11", // Valid CPF
        "639.292.470-10", // Invalid CPF
    ];
    for document in documents {
        assert!(Cpf::is_masked(document));
    }
}

#[test]
fn cpf_is_masked_2_invalid_masked() {
    let documents = [
        "63929247011",     // Valid CPF
        "63929247010",     // Invalid CPF
        "639.29247011",    // Invalid CPF
        "639292.47011",    // Invalid CPF
        "639292470-11",    // Invalid CPF
        "6392924701",      // Invalid CPF
        "639292470110",    // Invalid CPF
        "639.292.470-1",   // Invalid CPF
        "639.292.470-110", // Invalid CPF
        "63.929.247-011",  // Invalid CPF
        "6392.924.701-1",  // Invalid CPF
        "639:292.470-11",  // Invalid CPF
        "639.292:470-11",  // Invalid CPF
        "639.292.470:11",  // Invalid CPF
    ];
    for document in documents {
        assert!(!Cpf::is_masked(document));
    }
}

#[test]
fn cpf_mask_1_valid_mask() {
    let valid_cpf: &str = "63929247011";
    assert_eq!(Cpf::mask(valid_cpf), Ok(String::from("639.292.470-11")),);
    let invalid_cpf: &str = "63929247010";
    assert_eq!(Cpf::mask(invalid_cpf), Ok(String::from("639.292.470-10")),);
}

#[test]
fn cpf_mask_2_invalid_mask() {
    let documents = [
        "639.292.470-11", // Valid CPF
        "639.292.470-10", // Invalid CPF
        "6392924701",     // Invalid CPF
        "639292470110",   // Invalid CPF
    ];
    for document in documents {
        let result = Cpf::mask(document);
        assert_eq!(result, Err("The given string cannot be masked as CPF!"),);
    }
}

#[test]
fn cpf_generate_1() {
    for _ in 0..1000 {
        let cpf = Cpf::generate();
        assert!(Cpf::validate(&cpf));
        assert!(Cpf::is_bare(&cpf));
    }
}

#[test]
fn cpf_generate_masked_1() {
    for _ in 0..1000 {
        let cpf = Cpf::generate_masked();
        assert!(Cpf::validate(&cpf));
        assert!(Cpf::is_masked(&cpf));
    }
}

#[test]
fn cpf_document_1_constants() {
    assert_eq!(Cpf::NAME, "CPF");
    assert_eq!(Cpf::SIZE, 11);
}
