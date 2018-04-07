#include <stdio.h>
#include <stdlib.h>
#include "func1.h"

void func2(int con1, int con2)
{
  int x = 1;
  if (con1 == 0) {
    x = x + 1;
  }
  if (con2 > 1) {
    x = x * 2;
  }
  printf ("x = %d\n", x);
}
