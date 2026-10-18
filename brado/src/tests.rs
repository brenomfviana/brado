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

#[cfg(feature = "nis")]
#[test]
fn brado_nis_reexport_1_validate() {
    assert!(crate::nis::validate("40865658047"));
    assert!(crate::nis::validate("408.65658.04-7"));
    assert!(!crate::nis::validate("408.65658.04-6"));
}

#[cfg(feature = "nis")]
#[test]
fn brado_nis_reexport_2_document() {
    use brado_core::Document;
    use brado_nis::Nis;

    assert_eq!(Nis::NAME, "NIS");
    assert_eq!(Nis::SIZE, 11);
    assert!(Nis::validate("408.65658.04-7"));
}

#[cfg(feature = "nis")]
#[test]
fn docs_is_nis_1_validate() {
    assert!(crate::docs::is_nis("40865658047"));
    assert!(!crate::docs::is_nis("40865658046"));
}

#[cfg(feature = "eleitoral")]
#[test]
fn brado_eleitoral_reexport_1_validate() {
    assert!(crate::eleitoral::validate("773537801651"));
    assert!(crate::eleitoral::validate("7735 3780 1651"));
    assert!(!crate::eleitoral::validate("7735 3780 1650"));
}

#[cfg(feature = "eleitoral")]
#[test]
fn brado_eleitoral_reexport_2_document() {
    use brado_core::Document;
    use brado_eleitoral::Eleitoral;

    assert_eq!(Eleitoral::NAME, "Título Eleitoral");
    assert_eq!(Eleitoral::SIZE, 12);
    assert!(Eleitoral::validate("7735 3780 1651"));
}

#[cfg(feature = "eleitoral")]
#[test]
fn docs_is_eleitoral_1_validate() {
    assert!(crate::docs::is_eleitoral("773537801651"));
    assert!(!crate::docs::is_eleitoral("773537801650"));
}

#[cfg(feature = "renavam")]
#[test]
fn brado_renavam_reexport_1_validate() {
    assert!(crate::renavam::validate("79072338363"));
    assert!(crate::renavam::validate("7907233836-3"));
    assert!(!crate::renavam::validate("7907233836-2"));
}

#[cfg(feature = "renavam")]
#[test]
fn brado_renavam_reexport_2_document() {
    use brado_core::Document;
    use brado_renavam::Renavam;

    assert_eq!(Renavam::NAME, "RENAVAM");
    assert_eq!(Renavam::SIZE, 11);
    assert!(Renavam::validate("7907233836-3"));
}

#[cfg(feature = "renavam")]
#[test]
fn docs_is_renavam_1_validate() {
    assert!(crate::docs::is_renavam("79072338363"));
    assert!(!crate::docs::is_renavam("79072338362"));
}

#[cfg(feature = "certidao")]
#[test]
fn brado_certidao_reexport_1_validate() {
    assert!(crate::certidao::validate(
        "21924201552023106304243115818536"
    ));
    assert!(crate::certidao::validate(
        "219242 01 55 2023 1 06304 243 1158185-36"
    ));
    assert!(!crate::certidao::validate(
        "219242 01 55 2023 1 06304 243 1158185-35"
    ));
}

#[cfg(feature = "certidao")]
#[test]
fn brado_certidao_reexport_2_document() {
    use brado_certidao::Certidao;
    use brado_core::Document;

    assert_eq!(Certidao::NAME, "Certidão");
    assert_eq!(Certidao::SIZE, 32);
    assert!(Certidao::validate(
        "219242 01 55 2023 1 06304 243 1158185-36"
    ));
}

#[cfg(feature = "certidao")]
#[test]
fn docs_is_certidao_1_validate() {
    assert!(crate::docs::is_certidao("21924201552023106304243115818536"));
    assert!(!crate::docs::is_certidao(
        "21924201552023106304243115818535"
    ));
}
