# brado
Bradova (BRAzilian DOcs validator) é um pacote Rust para validação de documentos brasileiros.

Este projeto é inspirado no [validate-docbr](https://github.com/alvarofpp/validate-docbr).

# Contribuindo

Instalar nix (com flakes, ou seja, >2.4).


Fazer um fork do projeto.


Clonando o repositório, e considerando que se está usando `direnv`:
```bash
nix flake clone github:your-github-user/brado --dest ./brado \
&& cd brado 1>/dev/null 2>/dev/null \
&& direnv allow
```
