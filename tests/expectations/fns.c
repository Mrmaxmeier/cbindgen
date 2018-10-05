#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  void (*noArgs)();
  void (*anonymousArg)(int32_t);
  int32_t (*returnsNumber)();
  int8_t (*namedArgs)(int32_t first, int16_t snd);
} Fns;

void root(Fns _fns);
