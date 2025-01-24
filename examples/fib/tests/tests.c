#include "test_framework.h"

#include "fib.c"

#include <stdio.h>
#include <setjmp.h>

extern jmp_buf jmpbuf;
extern char *error_msg;

struct Test tests[] = {
{ "fib", test_fib },
};

int main() {
for (int i = 0; i < 1; i++) {
if (setjmp(jmpbuf) == 0) {
tests[i].test();
printf("Test `%s` passed\n", tests[i].name);
} else {
printf("Test `%s` failed: %s\n", tests[i].name, error_msg);
}
}
return 0;
}
