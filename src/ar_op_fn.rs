use std::io;

fn separator_line() {
    for _ in 0..=50 {
        print!("-");
    }
    println!()
}
//entry point cli
pub fn ar_op() -> bool {
    let mut yes_no: String = String::new();

    println!("Want to do some aritmetic operations? (Y/N): ");

    //gets input
    io::stdin().read_line(&mut yes_no).expect("Expect (Y/N)");
    let yes_no_lower: char = yes_no.to_lowercase().chars().next().unwrap();

    if yes_no_lower == 'y' {
        true
    } else {
        false
    }
}

// This ask for what arithmetic you want
pub fn ar_op_fn() {
    separator_line();

    loop {
        println!("What you want to do?");
        println!("1. (+)");
        println!("2. (-)");
        println!("3. (*)");
        println!("4. (/)");
        println!("5. Exit");

        let mut select: String = String::new();
        io::stdin().read_line(&mut select).expect("Expect a number");
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if select <= 0 || select >= 5 {
            separator_line();
            println!("You exit the program :D.");
            std::process::exit(0)
        }

        if select > 1 || select < 5 {
            let res: i32 = ar_op_fn_operations(select);
            println!("result = {}", res);
        }
        separator_line();
    }
}

//This does the different arithmetic operations
pub fn ar_op_fn_operations(option: i32) -> i32 {
    let mut first_num: String = String::new();
    let mut second_num: String = String::new();
    let res: i32;

    println!("Enter your numbers");
    io::stdin()
        .read_line(&mut first_num)
        .expect("Expect first number");
    io::stdin()
        .read_line(&mut second_num)
        .expect("Expect first number");

    let first_num: i32 = match first_num.trim().parse() {
        Ok(num) => num,
        Err(_) => std::process::exit(0),
    };
    let second_num: i32 = match second_num.trim().parse() {
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
