use brado::Brado;

#[derive(Brado)]
struct Documentos {
    #[brado(document = "cpf")]
    cpf: String,
}

fn main() {
    let docs = Documentos {
        cpf: "639.292.470-11".to_string(),
    };

    println!("válido: {}", docs.validate());
}
