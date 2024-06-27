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
    fn docs_is_cpf_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cpf(nis), false);
    }

    /* CNPJ */

    #[test]
    fn docs_is_cnpj_cnpj() {
        let cnpj: &str = "05200851000100";
        assert_eq!(brado::docs::is_cnpj(cnpj), true);
    }

    #[test]
    fn docs_is_cnpj_cpf() {
        let cpf: &str = "63929247011";
        assert_eq!(brado::docs::is_cnpj(cpf), false);
    }

    #[test]
    fn docs_is_cnpj_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cnpj(cnh), false);
    }

    #[test]
    fn docs_is_cnpj_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cnpj(nis), false);
    }

    /* CNH */

    #[test]
    fn docs_is_cnh_cnh() {
        let cnh: &str = "84718735264";
        assert_eq!(brado::docs::is_cnh(cnh), true);
    }

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
    fn docs_is_cnh_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cnh(nis), false);
    }

    /* CNS */

    #[test]
    fn docs_is_cns_cns() {
        let cnh: &str = "144082627260004";
        assert_eq!(brado::docs::is_cns(cnh), true);
    }

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
    fn docs_is_cns_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_cns(nis), false);
    }

    /* NIS/NIT/PIS/PASEP */

    #[test]
    fn docs_is_nis_nis() {
        let nis: &str = "40865658047";
        assert_eq!(brado::docs::is_nis(nis), true);
    }

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
}
