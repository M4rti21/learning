fn main() {
    let res = longest_common_prefix(vec![
        "a".to_string()
    ]);
    println!("{}", res);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res: String = String::new();
    for i in 0..strs[0].len() {
        let comp: char = strs[0].chars().nth(i).unwrap();
        for j in 0..strs.len() {
            if strs[j].len() <= i {
                return res;
            }
            let this: char = strs[j].chars().nth(i).unwrap();
            if this != comp {
                return res;
            }
        }
        res += &comp.to_string();
    }
    return res;
}

// Input: strs = ["flower","flow","flight"]
// Output: "fl"