#include "../src/fib.h"
#include <stdio.h>

int main() {
    for (int i = 0; i < 20; i++) {
        printf("%d: %d\n", i, fib(i));
    }
    return 0;
}
