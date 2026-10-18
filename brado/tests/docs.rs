#[cfg(test)]
mod docs_tests {
    /* CPF */

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_cpf() {
        let cpf: &str = "63929247011";
        assert!(brado::docs::is_cpf(cpf));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_cpf(cnpj));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_cpf(cnh));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_cpf(cns));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_cpf(nis));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_cpf(eleitoral));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_cpf(renavam));
    }

    #[cfg(feature = "cpf")]
    #[test]
    fn docs_is_cpf_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_cpf(certidao));
    }

    /* CNPJ */

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_cnpj(cpf));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(brado::docs::is_cnpj(cnpj));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_cnpj(cnh));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_cnpj(cns));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_cnpj(nis));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_cnpj(eleitoral));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_cnpj(renavam));
    }

    #[cfg(feature = "cnpj")]
    #[test]
    fn docs_is_cnpj_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_cnpj(certidao));
    }

    /* CNH */

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_cpf() {
        let cpf: &str = "63929247011";
        assert!(brado::docs::is_cnh(cpf));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_cnh(cnpj));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_cnh() {
        let cnh: &str = "84718735264";
        assert!(brado::docs::is_cnh(cnh));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_cnh(cns));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_cnh(nis));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_cnh(eleitoral));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_cnh(renavam));
    }

    #[cfg(feature = "cnh")]
    #[test]
    fn docs_is_cnh_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_cnh(certidao));
    }

    /* CNS */

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_cns(cpf));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_cns(cnpj));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_cns(cnh));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_cns() {
        let cnh: &str = "144082627260004";
        assert!(brado::docs::is_cns(cnh));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_cns(nis));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_cns(eleitoral));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_cns(renavam));
    }

    #[cfg(feature = "cns")]
    #[test]
    fn docs_is_cns_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_cns(certidao));
    }

    /* NIS/NIT/PIS/PASEP */

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_nis(cpf));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_nis(cnpj));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_nis(cnh));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_nis(cns));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_nis() {
        let nis: &str = "40865658047";
        assert!(brado::docs::is_nis(nis));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_nis(eleitoral));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_renavam() {
        let renavam: &str = "79072338363";
        assert!(brado::docs::is_nis(renavam));
    }

    #[cfg(feature = "nis")]
    #[test]
    fn docs_is_nis_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_nis(certidao));
    }

    /* Título Eleitoral */

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_eleitoral(cpf));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_eleitoral(cnpj));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_eleitoral(cnh));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_eleitoral(cns));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_eleitoral(nis));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(brado::docs::is_eleitoral(eleitoral));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_eleitoral(renavam));
    }

    #[cfg(feature = "eleitoral")]
    #[test]
    fn docs_is_eleitoral_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_eleitoral(certidao));
    }

    /* RENAVAM */

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_renavam(cpf));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_renavam(cnpj));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_renavam(cnh));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_renavam(cns));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_nis() {
        let nis: &str = "40865658047";
        assert!(brado::docs::is_renavam(nis));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_renavam(eleitoral));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_renavam() {
        let renavam: &str = "79072338363";
        assert!(brado::docs::is_renavam(renavam));
    }

    #[cfg(feature = "renavam")]
    #[test]
    fn docs_is_renavam_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(!brado::docs::is_renavam(certidao));
    }

    /* Certidão */

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_cpf() {
        let cpf: &str = "63929247011";
        assert!(!brado::docs::is_certidao(cpf));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_cnpj() {
        let cnpj: &str = "05200851000100";
        assert!(!brado::docs::is_certidao(cnpj));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_cnh() {
        let cnh: &str = "84718735264";
        assert!(!brado::docs::is_certidao(cnh));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_cns() {
        let cns: &str = "144082627260004";
        assert!(!brado::docs::is_certidao(cns));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_nis() {
        let nis: &str = "40865658047";
        assert!(!brado::docs::is_certidao(nis));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert!(!brado::docs::is_certidao(eleitoral));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_renavam() {
        let renavam: &str = "79072338363";
        assert!(!brado::docs::is_certidao(renavam));
    }

    #[cfg(feature = "certidao")]
    #[test]
    fn docs_is_certidao_certidao() {
        let certidao: &str = "21924201552023106304243115818536";
        assert!(brado::docs::is_certidao(certidao));
    }
}
