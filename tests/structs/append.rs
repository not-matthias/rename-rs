#[rename(append = "Append")]
pub struct Placeholder {
    one: u32,
    two: u32,
}

#[test]
fn test_append() {
    let test = PlaceholderAppend { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
