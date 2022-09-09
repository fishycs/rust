pub trait Foo {
    fn do_it(&self);
}

fn use_it<T>(input: T) {
    input.do_it();
}

fn main() {}
