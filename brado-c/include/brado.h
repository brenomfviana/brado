#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

bool validate_str(const char *document, bool is_masked, bool ignore_repeated);

} // extern "C"
