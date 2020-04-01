use rename_derive::rename;

#[doc = "This is just a simple test structure."]
#[rename(name = "Something")]
#[derive(Default, Debug)]
pub struct DemoName {
    one: u32,
    two: u32,
}

#[test]
fn test_create_default() {
    let test = Something::default();

    assert_eq!(test.one, 0);
    assert_eq!(test.two, 0);
}

#[test]
fn test_create_manual() {
    let test = Something { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
