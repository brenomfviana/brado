#[cfg(test)]
mod docs_tests {
    use brado;

    /* CPF */

    #[test]
    fn docs_is_cpf_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_cpf(cpf), true);
    }

    #[test]
    fn docs_is_cpf_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_cpf(cnpj), false);
    }

    #[test]
    fn docs_is_cpf_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cpf(cnh), false);
    }

    #[test]
    fn docs_is_cpf_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_cpf(cns), false);
    }

    #[test]
    fn docs_is_cpf_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cpf(nis), false);
    }

    #[test]
    fn docs_is_cpf_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_cpf(eleitoral), false);
    }

    #[test]
    fn docs_is_cpf_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_cpf(renavam), false);
    }

    /* CNPJ */

    #[test]
    fn docs_is_cnpj_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_cnpj(cpf), false);
    }

    #[test]
    fn docs_is_cnpj_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_cnpj(cnpj), true);
    }

    #[test]
    fn docs_is_cnpj_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cnpj(cnh), false);
    }

    #[test]
    fn docs_is_cnpj_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_cnpj(cns), false);
    }

    #[test]
    fn docs_is_cnpj_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cnpj(nis), false);
    }

    #[test]
    fn docs_is_cnpj_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_cnpj(eleitoral), false);
    }

    #[test]
    fn docs_is_cnpj_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_cnpj(renavam), false);
    }

    /* CNH */

    #[test]
    fn docs_is_cnh_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_cnh(cpf), true);
    }

    #[test]
    fn docs_is_cnh_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_cnh(cnpj), false);
    }

    #[test]
    fn docs_is_cnh_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cnh(cnh), true);
    }

    #[test]
    fn docs_is_cnh_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_cnh(cns), false);
    }

    #[test]
    fn docs_is_cnh_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cnh(nis), false);
    }

    #[test]
    fn docs_is_cnh_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_cnh(eleitoral), false);
    }

    #[test]
    fn docs_is_cnh_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_cnh(renavam), false);
    }

    /* CNS */

    #[test]
    fn docs_is_cns_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_cns(cpf), false);
    }

    #[test]
    fn docs_is_cns_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_cns(cnpj), false);
    }

    #[test]
    fn docs_is_cns_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cns(cnh), false);
    }

    #[test]
    fn docs_is_cns_cns() {
        let cnh: &str = "144082627260004";
        assert_eq!(brado::docs::is_cns(cnh), true);
    }

    #[test]
    fn docs_is_cns_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cns(nis), false);
    }

    #[test]
    fn docs_is_cns_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_cns(eleitoral), false);
    }

    #[test]
    fn docs_is_cns_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_cns(renavam), false);
    }

    /* NIS/NIT/PIS/PASEP */

    #[test]
    fn docs_is_nis_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_nis(cpf), false);
    }

    #[test]
    fn docs_is_nis_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_nis(cnpj), false);
    }

    #[test]
    fn docs_is_nis_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_nis(cnh), false);
    }

    #[test]
    fn docs_is_nis_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_nis(cns), false);
    }

    #[test]
    fn docs_is_nis_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_nis(nis), true);
    }

    #[test]
    fn docs_is_nis_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_nis(eleitoral), false);
    }

    #[test]
    fn docs_is_nis_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_nis(renavam), true);
    }

    /* Título Eleitoral */

    #[test]
    fn docs_is_eleitoral_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_eleitoral(cpf), false);
    }

    #[test]
    fn docs_is_eleitoral_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_eleitoral(cnpj), false);
    }

    #[test]
    fn docs_is_eleitoral_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_eleitoral(cnh), false);
    }

    #[test]
    fn docs_is_eleitoral_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_eleitoral(cns), false);
    }

    #[test]
    fn docs_is_eleitoral_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_eleitoral(nis), false);
    }

    #[test]
    fn docs_is_eleitoral_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_eleitoral(eleitoral), true);
    }

    #[test]
    fn docs_is_eleitoral_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_eleitoral(renavam), false);
    }

    /* RENAVAM */

    #[test]
    fn docs_is_renavam_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_renavam(cpf), false);
    }

    #[test]
    fn docs_is_renavam_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_renavam(cnpj), false);
    }

    #[test]
    fn docs_is_renavam_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_renavam(cnh), false);
    }

    #[test]
    fn docs_is_renavam_cns() {
        let cns: &str = "144082627260004";
        assert_eq!(brado::docs::is_renavam(cns), false);
    }

    #[test]
    fn docs_is_renavam_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_renavam(nis), true);
    }

    #[test]
    fn docs_is_renavam_eleitoral() {
        let eleitoral: &str = "773537801651";
        assert_eq!(brado::docs::is_renavam(eleitoral), false);
    }

    #[test]
    fn docs_is_renavam_renavam() {
        let renavam: &str = "79072338363";
        assert_eq!(brado::docs::is_renavam(renavam), true);
    }
}
