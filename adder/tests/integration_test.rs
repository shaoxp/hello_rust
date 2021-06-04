use adder;

mod common;

mod abc{
    #[test]
    fn it_add_two() {
        super::common::setup();
        assert_eq!(4,adder::add_two(2))
    }

    #[test]
    fn it_add_two_ne() {
        assert_ne!(5,adder::add_two(2))
    }
}