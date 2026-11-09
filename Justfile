# Atalhos de desenvolvimento (workspace brado)
# Uso: just <receita> [mod=<crate>] [args="..."]
# Ex.: just test mod=brado_cpf args="-- --nocapture"

set shell := ["bash", "-uc"]

# Formata todo o workspace
fmt:
    cargo fmt --all

# Executa clippy (workspace inteiro ou crate específica via mod)
clippy mod="" *args="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ -n "{{mod}}" ]]; then
        cargo clippy -p "{{mod}}" {{args}}
    else
        cargo clippy --workspace {{args}}
    fi

# Executa testes (workspace inteiro ou crate específica via mod)
test mod="" *args="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ -n "{{mod}}" ]]; then
        cargo test -p "{{mod}}" {{args}}
    else
        cargo test --workspace {{args}}
    fi

# Roda exemplos (todos, ou apenas os informados)
# Uso: just examples                (todos)
#      just examples derive_struct  (específico)
#      just examples ex1 ex2        (vários)
examples *names:
    #!/usr/bin/env bash
    set -euo pipefail
    names=( {{names}} )
    if [[ ${#names[@]} -eq 0 ]]; then
        for dir in examples/*/; do
            echo "==> $dir"
            (cd "$dir" && cargo run --quiet)
        done
    else
        for name in "${names[@]}"; do
            dir="examples/$name"
            if [[ ! -d "$dir" ]]; then
                echo "Exemplo não encontrado: $name (esperado em $dir)" >&2
                exit 1
            fi
            echo "==> $dir/"
            (cd "$dir" && cargo run --quiet)
        done
    fi
