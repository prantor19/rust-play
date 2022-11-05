#[derive(Debug)]
pub enum MyOption<T> {
    MySome(T),
    MyNone,
}

pub trait ToMyOption {
    type INPUT;
    type OUTPUT;
    fn conv(&self) -> MyOption<Self::OUTPUT>;
}

impl<T> ToMyOption for Option<T>
where
    T: Clone,
{
    type OUTPUT = T;
    type INPUT = Option<Self::OUTPUT>;
    fn conv(self: &Self::INPUT) -> MyOption<Self::OUTPUT> {
        match &self {
            Some(v) => MyOption::MySome(v.clone()),
            None => MyOption::MyNone,
        }
    }
}

impl ToMyOption for String {
    type OUTPUT = String;
    type INPUT = String;
    fn conv(&self) -> MyOption<Self::OUTPUT> {
        match self.as_str() {
            "" => MyOption::MyNone,
            v @ _ => MyOption::MySome(v.to_owned()),
        }
    }
}

impl ToMyOption for i32 {
    type OUTPUT = i32;
    type INPUT = i32;
    fn conv(&self) -> MyOption<Self::OUTPUT> {
        match self {
            0 => MyOption::MyNone,
            v @ _ => MyOption::MySome(*v),
        }
    }
}
