mod cpp_glue;
pub use crate::cpp_glue::*;

#[derive(Clone, Default)]
pub struct Foo {
    data: i32,
    text: String
}

impl Foo {
    fn new(val: i32) -> Foo {
        let mut f = Foo::default();
        f.text = String::from("Hello World!");
        f.set_field(val);
        return f;
    }

    fn f(&self, a: i32, b: i32) -> i32 {
        self.data + a + b
    }

    fn set_field(&mut self, v: i32) {
        self.data = v;
    }
    fn get_text(&self) -> &str { self.text.as_str() }

}

fn f2(a: i32) -> i32 {
    a * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = Foo::new(5);
        assert_eq!(8, foo.f(1, 2));

        let foo = Foo::default();
        assert_eq!(0, foo.f(0, 0));
    }
}
