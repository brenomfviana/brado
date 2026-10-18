//! Testes de integração do módulo NIS.
//!
//! Todos os testes exercitam as funções do trait [`Document`]
//! implementadas por [`Nis`]. As funções livres do módulo `api` são
//! cobertas pelos doctests.

use brado_core::Document;

use crate::Nis;

#[test]
fn nis_validate_1_valid_niss() {
    let valid_niss = ["40865658047", "408.65658.04-7"];
    for valid_nis in valid_niss {
        assert!(Nis::validate(valid_nis));
    }
}

#[test]
fn nis_validate_2_invalid_niss() {
    let invalid_niss = [
        "4086565804",
        "40865658046",
        "408656580476",
        "408.65658.04-",
        "408.65658.04-6",
        "408.65658.04-76",
        "40.86565.80-47",
        "4086.56580.47-",
        "408:65658.04-7",
        "408.65658:04-7",
        "408.65658.04:7",
        "AAAAAAAAAAA",
        "AAA.AAAAA.AA-A",
    ];
    for invalid_nis in invalid_niss {
        assert!(!Nis::validate(invalid_nis));
    }
}

#[test]
fn nis_validate_3_invalid_niss_repeated_numbers() {
    for i in 1..=9 {
        let document: String = (vec![i.to_string(); 11]).concat();
        assert!(!Nis::validate(&document));
    }
}

#[test]
fn nis_is_bare_1_valid_bare() {
    let documents = [
        "40865658047", // Valid NIS/NIT/PIS/PASEP
        "40865658046", // Invalid NIS/NIT/PIS/PASEP
    ];
    for document in documents {
        assert!(Nis::is_bare(document));
    }
}

#[test]
fn nis_is_bare_2_invalid_bare() {
    let documents = [
        "408.65658.04-7", // Valid NIS/NIT/PIS/PASEP
        "408.65658.04-6", // Invalid NIS/NIT/PIS/PASEP
        "408.65658047",   // Invalid NIS/NIT/PIS/PASEP
        "40865658.047",   // Invalid NIS/NIT/PIS/PASEP
        "4086565804-7",   // Invalid NIS/NIT/PIS/PASEP
        "4086565804",     // Invalid NIS/NIT/PIS/PASEP
        "408656580476",   // Invalid NIS/NIT/PIS/PASEP
    ];
    for document in documents {
        assert!(!Nis::is_bare(document));
    }
}

#[test]
fn nis_is_masked_1_valid_masked() {
    let documents = [
        "408.65658.04-7", // Valid NIS/NIT/PIS/PASEP
        "408.65658.04-6", // Invalid NIS/NIT/PIS/PASEP
    ];
    for document in documents {
        assert!(Nis::is_masked(document));
    }
}

#[test]
fn nis_is_masked_2_invalid_masked() {
    let documents = [
        "40865658047",     // Valid NIS/NIT/PIS/PASEP
        "40865658046",     // Invalid NIS/NIT/PIS/PASEP
        "408.65658047",    // Invalid NIS/NIT/PIS/PASEP
        "40865658.047",    // Invalid NIS/NIT/PIS/PASEP
        "4086565804-7",    // Invalid NIS/NIT/PIS/PASEP
        "4086565804",      // Invalid NIS/NIT/PIS/PASEP
        "408656580476",    // Invalid NIS/NIT/PIS/PASEP
        "408.65658.04-",   // Invalid NIS/NIT/PIS/PASEP
        "408.65658.04-76", // Invalid NIS/NIT/PIS/PASEP
        "40.86565.80-47",  // Invalid NIS/NIT/PIS/PASEP
        "4086.56580.47-",  // Invalid NIS/NIT/PIS/PASEP
        "408:65658.04-7",  // Invalid NIS/NIT/PIS/PASEP
        "408.65658:04-7",  // Invalid NIS/NIT/PIS/PASEP
        "408.65658.04:7",  // Invalid NIS/NIT/PIS/PASEP
    ];
    for document in documents {
        assert!(!Nis::is_masked(document));
    }
}

#[test]
fn nis_mask_1_valid_mask() {
    let valid_nis: &str = "40865658047";
    assert_eq!(Nis::mask(valid_nis), Ok(String::from("408.65658.04-7")));
    let invalid_nis: &str = "40865658046";
    assert_eq!(Nis::mask(invalid_nis), Ok(String::from("408.65658.04-6")));
}

#[test]
fn nis_mask_2_invalid_mask() {
    let documents = [
        "408.65658.04-7", // Valid NIS/NIT/PIS/PASEP
        "408.65658.04-6", // Invalid NIS/NIT/PIS/PASEP
        "4086565804",     // Invalid NIS/NIT/PIS/PASEP
        "408656580476",   // Invalid NIS/NIT/PIS/PASEP
    ];
    for document in documents {
        let result = Nis::mask(document);
        assert_eq!(
            result,
            Err("The given string cannot be masked as NIS/NIT/PIS/PASEP!"),
        );
    }
}

#[test]
fn nis_generate_1() {
    for _ in 0..1000 {
        let nis = Nis::generate();
        assert!(Nis::validate(&nis));
        assert!(Nis::is_bare(&nis));
    }
}

#[test]
fn nis_generate_masked_1() {
    for _ in 0..1000 {
        let nis = Nis::generate_masked();
        assert!(Nis::validate(&nis));
        assert!(Nis::is_masked(&nis));
    }
}

#[test]
fn nis_document_1_constants() {
    assert_eq!(Nis::NAME, "NIS");
    assert_eq!(Nis::SIZE, 11);
}
