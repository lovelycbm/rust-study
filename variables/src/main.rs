fn main() {
    // 첫번째와 두번째는 다른 spaces 임 
    
    // let _guess: u32 = "42".parse().expect("Not a number!");

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    let _sum = 5 + 10 ;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _qutient = 57.7 /32.2;
    let _remainder = 43 % 5;
    
                
    let _tup = (500, 6.4, 1);
    
    let (x, y, z) = _tup;
    println!("The value of y is: {}", y);    
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    // 뒤부분이 인덳로 접근하는 느낌이구나 
    // 다른 곳 튜플이랑 비슷한듯 
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    
    println!("{}",five_hundred);
    
    
    
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    // index 에러 확인함. 
    let element = a[index];

    println!("The value of element is: {}", element);
    
    
    
    
    println!("qutient: {}", _qutient);


    
}
