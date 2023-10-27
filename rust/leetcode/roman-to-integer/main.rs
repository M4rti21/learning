use std::collections::HashMap;

fn main() {
    let inp = String::from("MCMXCIV");
    let res = roman_to_int(inp);
    println!("{}", res);
}

pub fn roman_to_int(s: String) -> i32 {
    let nums = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut ans: i32 = 0;
    let mut do_next: bool = true;
    for i in 0..s.len() {
        let num: i32 = *nums.get(&s.chars().nth(i).unwrap()).unwrap();
        if !do_next {
            do_next = true;
        } else {
            if i + 1 < s.len() {
                let next: i32 = *nums.get(&s.chars().nth(i+1).unwrap()).unwrap();
                if next > num {
                    ans += next - num;
                    do_next = false;
                } else {
                    ans += num;
                }
            } else {
                ans += num;
            }
        }
    }
    return ans;
}
