//! Testes de integração do módulo CNPJ.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Cnpj`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Cnpj;

#[test]
fn cnpj_validate_1_valid_cnpjs() {
    let valid_cnpjs = [
        "05200851000100",
        "05.200.851/0001-00",
        "A5200851000123",
        "A5.200.851/0001-23",
        "AAAAAAAAAAAA45",
        "AA.AAA.AAA/AAAA-45",
    ];
    for valid_cnpj in valid_cnpjs {
        assert!(Cnpj::validate(valid_cnpj));
    }
}

#[test]
fn cnpj_validate_2_invalid_cnpjs() {
    let invalid_cnpjs = [
        "0520085100010",
        "05200851000101",
        "052008510001001",
        "052008510001A1",
        "0520085100010A",
        "05.200.851/0001-0-0",
        "05.200.851/0001-0",
        "05.200.851/0001-01",
        "05.200.851/0001-001",
        "0.520.085/1000-100",
        "052.008.510/0010-0",
        "05:200.851/0001-00",
        "05.200:851/0001-00",
        "05.200.851:0001-00",
        "05.200.851/0001:00",
        "AAAAAAAAAAAAA",
        "AA.AAA.AAA/AAAA-AA",
    ];
    for invalid_cnpj in invalid_cnpjs {
        assert!(!Cnpj::validate(invalid_cnpj));
    }
}

#[test]
fn cnpj_validate_3_invalid_cnpjs_repeated_numbers() {
    for i in 0..=9 {
        let cnpj_rep_num = (vec![i.to_string(); 14]).concat();
        assert!(!Cnpj::validate(&cnpj_rep_num));
    }
}

#[test]
fn cnpj_is_bare_1_valid_bare() {
    let documents = [
        "05200851000100", // Valid CNPJ
        "A5200851000100", // Valid CNPJ
        "05200851000101", // Invalid CNPJ
    ];
    for document in documents {
        assert!(Cnpj::is_bare(document));
    }
}

#[test]
fn cnpj_is_bare_2_invalid_bare() {
    let documents = [
        "05.200.851/0001-00", // Valid CNPJ
        "05.200.851/0001-01", // Invalid CNPJ
        "05.200851000101",    // Invalid CNPJ
        "05200.851000101",    // Invalid CNPJ
        "05200851/000101",    // Invalid CNPJ
        "052008510001-01",    // Invalid CNPJ
        "0520085100010",      // Invalid CNPJ
        "052008510001001",    // Invalid CNPJ
    ];
    for document in documents {
        assert!(!Cnpj::is_bare(document));
    }
}

#[test]
fn cnpj_is_masked_1_valid_masked() {
    let documents = [
        "05.200.851/0001-00", // Valid CNPJ
        "05.200.851/0001-01", // Invalid CNPJ
    ];
    for document in documents {
        assert!(Cnpj::is_masked(document));
    }
}

#[test]
fn cnpj_is_masked_2_invalid_masked() {
    let documents = [
        "05200851000100",      // Valid CNPJ
        "05200851000101",      // Invalid CNPJ
        "05.200851000101",     // Invalid CNPJ
        "05200.851000101",     // Invalid CNPJ
        "05200851/000101",     // Invalid CNPJ
        "052008510001-01",     // Invalid CNPJ
        "0520085100010",       // Invalid CNPJ
        "052008510001001",     // Invalid CNPJ
        "05.200.851/0001-0",   // Invalid CNPJ
        "05.200.851/0001-001", // Invalid CNPJ
        "0.520.085/1000-100",  // Invalid CNPJ
        "052.008.510/0010-0",  // Invalid CNPJ
        "05:200.851/0001-00",  // Invalid CNPJ
        "05.200:851/0001-00",  // Invalid CNPJ
        "05.200.851:0001-00",  // Invalid CNPJ
        "05.200.851/0001:00",  // Invalid CNPJ
    ];
    for document in documents {
        assert!(!Cnpj::is_masked(document));
    }
}

#[test]
fn cnpj_mask_1_valid_mask() {
    let valid_cnpj: &str = "05200851000100";
    assert_eq!(
        Cnpj::mask(valid_cnpj),
        Ok(String::from("05.200.851/0001-00"))
    );
    let invalid_cnpj: &str = "05200851000101";
    assert_eq!(
        Cnpj::mask(invalid_cnpj),
        Ok(String::from("05.200.851/0001-01"))
    );
}

#[test]
fn cnpj_mask_2_invalid_mask() {
    let documents = [
        "05.200.851/0001-00", // Valid CNPJ
        "05.200.851/0001-01", // Invalid CNPJ
        "0520085100010",      // Invalid CNPJ
        "052008510001001",    // Invalid CNPJ
    ];
    for document in documents {
        let result = Cnpj::mask(document);
        assert_eq!(result, Err("The given string cannot be masked as CNPJ!"),);
    }
}

#[test]
fn cnpj_generate_1() {
    for _ in 0..1000 {
        let cnpj = Cnpj::generate();
        assert!(Cnpj::validate(&cnpj));
        assert!(Cnpj::is_bare(&cnpj));
    }
}

#[test]
fn cnpj_generate_masked_1() {
    for _ in 0..1000 {
        let cnpj = Cnpj::generate_masked();
        assert!(Cnpj::validate(&cnpj));
        assert!(Cnpj::is_masked(&cnpj));
    }
}

#[test]
fn cnpj_document_1_constants() {
    assert_eq!(Cnpj::NAME, "CNPJ");
    assert_eq!(Cnpj::SIZE, 14);
}
