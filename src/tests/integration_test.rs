// integration tests are entirely external to your library.
// They use your library in the same way any other code would,
// which means they can only call functions that are part of your library’s public API.
// Their purpose is to test whether many parts of your library work together correctly.
// Units of code that work correctly on their own could have problems when integrated,
// so test coverage of the integrated code is important as well.
// To create integration tests, you first need a tests directory.

// use adder;

// #[test]
// fn it_adds_two() {
//     assert_eq!(4, adder::add_two(2));
// }
