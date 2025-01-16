mod custom_types;
#[macro_use]
mod macro_rules;
mod closures;
mod r#unsafe;

fn main() {
    say_hello!();
    create_function!(foo);
    create_function!(bar);

    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    print!("{:?}", calculate!(eval 1 + 2));
}
