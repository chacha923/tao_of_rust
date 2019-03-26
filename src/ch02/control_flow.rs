pub fn if_expr(){
    let n = 13;
    let big_n = if n < 10 && n > -10 {
        10 * n
    }else {
        n / 2
    };
    assert_eq!(big_n, 6);
}

pub fn while_fizzbuzz(){    
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz")
        }else if n % 3 == 0 {
            println!("fizz")
        }else if n % 5 == 0 {
            println!("buzz");
        }else {
            println!("{}", n);
        }
        n += 1;
    }
}

pub fn for_fizzbuzz(){
    for n in 1..101 {
           if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn while_true(x:i32) -> i32{
    while true {
        return x + 1;
    }
    x
}

pub fn if_true(x:i32) -> i32 {
    if true {
        return x+1;
    }
    x
}

/// # match 匹配
///
/// Basic usage:
///
/// ```
/// fn match_expr() {
///     let number = 42;
///     match number {
///         0 => println!("Origin"),  // 匹配数字
///         1...3 => println!("All"), // 匹配范围
///         | 5 | 7 | 13 => println!("Bad Luck"), // 匹配相同的分支
///         n @ 42 => println!("Answer is {}", n), // 使用@可以创建绑定n，分支右侧表达式中可用
///         _ => println!("Common"),  // 下划线为通用匹配
///     }
/// }
/// match_expr();
/// ```
pub fn match_expr(){
    let number = 42;
    match number {
        0 => println!("Origin"),
        1...3 => println!("All"),
        |5|7|13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }
}

pub fn match_bool() {
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    assert_eq!(binary,1)
}

pub fn if_let_bool(){
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
}
