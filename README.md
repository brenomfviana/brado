# brado
<a href="https://crates.io/crates/brado">
  <img src="https://img.shields.io/crates/v/brado.svg" alt="latest release" />
</a>
<a href="https://crates.io/crates/brado">
  <img src="https://img.shields.io/crates/d/brado" alt="latest release" />
</a>
</a>
<a href="https://github.com/brenomfviana/brado/issues">
  <img src="https://img.shields.io/github/issues/brenomfviana/brado" alt="latest release" />
</a>


Brado é um pacote Rust para validação de documentos brasileiros. Este projeto é inspirado na biblioteca Python [validate-docbr](https://github.com/alvarofpp/validate-docbr).

Brado fornece funções para identificação, validação e geração de documentos brasileiros. O nome desta biblioteca (Brado) é um acrônimo de BRAzilian DOcs validator (validador de DOcumentos BRAsileiros).

> :warning: A documentação desta biblioteca pode ser acessada [aqui](https://docs.rs/brado/).


## Guia Rápido

Para adicionar o pacote ao projeto, basta rodar o seguinte comando:

```bash
cargo add brado
```

Ou adicionar a linha a seguir no arquivo `Cargo.toml`:

```toml
brado = "0.5.0"
```


## Documentos

- [x] CPF: Cadastro de Pessoa Física;
- [x] CNH: Carteira Nacional de Habilitação;
- [x] CNPJ: Cadastro Nacional da Pessoa Jurídica;
- [x] CNS: Cartão Nacional de Saúde;
- [x] NIS: NIS/NIT/PIS/PASEP;
- [ ] Título eleitoral: Cadastro que permite cidadãos brasileiros votar;
- [ ] RENAVAM: Registro Nacional de Veículos Automotores;
- [ ] Certidão de Nascimento/Casamento/Óbito.


## Funções

Todos os documentos possuem as mesmas funções e funcionam da mesma forma.

### validate

Valida o documento passado como parâmetro (`&str`). Retorna um valor booleano (`bool`), `true` caso o documento seja válido, ou `false` caso contrário.

```rust
use brado::cpf;

cpf::validate("63929247011"); // true
cpf::validate("639.292.470-11"); // true

cpf::validate("63929247010"); // false
cpf::validate("639.292.470-10"); // false
```

### mask

Mascara o documento passado como parâmetro (`&str`), apenas se não possuir símbolos e tiver o número de caracteres do documento sem símbolos. Retorna uma string (`Result<String, &'static str>`) correspondente ao documento mascarado ou um erro.

```rust
use brado::cpf;

cpf::mask("63929247011"); // Ok("639.292.470-11")

cpf::mask("639.292.470-11"); // Err("The given string cannot be masked as CPF!")
cpf::mask("639292470"); // Err("The given string cannot be masked as CPF!")
```

### is_bare

Verifica se o documento passado como parâmetro (`&str`) não possui símbolos. Retorna um valor booleano (`bool`), `true` caso o documento não possua símbolos, ou `false` caso contrário.

```rust
use brado::cpf;

cpf::is_bare("63929247011"); // true (CPF válido sem máscara)
cpf::is_bare("63929247010"); // true (CPF inválido sem máscara)

cpf::is_bare("639.292.470-11"); // false (CPF válido com máscara)
cpf::is_bare("639.29247011"); // false (CPF válido mascarado errado)
cpf::is_bare("639292470110"); // false (CPF inválido sem máscara)
```

> OBS: se for utilizada a função `cpf::is_bare` para verificar se um CNPJ não possui símbolos, o resultado será `false`! Isso acontece pois esta função considera que a string é um CPF, ou seja, possui 11 dígitos.

### is_masked

Verifica se o documento passado como parâmetro (`&str`) está mascarado de acordo com o documento correspondente. Retorna um valor booleano (`bool`), `true` caso o documento esteja mascarado, ou `false` caso contrário.

```rust
use brado::cpf;

cpf::is_masked("639.292.470-11"); // true (CPF válido com máscara)
cpf::is_masked("639.292.470-10"); // true (CPF inválido com máscara)

cpf::is_masked("63929247011"); // false (CPF válido sem máscara)
cpf::is_masked("6392.92.470-11"); // false (CPF válido mascarado errado)
cpf::is_masked("639.292.470-110"); // false (CPF inválido com máscara)
```

> OBS: `cpf::is_masked` verifica se a string passada está mascarada como um CPF. `cnpj::is_masked` verifica se a string passada está mascarada como um CNPJ.

### generate

Gera um novo documento sem símbolos (`String`).

```rust
use brado::cpf;

cpf::generate(); // "63929247011"
```

### generate_masked

Gera um novo documento mascarado (`String`).

```rust
use brado::cpf;

cpf::generate_masked(); // "639.292.470-11"
```

### docs::is_cpf, docs::is_cnpj, docs::is_cnh, docs::is_cns

São funções que verificam se o documento passado como parâmetro (`&str`) são, respectivamente, CPF, CNPJ, CNH e CNS válidos. Essas funções são atalhos (apelidos) para as funções de validação de cada documento. São indicadas para o contexto de identificação do tipo do documento.

```rust
use brado::docs;

docs::is_cpf("639.292.470-11"); // true
docs::is_cnpj("639.292.470-11"); // false
```


# Como Contribuir

1. Fazer um fork do projeto;
2. Criar um Pull Request com as sugestões de alteração.

## (Opcional) Configuração do Projeto via Nix

1. Instalar nix (com flakes, ou seja, >2.4);
2. Fazer um fork do projeto;
3. Clonando o repositório, e considerando que se está usando `direnv`:
```bash
nix flake clone github:your-github-user/brado --dest ./brado \
&& cd brado 1>/dev/null 2>/dev/null \
&& direnv allow
```


## Testes

Para rodar os testes, basta executar o comando a seguir:

```bash
make test
```
