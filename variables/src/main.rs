
fn main() {   
    let x = plus_one(5);

    println!("The value of x is {}",x);
}

fn plus_one(x: i32) -> i32 {
    // 반환 부분이 세미콜론이 없는것으로 이해하면 됨.
    let y = x+1;
    y
}