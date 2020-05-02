#![feature(concat_idents)]

use rename_derive::rename;

macro_rules! module {
    ($name: ident) => {
        // Note: `concat_idents` won't work here.
        #[rename(append = "Data")]
        pub(crate) struct $name {
            pub one: u64,
            pub two: u64,
        }

        pub(crate) struct $name {
            data: concat_idents!($name, Data),
        }
    };
}

module!(Test);

fn main() {
    let test = Test {
        data: TestData { one: 1, two: 2 },
    };

    println!("test.data.one: {}", test.data.one);
    println!("test.data.two: {}", test.data.two);
}
