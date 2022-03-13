struct  User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {       
    let mut user1 = User {
        email : String::from("lovelycbm@naver.com"),
        username : String::from("lovelycbm"),
        sign_in_count : 1,
        active : true
    };
    
    user1.email = String::from("lovelybm@hanmail.net");
    
    let user2 = User {
        email : String::from("test@test.com"),
        username : String::from("test"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = PointI(0,0,0);
}


fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}