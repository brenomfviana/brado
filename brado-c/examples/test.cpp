#include <iostream>
#include <memory.h>
#include <cstdio>
#include "../include/brado.h"

int main() {
    char cpf[16] = "63929247011";
    if (validate_str(cpf, false, false)) {
        std::cout << "Válido!" << std::endl;
    } else {
        std::cout << "Inválido!" << std::endl;
    }
}
