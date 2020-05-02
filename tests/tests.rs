use rename_derive::rename;

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

#[rename(prepend = "Prepend")]
pub struct Prepend {
    one: u32,
    two: u32,
}

#[test]
fn test_prepend() {
    let test = PrependPrepend { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}

#[rename(append = "Append")]
pub struct Append {
    one: u32,
    two: u32,
}

#[test]
fn test_append() {
    let test = AppendAppend { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}

#[rename(name = "Two", prepend = "One", append = "Three")]
pub struct AnotherPlaceholder {
    one: u32,
    two: u32,
}

#[test]
fn test_prepend_name_append() {
    let test = OneTwoThree { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
