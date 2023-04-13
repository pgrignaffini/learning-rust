// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// the returned reference will be valid as long as both the parameters are valid.
// if the concrete lifetimes of the params are different, the lifetime returned by the function is the smaller of the lifetimes of the references passed in
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello, world!");
}
