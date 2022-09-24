use std::io;

// This ask for what arithmetic you want
pub fn ar_op_fn() {
    println!("sds");
    loop {
        println!("What you want to do?");
        println!("1. (+)");
        println!("2. (-)");
        println!("3. (*)");
        println!("4. (/)");

        let mut select: String = String::new();
        io::stdin().read_line(&mut select).expect("Expect a number");
        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if select > 0 || select < 5 {
        } else {
            println!("Your option {} isn't available", select);
            std::process::exit(0)
        }
    }
}

//This does the different arithmetic operations
pub fn ar_op_fn_operations(option: i32) -> i64 {
    let mut first_num: String = String::new();
    let mut second_num: String = String::new();
    let mut res: i64 = 0;

    io::stdin()
        .read_line(&mut first_num)
        .expect("Expect first number");
    io::stdin()
        .read_line(&mut second_num)
        .expect("Expect first number");

    let first_num: i64 = match first_num.trim().parse() {
        Ok(num) => num,
        Err(_) => std::process::exit(0),
    };
    let second_num: i64 = match second_num.trim().parse() {
        Ok(num) => num,
        Err(_) => std::process::exit(0),
    };

    if option == 1 {
        res = first_num + second_num;
    } else if option == 2 {
        res = first_num - second_num;
    } else if option == 3 {
        res = first_num * second_num;
    } else if option == 4 {
        res = first_num / second_num;
    } else {
        std::process::exit(0)
    }
    res
}
