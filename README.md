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

Brado fornece funções para identificação, validação e geração de documentos brasileiros. O nome desta biblioteca (Brado) é um acronimo de BRAzilian DOcs validator (validador de DOcumentos BRAsileiros).

> :warning: A documentação desta biblioteca pode ser acessada [aqui](https://docs.rs/brado/).


## Guia Rápido

Para adicionar o pacote ao projeto, basta rodar o seguinte comando:

```bash
cargo add brado
```

Ou adicionar a linha a seguir no arquivo `Cargo.toml`:

```toml
brado = "0.3.5"
```


## Documentos

- [x] CPF: Cadastro de Pessoa Física;
- [x] CNH: Carteira Nacional de Habilitação;
- [x] CNPJ: Cadastro Nacional da Pessoa Jurídica;
- [ ] CNS: Cartão Nacional de Saúde;
- [ ] PIS: PIS/NIS/PASEP/NIT;
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

Mascara o documento passado como parâmetro (`&str`). Retorna a string (`String`) correspondente ao documento mascarado. A string passada não deve possuir símbolos.

```rust
use brado::cpf;

cpf::mask("63929247011"); // "639.292.470-11"

cpf::mask("639.292.470-11"); // panic!
cpf::mask("639292470"); // panic!
```

### is_bare

Verifica se o documento passado como parâmetro (`&str`) não possui símbolos. Retorna um valor booleano (`bool`), `true` caso o documento não possua símbolos, ou `false` caso contrário.

```rust
use brado::cpf;

cpf::is_bare("63929247011"); // true
cpf::is_bare("63929247010"); // true

cpf::is_bare("639.292.470-11"); // false
cpf::is_bare("639.29247011"); // false
cpf::is_bare("639292470110"); // false
```

> OBS: se for utilizada a função `cpf::is_bare` para verificar se um CNPJ não possui símbolos, o resultado será `false`! Isso acontece pois esta função considera que a string é um CPF, ou seja, possui 11 dígitos.

### is_masked

Verifica se o documento passado como parâmetro (`&str`) está mascarado de acordo com o documento correspondente. Retorna um valor booleano (`bool`), `true` caso o documento esteja mascarado, ou `false` caso contrário.

```rust
use brado::cpf;

cpf::is_masked("639.292.470-10"); // true

cpf::is_masked("63929247011"); // false
cpf::is_masked("6392.92.470-11"); // false
cpf::is_masked("639.292.470-110"); // false
```

> OBS: `cpf::is_masked` verifica se a string passada está mascarada como um CPF. `cnpj::is_masked` verifica se a string passada está mascarada como um CNPJ.

### generate

Gera um novo documento sem símbolos (`String`).

```rust
use brado::cpf;

cpf::generate(); // "639.292.470-11"
```

### generate_masked

Gera um novo documento mascarado (`String`).

```rust
use brado::cpf;

cpf::generate_masked(); // "639.292.470-11"
```


# Como Contribuir

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
