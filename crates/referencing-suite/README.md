# referencing-suite

This is a small supporting library for generating test cases from the JSON referencing test suite.
The main entrypoint is the `test` macro that accepts a path to the suite and generates test functions
that accept paths to individual test cases.

