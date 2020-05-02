#[rename(name = "Name")]
pub struct Placeholder {
    one: u32,
    two: u32,
}

#[test]
fn test_name() {
    let test = Name { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
