#include <iostream>
#include <memory.h>
#include <cstdio>
#include "../include/brado.h"

int main() {
    char cpf[16] = "63929247011";
    if (cpf_validate_str(cpf, false, false)) {
        std::cout << "Válido!" << std::endl;
    } else {
        std::cout << "Inválido!" << std::endl;
    }

    char cnpj[22] = "05200851000100";
    if (cnpj_validate_str(cnpj, false)) {
        std::cout << "Válido!" << std::endl;
    } else {
        std::cout << "Inválido!" << std::endl;
    }
}
