// this module contains a public trait Inc, to increment a value
// and it implements it by using a private trait Add
mod my_math {
    use private_parts::Add;

    pub struct Val {
        pub val: i32,
    }

    // this is necessary to encapsulate the private trait
    // the module is private, so the trait is not exported
    mod private_parts {
        pub trait Add {
            fn add(&mut self, other: i32);
        }
    }

    // in the following code, we have to use adequate namespacing
    impl Add for Val {
        fn add(&mut self, other: i32) {
            self.val += other;
        }
    }

    pub trait Inc: Add {
        fn inc(&mut self);
    }

    impl Inc for Val {
        fn inc(&mut self) {
            self.add(1)
        }
    }
}

fn main() {
    use my_math::Inc;
    let mut b = my_math::Val { val: 3 };
    println!("value: {}", b.val);
    b.inc();
    println!("value: {}", b.val);
}
