fn main() {
    let res = is_palindrome(12331);
    println!("{}", res);
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    };
    let s: String = x.to_string();
    for i in 0..s.len()/2 {
        if s.chars().nth(i) != s.chars().nth(s.len()-1-i) {
            return false;
        }
    }
    return true;
}
