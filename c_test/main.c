#include "lib.h"
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    printf("is_even(2) = %d\n", is_even(2));
    char buf[100];
    scanf("%s", buf);
    printf("buf = %s\n", buf);
    return 0;
}
