#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

bool cpf_validate_str(const char *document, bool is_masked, bool ignore_repeated);

bool cnpj_validate_str(const char *document, bool is_masked);

} // extern "C"
