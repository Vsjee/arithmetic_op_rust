use std::io;
mod ar_op_fn;

fn main() {
    let entry: bool = ar_op();
    println!("{}", entry);

    if true {
        let res: i64 = ar_op_fn::ar_op_fn_operations(1);
        println!("{}", res);
        println!("yo")
    }
}

fn ar_op() -> bool {
    let mut t_f: bool = false;
    let mut yes_no: String = String::new();
    println!("Want to do some aritmetic operations? (Y/N): ");

    //gets input
    io::stdin().read_line(&mut yes_no).expect("Expect (Y/N)");

    //to lower
    let yes_no_lower: String = yes_no.to_lowercase();

    println!("{}", yes_no_lower);
    if yes_no_lower.eq("y") {
        t_f = true;
    }
    println!("{}", t_f);
    t_f
}
