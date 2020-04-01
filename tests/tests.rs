use rename_derive::rename;

#[doc = "This is just a simple test structure."]
#[rename(name = "Something")]
#[derive(Default, Debug)]
pub struct TestNameStruct {
    one: u32,
    two: u32,
}

#[test]
fn test_name() {
    let test = Something::default();

    assert_eq!(test.one, 0);
    assert_eq!(test.two, 0);
}

#[rename(append = "Two")]
pub struct Something {
    one: u32,
    two: u32,
}

#[test]
fn test_append() {
    let test = SomethingTwo { one: 1, two: 2 };

    assert_eq!(test.one, 1);
    assert_eq!(test.two, 2);
}
