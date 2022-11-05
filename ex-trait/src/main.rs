use std::ops::Deref;

use ex_trait_lib::{self, MyOption, ToMyOption as ToMyOptionEx};

pub trait ToMyOption {
    type INPUT;
    type OUTPUT;
    fn conv(&self) -> MyOption<Self::OUTPUT>;
}

trait SingleGeneric<T> {}
impl<T> SingleGeneric<Option<T>> for Option<T> {}

// impl<T: JustGeneric, U> ToMyOption for T
// where
//     T: ex_trait_lib::ToMyOption<INPUT = T, OUTPUT = U> + Clone,
// {
//     type OUTPUT = U;
//     type INPUT = T;
//     fn conv(&self) -> MyOption<U> {
//         self.conv()
//     }
// }

impl<T, U> ToMyOption for Rc<dyn SingleGeneric<T>>
where
    T: ex_trait_lib::ToMyOption<INPUT = dyn SingleGeneric<T>, OUTPUT = U> + Clone,
{
    type OUTPUT = U;
    type INPUT = Rc<dyn SingleGeneric<T>>;
    fn conv(&self) -> MyOption<U> {
        let temp = self.deref();
        ex_trait_lib::ToMyOption::conv(temp)
    }
}

// impl ToMyOption for i64 {
//     type OUTPUT = i64;
//     type INPUT = i64;
//     fn conv(&self) -> MyOption<Self::OUTPUT> {
//         match self {
//             0 => MyOption::MyNone,
//             v @ _ => MyOption::MySome(*v),
//         }
//     }
// }

fn test<T, U>(val: impl ToMyOption<INPUT = T, OUTPUT = U>) -> MyOption<U> {
    val.conv()
}

fn main() {
    // let val = "Test".to_owned();
    // println!("{:#?}", test(val));

    // let val = "".to_owned();
    // println!("{:#?}", test(val));

    // let val = 12;
    // println!("{:#?}", test(val));

    // let val = 0;
    // println!("{:#?}", test(val));

    let val = Some(12i32);
    println!("{:#?}", val.conv());
    // println!("{:#?}", 11.conv());
    // println!("{:#?}", test(val));
}
