# AGENTS.md

Diretrizes para agentes de IA e contribuidores que trabalham neste repositório.

## Visão Geral do Projeto

`brado` é uma biblioteca Rust para validação, mascaramento e geração de
documentos brasileiros (CPF, CNPJ, CNH, CNS, NIS, Título Eleitoral, RENAVAM,
Certidões de Nascimento, Casamento e Óbito). O nome é um acrônimo de
BRAzilian DOcs validator (validador de DOcumentos BRAsileiros).

- **Linguagem:** Rust (edição 2024)
- **Licença:** MIT
- **Idioma da documentação:** a documentação e os comentários voltados ao
  usuário são escritos em português brasileiro (`pt-BR`). Mantenha assim.
- **Dependências:** mantenha-as no mínimo absoluto. Não adicione
  dependências sem um forte motivo. As exceções aceitas são `rand` (geração
  aleatória) e, exclusivamente em `brado_derive`, o trio de macros
  procedural `syn`/`quote`/`proc-macro2`.

## Arquitetura: Workspace Estilo `serde`

O projeto é (em migração, ver [Plano de Migração](#estado-atual--plano-de-migração))
um **workspace Cargo** organizado como o `serde`: uma crate fachada, uma
crate core com os traits e utilitários, uma crate de macros procedural e
**uma crate por documento**. O usuário importa apenas os documentos de que
precisa, via features da fachada:

```rust
[dependencies]
brado = { version = "1", features = ["cpf"] } // importa apenas o CPF
```

```rust
use brado::cpf; // re-export da crate brado_cpf

cpf::validate("639.292.470-11");
```

### Crates

| Crate | Papel | Análogo no `serde` |
|-------|-------|--------------------|
| `brado` | **Fachada.** Re-exporta as crates de documento como módulos (`brado::cpf`), expõe os atalhos de identificação (`docs::is_cpf`, ...) e define as features por documento. É a única crate que o usuário precisa adicionar. | `serde` |
| `brado_core` | **Núcleo compartilhado.** Trait `Document` (API uniforme), auxiliares genéricos (`common`) e tipos de erro. Sem dependências de outros crates do workspace. | `serde_core` |
| `brado_derive` | **Macros procedural.** `#[derive(Brado)]` e atributos `#[brado(...)]` para validar structs cujos campos são documentos brasileiros. Única crate com dependências pesadas. | `serde_derive` |
| `brado_cpf`, `brado_cnpj`, `brado_cnh`, `brado_cns`, `brado_nis`, `brado_eleitoral`, `brado_renavam`, `brado_certidao` | **Uma crate por documento.** Implementa o trait `Document` e as funções livres do documento. | `serde_json`, `serde_yaml`, ... |

### Regras

- **Um documento = uma crate.** Diretório `brado_<doc>/` (underscore no
  filesystem), nome publicado `brado-<doc>` (hífen no crates.io). Nada de
  "submódulos de documento" dentro da fachada: a fachada apenas re-exporta.
- **Isolamento estrito.** Uma crate de documento depende apenas de
  `brado_core` (via `[workspace.dependencies]` com `path`). Ela **nunca**
  depende de outra crate de documento nem da fachada.
- **API pública uniforme.** Toda crate de documento expõe:
  - funções livres com os mesmos nomes e semântica em seu módulo raiz:
    `validate(doc: &str) -> bool`, `is_bare(doc: &str) -> bool`,
    `is_masked(doc: &str) -> bool`,
    `mask(doc: &str) -> Result<String, &'static str>`, `generate() -> String`,
    `generate_masked() -> String`;
  - um `pub struct` de marcador (ex.: `Cpf`) com `impl Document`.
- **`brado_core` é o único lugar de compartilhamento.** Auxiliares
  genéricos (`get_digits`, `get_symbols`, `unmask`, `is_repeated`,
  `random_decimal_vector`, ...) ficam em `brado_core::common` (públicos e
  documentados). O trait `Document` captura a API uniforme:

  ```rust
  pub trait Document {
      const NAME: &'static str;
      const SIZE: usize;

      fn validate(doc: &str) -> bool;
      fn is_bare(doc: &str) -> bool;
      fn is_masked(doc: &str) -> bool;
      fn mask(doc: &str) -> Result<String, &'static str>;
      fn generate() -> String;
      fn generate_masked() -> String;
  }
  ```

- **Fachada = re-export + features.** Cada documento corresponde a uma
  feature homônima na fachada (`cpf`, `cnpj`, ...) que habilita a dependência
  correspondente e o re-export `pub use brado_cpf as cpf;` sob
  `#[cfg(feature = "...")]`. Todos os documentos estão em `default` para
  manter `use brado::cpf` funcionando fora da caixa. Os atalhos
  `docs::is_<doc>` da fachada são guiados pela mesma feature do documento.
- **`brado_derive` é opcional.** A feature `derive` da fachada liga a
  dependência de `brado_derive` (estilo `serde/derive`), mas fica **fora**
  de `default` para não impor `syn`/`quote` a quem só valida documentos.
  O derive gera um `impl Document`/`impl Validate` para structs, usando
  atributos de campo como `#[brado(document = "cpf")]`.
- **Metadados compartilhados.** Versão, licença, edição e URLs vivem em
  `[workspace.package]`; dependências comuns (inclusive as crates internas
  do workspace) vivem em `[workspace.dependencies]`. Nunca repita versões
  nos `Cargo.toml` das crates.
- **Compatibilidade da API pública.** `brado::cpf::validate(...)` é a API
  estável prometida ao usuário; a migração não pode quebrá-la.

## Estado Atual & Plano de Migração

**Estado atual:** Fases 1 e 2 em andamento — o repositório é um **workspace
Cargo** (raiz + `brado_core` + fachada `brado` + `brado_cpf`); `common.rs`
vive em `brado_core::common` e o trait `Document` em `brado_core`. O CPF já
foi extraído para a crate `brado_cpf` (testes em `brado_cpf/src/tests.rs` e
re-export na fachada guiado pela feature `cpf`); os demais documentos ainda
são módulos autocontidos da fachada (`brado/src/<doc>.rs` + `docs.rs`). A
arquitetura completa é o **alvo**; novos documentos devem seguir o padrão
da `brado_cpf` (uma crate por documento, guiada por feature).

Fases da migração (concluídas em ordem, cada uma mantendo build, testes,
clippy e a API `brado::<doc>::<fn>` verde):

1. **Workspace + `brado_core` (concluída).** Criado o `Cargo.toml` de
   workspace na raiz (`members = ["brado", "brado_core"]`,
   `[workspace.package]`, `[workspace.dependencies]`); `common.rs` movido
   para `brado_core::common` e o trait `Document` criado em `brado_core`; a
   crate `brado` depende de `brado_core` e re-exporta `common` (`pub use
   brado_core::common;`) para manter a API pública.
2. **Extrair documentos (um PR por documento, comece pelo `cpf`).** Mover
   `brado/src/<doc>.rs` para a crate `brado_<doc>`; implementar
   `Document`; substituir na fachada o módulo por feature + re-export;
   converter `brado/tests/<doc>.rs` em `brado_<doc>/src/tests.rs`, um
   módulo `#[cfg(test)]` declarado no `lib.rs` da crate. Ordem sugerida:
   `cpf`, `cnpj`, `nis`, `renavam`, `cnh`, `cns`, `eleitoral`, `certidao`.
3. **`brado_derive`.** Criar a crate com o `#[derive(Brado)]`, os atributos
   `#[brado(...)]` e a feature `derive` na fachada, com testes de UI/compilação.
4. **Fachada final.** `docs.rs` (atalhos `is_*`) e `lib.rs` da fachada
   reduzidos a re-exports guiados por features; documentação da crate
   fachada como página principal do docs.rs.
5. **CI/publicação.** Atualizar workflows para o workspace
   (`cargo fmt --all`, `cargo clippy --workspace`, `cargo test --workspace`)
   e publicar na ordem `brado_core` → crates de documento → `brado_derive`
   → `brado`.

## Layout do Repositório (alvo)

```
.
├── AGENTS.md
├── Cargo.toml            # raiz do workspace ([workspace], [workspace.package], [workspace.dependencies])
├── Makefile              # atalhos de fmt / test / clippy no nível do workspace
├── flake.nix             # shell de dev Nix (git, gnumake, cargo, rustc, rustfmt, clippy)
├── README.md             # documentação do usuário em pt-BR
├── brado_core/           # trait Document + auxiliares comuns
│   └── src/
│       ├── lib.rs
│       ├── common.rs
│       └── document.rs
├── brado_derive/         # macros #[derive(Brado)] e #[brado(...)]
│   └── src/lib.rs
├── brado/                # crate fachada (features por documento + re-exports)
│   └── src/
│       ├── lib.rs
│       ├── docs.rs       # atalhos is_cpf, is_cnpj, ...
│       └── tests.rs      # testes de fumaça dos re-exports
├── brado_cpf/            # uma crate por documento...
│   └── src/
│       ├── lib.rs
│       └── tests.rs
├── brado_cnpj/
├── brado_cnh/
├── brado_cns/
├── brado_nis/
├── brado_eleitoral/
├── brado_renavam/
└── brado_certidao/
```

## Comandos

Após a migração, todos os comandos rodam na **raiz do repositório** (raiz do
workspace); o Makefile permanece como atalho:

| Tarefa | Makefile | Cargo direto |
|--------|----------|--------------|
| Formatar | `make fmt` | `cargo fmt --all` |
| Testar (workspace) | `make test` | `cargo test --workspace` |
| Testar (uma crate) | `make test mod=brado_cpf` | `cargo test -p brado-cpf` |
| Lint | `make clippy` | `cargo clippy --workspace` |

Argumentos extras podem ser passados via `args="..."` (ex.:
`make test args="-- --nocapture"`).

O ambiente de desenvolvimento pode ser carregado com Nix + direnv
(`use flake` no `.envrc`); ele fornece `cargo`, `rustc`, `rustfmt` e
`clippy`.

## Portões de Qualidade

O CI (`.github/workflows/tests.yaml`) executa, nesta ordem, em cada pull
request: build, testes e clippy no workspace. **Os três devem passar antes
de considerar uma tarefa concluída.** Localmente, antes de dar a tarefa
como pronta:

1. `cargo fmt --all` — o código deve estar limpo conforme o rustfmt
   (configurações padrão).
2. `cargo clippy --workspace` — sem warnings; corrija cada lint em vez de
   suprimi-lo (evite `#[allow(...)]`, a menos que o código seja
   intencionalmente incomum e o motivo esteja documentado).
3. `cargo test --workspace` — inclui doctests; todos devem passar.

A publicação de releases (`.github/workflows/deploy.yaml`) publica as crates
na ordem de dependência: `brado_core` → crates de documento →
`brado_derive` → `brado`.

## Estilo de Código

- Siga as configurações padrão do rustfmt; corresponda ao estilo existente
  (ex.: anotações de tipo explícitas em bindings `let`, funções pequenas e
  focadas, retornos antecipados para falhas de validação).
- Prefira entradas `&str` e saídas baratas `bool`/`String`/`Result`; esta é
  uma biblioteca de validação em caminho quente (hot path), evite alocações
  e clones desnecessários nos caminhos de validação.
- Sem código `unsafe` (inclusive em `brado_derive`; use `syn`/`quote` para
  gerar código, não escreva tokens à mão).
- Exporte apenas a API de que o usuário precisa; mantenha auxiliares
  internos de documento privados (`fn`), a menos que sejam úteis em várias
  crates (nesse caso, mova para `brado_core::common` e documente-os).

## Requisitos de Documentação

- Todo item público precisa de um comentário de documentação `///` em
  `pt-BR` descrevendo o que ele faz e indicando o significado do retorno.
- Os comentários de documentação devem conter seções `## Exemplos` com
  doctests **executáveis** (eles são executados pelo `cargo test`). Marque
  exemplos esperados falhar com `should_panic` quando apropriado.
- Doctests da fachada usam `brado::...`; doctests das crates de documento
  usam o nome próprio da crate (`brado_cpf::...`).
- A documentação de crate fica no topo de cada `src/lib.rs`; a página
  principal do docs.rs é a da fachada `brado`.

## Testes

- **Não existem diretórios `tests/` de integração:** os testes vivem em um
  arquivo dentro de `src/` de cada crate.
- Cada crate de documento mantém seus testes em `brado_<doc>/src/tests.rs`,
  declarado no `lib.rs` com `#[cfg(test)] mod tests;`.
- Dentro do módulo, use `use super::*;` (ou `use crate::*;`) e mantenha os
  nomes `<doc>_<função>_<caso>` (ex.: `cpf_validate_1_valid_cpfs`).
- Cada documento precisa cobrir: `validate` (válido, inválido, dígitos
  repetidos, máscaras malformadas), `is_bare`, `is_masked`, `mask` (sucesso
  e falha), `generate` e `generate_masked`, além dos métodos do trait
  `Document`.
- A fachada mantém testes de fumaça em `brado/src/tests.rs` garantindo que
  os re-exports guiados por feature funcionam (`brado::cpf::validate`,
  `docs::is_cpf`).
- Os doctests dos comentários `///` continuam sendo executados pelo
  `cargo test`.
- Ao adicionar um documento, adicione seu arquivo de testes junto da crate.

## Checklist: Adicionar um Documento Novo

1. Crie a crate `brado_<doc>/` (nome publicado `brado-<doc>`) dependendo
   apenas de `brado_core` via `[workspace.dependencies]`.
2. Implemente as funções livres públicas (`validate`, `is_bare`,
   `is_masked`, `mask`, `generate`, `generate_masked`) e o trait
   `Document` para um marcador (ex.: `Cpf`).
3. Reutilize auxiliares de `brado_core::common`; adicione novos auxiliares
   genéricos lá.
4. Registre a crate no `members` do workspace e em `[workspace.dependencies]`.
5. Na fachada: adicione a dependência, a feature homônima (em `default`) e
   o re-export `pub use brado_<doc> as <doc>;` sob `#[cfg(feature = "...")]`.
6. Adicione o atalho `is_<doc>` em `brado/src/docs.rs`, guiado pela mesma
   feature.
7. Adicione `brado_<doc>/src/tests.rs` cobrindo todas as funções públicas,
   declarado com `#[cfg(test)] mod tests;` no `lib.rs`.
8. Atualize o `README.md` (lista de documentos), o `Makefile` se preciso, e
   a documentação da fachada.
9. Rode `make fmt`, `make clippy` e `make test` na raiz do workspace.
