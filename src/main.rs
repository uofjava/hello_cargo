use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    let mut guess = String::new();
    let number = rand::thread_rng().gen_range(1,100);
    println!("随机数是：{}", number);
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("输入值：{} ", guess);
    let guess:u32 = guess.trim().parse().expect("请输入数字");

    match guess.cmp(&number) {
        Ordering::Less => print!("小了"),
        Ordering::Greater => print!("大了"),
        Ordering::Equal => print!("OK")
    }
}
