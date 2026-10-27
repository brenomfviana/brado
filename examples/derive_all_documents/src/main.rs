use brado::Brado;

#[derive(Brado)]
struct Documentos {
    #[brado(document = "cpf")]
    cpf: String,
    #[brado(document = "cnpj")]
    cnpj: String,
    #[brado(document = "cnh")]
    cnh: String,
    #[brado(document = "cns")]
    cns: String,
    #[brado(document = "nis")]
    nis: String,
    #[brado(document = "eleitoral")]
    eleitoral: String,
    #[brado(document = "renavam")]
    renavam: String,
    #[brado(document = "certidao")]
    certidao: String,
}

fn main() {
    let docs = Documentos {
        cpf: "639.292.470-11".to_string(),
        cnpj: "11.222.333/0001-81".to_string(),
        cnh: "84718735264".to_string(),
        cns: "144082627260004".to_string(),
        nis: "40865658047".to_string(),
        eleitoral: "773537801651".to_string(),
        renavam: "79072338363".to_string(),
        certidao: "219242 01 55 2023 1 06304 243 1158185-36".to_string(),
    };

    println!("todos válidos: {}", docs.validate());
}
