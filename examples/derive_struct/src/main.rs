use brado::Brado;

#[derive(Brado)]
struct Cadastro {
    #[brado(document = "cpf")]
    cpf: String,
    #[brado(document = "cnpj")]
    cnpj: String,
}

fn main() {
    let cadastro = Cadastro {
        cpf: "639.292.470-11".to_string(),
        cnpj: "11.222.333/0001-81".to_string(),
    };

    println!("válido: {}", cadastro.validate());
}
