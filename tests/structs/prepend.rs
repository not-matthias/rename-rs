#[rename(prepend = "Prepend")]
pub struct Placeholder {
    one: u32,
    two: u32,
}

#[test]
fn test_prepend() {
    let test = PrependPlaceholder { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
