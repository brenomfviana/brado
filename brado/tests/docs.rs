#[cfg(test)]
mod docs_tests {
    use brado;

    /* CPF */

    #[test]
    fn docs_is_cpf_cpf() {
        let cpf: String = String::from("63929247011");
        assert_eq!(brado::docs::is_cpf(&cpf), true);
    }

    #[test]
    fn docs_is_cpf_cnpj() {
        let cnpj: String = String::from("05200851000100");
        assert_eq!(brado::docs::is_cpf(&cnpj), false);
    }

    #[test]
    fn docs_is_cpf_cnh() {
        let cnh: String = String::from("84718735264");
        assert_eq!(brado::docs::is_cpf(&cnh), false);
    }

    /* CNPJ */

    #[test]
    fn docs_is_cnpj_cnpj() {
        let cnpj: String = String::from("05200851000100");
        assert_eq!(brado::docs::is_cnpj(&cnpj), true);
    }

    #[test]
    fn docs_is_cnpj_cpf() {
        let cpf: String = String::from("63929247011");
        assert_eq!(brado::docs::is_cnpj(&cpf), false);
    }

    #[test]
    fn docs_is_cnpj_cnh() {
        let cnh: String = String::from("84718735264");
        assert_eq!(brado::docs::is_cnpj(&cnh), false);
    }

    /* CNH */

    #[test]
    fn docs_is_cnh_cnh() {
        let cnh: String = String::from("84718735264");
        assert_eq!(brado::docs::is_cnh(&cnh), true);
    }

    #[test]
    fn docs_is_cnh_cpf() {
        let cpf: String = String::from("63929247011");
        assert_eq!(brado::docs::is_cnh(&cpf), true);
    }

    #[test]
    fn docs_is_cnh_cnpj() {
        let cnpj: String = String::from("05200851000100");
        assert_eq!(brado::docs::is_cnh(&cnpj), false);
    }
}
