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

Brado (BRAzilian DOcs validator) é um pacote Rust para validação de documentos brasileiros.

Este projeto é inspirado no [validate-docbr](https://github.com/alvarofpp/validate-docbr).

# Contribuindo

1. Instalar nix (com flakes, ou seja, >2.4);
2. Fazer um fork do projeto;
3. Clonando o repositório, e considerando que se está usando `direnv`:
```bash
nix flake clone github:your-github-user/brado --dest ./brado \
&& cd brado 1>/dev/null 2>/dev/null \
&& direnv allow
