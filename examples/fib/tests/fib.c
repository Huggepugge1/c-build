#include "../src/fib.h"
#include "test_framework.h"

TEST(fib) {
    ASSERT_EQ(fib(0), 0);
    ASSERT_EQ(fib(1), 1);
    ASSERT_EQ(fib(2), 1);
    ASSERT_EQ(fib(3), 2);
    ASSERT_EQ(fib(4), 3);
    ASSERT_EQ(fib(5), 5);
    ASSERT_EQ(fib(6), 8);
    ASSERT_EQ(fib(7), 13);
    ASSERT_EQ(fib(8), 21);
}
