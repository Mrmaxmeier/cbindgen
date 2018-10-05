#include <cstdint>
#include <cstdlib>

struct Fns {
  void (*noArgs)();
  void (*anonymousArg)(int32_t);
  int32_t (*returnsNumber)();
  int8_t (*namedArgs)(int32_t first, int16_t snd);
};

extern "C" {

void root(Fns _fns);

} // extern "C"
