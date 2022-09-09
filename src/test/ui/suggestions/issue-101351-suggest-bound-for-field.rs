// run-rustfix
// check-only

trait Foo {
    type Associated;
}

struct Generic<T> {
    field: T::Associated, //~ ERROR associated type `Associated` not found
}

fn main() {}
