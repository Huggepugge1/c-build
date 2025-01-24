#include <string.h>
#include <math.h>

#pragma once
#define TEST(name) void test_##name()
#define TO_STRING(x) #x
#define STRINGIFY(x) TO_STRING(x)

#define ASSERT(cond)                                                           \
    if (!(cond)) {                                                             \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #cond);                     \
    }

#define ASSERT_EQ(a, b)                                                        \
    if ((a) != (b)) {                                                          \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

#define ASSERT_STRING_EQ(a, b)                                                 \
    if (strcmp(a, b) != 0) {                                                   \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

#define ASSERT_NULL(a)                                                         \
    if ((a) != NULL) {                                                         \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != NULL");             \
    }

#define ASSERT_NOT_NULL(a)                                                     \
    if ((a) == NULL) {                                                         \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " == NULL");             \
    }
    
#define ASSERT_FALSE(a)                                                        \
    if (a) {                                                                   \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " is not false");        \
    }

#define ASSERT_FLOAT_EQ(a, b)                                                  \
    if (fabs((a) - (b)) > 1e-6) {                                              \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

typedef void test_fn();

struct Test {
    const char *name;
    test_fn *test;
};

void fail(char *msg);
