use crate::cnh;
use crate::cnpj;
use crate::cpf;

pub fn is_cpf(document: &str) -> bool {
    cpf::validate(document)
}

pub fn is_cnpj(document: &str) -> bool {
    cnpj::validate(document)
}

pub fn is_cnh(document: &str) -> bool {
    cnh::validate(document)
}
