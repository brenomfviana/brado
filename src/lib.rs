pub mod docs;

#[cfg(test)]
mod cpf_tests {
    use super::*;
    use crate::docs::doc::Document;

    #[test]
    fn validate_cpf_1() {
        let doc: Document = Document::new("11111111111");
        assert_eq!(false, docs::cpf::validate(doc, false));
    }

    #[test]
    fn validate_cpf_2() {
        let doc: Document = Document::new("111.111.111-11");
        assert_eq!(false, docs::cpf::validate(doc, false));
    }

    #[test]
    fn validate_cpf_3() {
        let doc: Document = Document::new("63929247011");
        assert_eq!(true, docs::cpf::validate(doc, true));
    }

    #[test]
    fn validate_cpf_4() {
        let doc: Document = Document::new("639.292.470-11");
        assert_eq!(true, docs::cpf::validate(doc, false));
    }
}

#[cfg(test)]
mod cnpj_tests {
    use super::*;
    use crate::docs::doc::Document;

    #[test]
    fn validate_cnpj_1() {
        let doc: Document = Document::new("05200851000100");
        assert_eq!(true, docs::cnpj::validate(doc));
    }

    #[test]
    fn validate_cnpj_2() {
        let doc: Document = Document::new("05.200.851/0001-00");
        assert_eq!(true, docs::cnpj::validate(doc));
    }
}
