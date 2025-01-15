// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}
