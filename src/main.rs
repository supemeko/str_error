use str_error::{self, StrError};

#[derive(Debug)]
struct SomeStructA {}

#[derive(Debug)]
struct SomeStructB {}

fn step1<T: StrError>(str_error: &mut T) -> Option<SomeStructB> {
    str_error.report("start step1");
    // ...
    str_error.report("step1 is ok");
    Some(SomeStructB {})
}

fn step2<T: StrError>(str_error: &mut T, a: &SomeStructA, b: &SomeStructB) {
    str_error.report("start step2");
    // ...
    str_error.report(&format!("display {:?}, {:?}", a, b));
    // ...
    str_error.throws("we have encountered some errors");
}

fn some_handle<T: StrError>(str_error: &mut T, a: &SomeStructA) -> Option<SomeStructB> {
    let b: SomeStructB = step1(str_error)?;
    // u can Check for errors and return manually, like this:
    if str_error.catch() {
        // tailing in work
        return None;
    }
    // or just return
    //str_error.catch_return()?;

    step2(str_error, &a, &b);

    str_error.upthrows_if()?;

    Some(b)
}

fn main() {
    println!("Hello, world!");
    let mut err = str_error::first();

    let a = SomeStructA {};
    let b = some_handle(&mut err, &a);
    if b.is_none() || err.catch() {
        println!("----error report----\n{}\n---- auto repeat error and log record ----", err.err_msg());
        let mut reporter = str_error::reporter();
        some_handle(&mut reporter, &a);
        for u in &reporter.reporter.data {
            println!("{}", u)
        }
        println!("----end of report----");
        return;
    }
    println!("if normal")
}
