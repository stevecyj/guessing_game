use std::io;

fn main() {
    println!("請猜測一個數字：");
    println!("請輸入你的猜測數字。");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("讀取行數失敗");

    println!("你猜測的數字是：{guess}");
}
