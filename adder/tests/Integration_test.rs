use adder;

mod common;
// an integration test of a funcion in the adder crate
#[test]() {
    fn it_adds_two() {
        common::setup();
        assert_eq(4, adder::add_two(2));
    }
}
