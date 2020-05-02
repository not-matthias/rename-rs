pub mod append;
pub mod name;
pub mod prepend;

#[rename(name = "Two", prepend = "One", append = "Three")]
pub struct Placeholder {
    one: u32,
    two: u32,
}

#[test]
fn test_prepend_name_append() {
    let test = OneTwoThree { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
