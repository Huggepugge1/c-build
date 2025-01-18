# Testing
`c-builder` provides a simple way to run tests for your project.
Simply run `c-builder test` to run the tests.

## The testing framework
`c-builder` uses its own testing framework to run tests.
The framework is simple and easy to use.
To create a test file, simply create a new file in the `tests` directory.

Every test in the file is then written as follows:
```c
TEST(test_name) {
    // Test code here
}
```

To run the tests, simply run `c-builder test`.

## Test Macros
`c-builder` provides a few macros to help with testing.

### Passing a Test
If a test passes, the output will be:
```bash
Test `test_name` passed
```

### Failing a Test
If a test fails, the output will be:
```bash
Test `test_name` failed: <path_to_test_file>:<line_number>: <condition>
```

### `ASSERT`
The `ASSERT` macro is used to check if a condition is true.
If the condition is false, the test will fail.
```c
ASSERT(1 == 1);
```

### `ASSERT_EQ`
The `ASSERT_EQ` macro is used to check if two values are equal.
If the values are not equal, the test will fail.
```c
ASSERT_EQ(1, 1);
```

### `ASSERT_STRING_EQ`
The `ASSERT_STRING_EQ` macro is used to check if two strings are equal.
If the strings are not equal, the test will fail.
```c
ASSERT_STRING_EQ("hello", "hello");
```

### `ASSERT_NULL`
The `ASSERT_NULL` macro is used to check if a pointer is `NULL`.
If the pointer is not `NULL`, the test will fail.
```c
ASSERT_NULL(NULL);
```

### `ASSERT_NOT_NULL`
The `ASSERT_NOT_NULL` macro is used to check if a pointer is not `NULL`.
If the pointer is `NULL`, the test will fail.
```c
ASSERT_NOT_NULL(ptr);
```

### `ASSERT_FALSE`
The `ASSERT_FALSE` macro is used to check if a condition is false.
If the condition is true, the test will fail.
```c
ASSERT_FALSE(0);
```

### `ASSERT_FLOAT_EQ`
The `ASSERT_FLOAT_EQ` macro is used to check if two floating point numbers are equal.
This is done by checking if the difference between the numbers is less than a small epsilon value.
Epsilon is set to `1e-6`.
Future versions of `c-builder` may allow the user to set the epsilon value.
```c
ASSERT_FLOAT_EQ(1.0, 1.0);
```
