//! Testes de fumaça dos re-exports da fachada brado.

#[cfg(feature = "cpf")]
#[test]
fn brado_cpf_reexport_1_validate() {
    assert!(crate::cpf::validate("63929247011"));
    assert!(crate::cpf::validate("639.292.470-11"));
    assert!(!crate::cpf::validate("639.292.470-10"));
}

#[cfg(feature = "cpf")]
#[test]
fn brado_cpf_reexport_2_document() {
    use brado_core::Document;
    use brado_cpf::Cpf;

    assert_eq!(Cpf::NAME, "CPF");
    assert_eq!(Cpf::SIZE, 11);
    assert!(Cpf::validate("639.292.470-11"));
}

#[cfg(feature = "cpf")]
#[test]
fn docs_is_cpf_1_validate() {
    assert!(crate::docs::is_cpf("63929247011"));
    assert!(!crate::docs::is_cpf("63929247010"));
}
