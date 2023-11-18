pub mod docs;

#[cfg(test)]
mod cpf_tests {
    use super::*;
    use crate::docs::doc::Document;

    #[test]
    fn validate_cpf_1() {
        let cpf_doc: Document = Document::new("11111111111");
        assert_eq!(false, docs::cpf::validate(&cpf_doc, false, false));
    }

    #[test]
    fn validate_cpf_2() {
        let cpf_doc: Document = Document::new("111.111.111-11");
        assert_eq!(false, docs::cpf::validate(&cpf_doc, true, false));
    }

    #[test]
    fn validate_cpf_3() {
        let cpf_doc: Document = Document::new("63929247011");
        assert_eq!(true, docs::cpf::validate(&cpf_doc, false, true));
    }

    #[test]
    fn validate_cpf_4() {
        let cpf_doc: Document = Document::new("639.292.470-11");
        assert_eq!(true, docs::cpf::validate(&cpf_doc, true, false));
    }

    #[test]
    fn validate_str_cpf_1() {
        let bare_cpf = "11111111111";
        assert_eq!(false, docs::cpf::validate_str(bare_cpf, false, false));
    }

    #[test]
    fn validate_str_cpf_2() {
        let bare_cpf = "111.111.111-11";
        assert_eq!(false, docs::cpf::validate_str(bare_cpf, true, false));
    }

    #[test]
    fn validate_str_cpf_3() {
        let bare_cpf = "63929247011";
        assert_eq!(true, docs::cpf::validate_str(bare_cpf, false, true));
    }

    #[test]
    fn validate_str_cpf_4() {
        let bare_cpf = "639.292.470-11";
        assert_eq!(true, docs::cpf::validate_str(&bare_cpf, true, false));
    }
}

#[cfg(test)]
mod cnpj_tests {
    use super::*;
    use crate::docs::doc::Document;

    #[test]
    fn validate_cnpj_1() {
        let cnpj_doc: Document = Document::new("05200851000100");
        assert_eq!(true, docs::cnpj::validate(&cnpj_doc, false));
    }

    #[test]
    fn validate_cnpj_2() {
        let cnpj_doc: Document = Document::new("05.200.851/0001-00");
        assert_eq!(true, docs::cnpj::validate(&cnpj_doc, true));
    }

    #[test]
    fn validate_str_cnpj_1() {
        let bare_cnpj = "05200851000100";
        assert_eq!(true, docs::cnpj::validate_str(&bare_cnpj, false));
    }

    #[test]
    fn validate_str_cnpj_2() {
        let bare_cnpj = "05.200.851/0001-00";
        assert_eq!(true, docs::cnpj::validate_str(&bare_cnpj, true));
    }
}
