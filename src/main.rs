mod ar_op_fn;

fn main() {
    let entry: bool = ar_op_fn::ar_op();

    if entry {
        ar_op_fn::ar_op_fn();
        println!("You exit the program :D.")
    } else {
        println!("You exit the program :D.")
    }
}
