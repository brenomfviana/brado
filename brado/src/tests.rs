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

#[cfg(feature = "cnpj")]
#[test]
fn brado_cnpj_reexport_1_validate() {
    assert!(crate::cnpj::validate("05200851000100"));
    assert!(crate::cnpj::validate("05.200.851/0001-00"));
    assert!(!crate::cnpj::validate("05.200.851/0001-01"));
}

#[cfg(feature = "cnpj")]
#[test]
fn brado_cnpj_reexport_2_document() {
    use brado_cnpj::Cnpj;
    use brado_core::Document;

    assert_eq!(Cnpj::NAME, "CNPJ");
    assert_eq!(Cnpj::SIZE, 14);
    assert!(Cnpj::validate("05.200.851/0001-00"));
}

#[cfg(feature = "cnpj")]
#[test]
fn docs_is_cnpj_1_validate() {
    assert!(crate::docs::is_cnpj("05200851000100"));
    assert!(!crate::docs::is_cnpj("05200851000101"));
}

#[cfg(feature = "cnh")]
#[test]
fn brado_cnh_reexport_1_validate() {
    assert!(crate::cnh::validate("84718735264"));
    assert!(crate::cnh::validate("847 187 352 64"));
    assert!(!crate::cnh::validate("847 187 352 65"));
}

#[cfg(feature = "cnh")]
#[test]
fn brado_cnh_reexport_2_document() {
    use brado_cnh::Cnh;
    use brado_core::Document;

    assert_eq!(Cnh::NAME, "CNH");
    assert_eq!(Cnh::SIZE, 11);
    assert!(Cnh::validate("847 187 352 64"));
}

#[cfg(feature = "cnh")]
#[test]
fn docs_is_cnh_1_validate() {
    assert!(crate::docs::is_cnh("84718735264"));
    assert!(!crate::docs::is_cnh("84718735265"));
}

#[cfg(feature = "cns")]
#[test]
fn brado_cns_reexport_1_validate() {
    assert!(crate::cns::validate("144082627260004"));
    assert!(crate::cns::validate("144 0826 2726 0004"));
    assert!(!crate::cns::validate("144 0826 2726 0005"));
}

#[cfg(feature = "cns")]
#[test]
fn brado_cns_reexport_2_document() {
    use brado_cns::Cns;
    use brado_core::Document;

    assert_eq!(Cns::NAME, "CNS");
    assert_eq!(Cns::SIZE, 15);
    assert!(Cns::validate("144 0826 2726 0004"));
}

#[cfg(feature = "cns")]
#[test]
fn docs_is_cns_1_validate() {
    assert!(crate::docs::is_cns("144082627260004"));
    assert!(!crate::docs::is_cns("144082627260005"));
}
