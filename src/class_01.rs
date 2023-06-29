
pub fn class_test() {
    println!("（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符");
    mod_test1::fn_test1();
    println!();
    println!("（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符");
    mod_test2::fn_test2();
}

//（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
pub mod mod_test1 {
    pub fn fn_test1() {
        for ch in ('Z'..='a').rev() {
            print!("{}", ch);
        }
    }
}

//（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
pub mod mod_test2 {
    pub fn fn_test2() {
        for ch in 'A'..='z' {
            print!("{}", ch);
        }
    }
}
