#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct Fns {
  void (*noArgs)();
  void (*anonymousArg)(int32_t);
  int32_t (*returnsNumber)();
  int8_t (*namedArgs)(int32_t first, int16_t snd);
};

void root(struct Fns _fns);
