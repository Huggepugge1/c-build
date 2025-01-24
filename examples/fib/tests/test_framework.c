#include "test_framework.h"

#include <setjmp.h>

jmp_buf jmpbuf;
char *error_msg;

void fail(char *msg) {
    error_msg = msg;
    longjmp(jmpbuf, 1);
}
