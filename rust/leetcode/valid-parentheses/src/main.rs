use std::collections::LinkedList;

fn main() {
    let res = is_valid("()[]{}".to_string());
    println!("{}", res);
}

pub fn is_valid(s: String) -> bool {
    let mut stack: LinkedList<char> = LinkedList::new();
    let open: LinkedList<char> = LinkedList::from(['[', '{', '(']);
    let close: LinkedList<char> = LinkedList::from([']', '}', ')']);
    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();
        if !open.contains(&c) && !close.contains(&c) {
            return false;
        }
        if open.contains(&c) {
            stack.push_back(c);
        } else {
            if (stack.len() < 1) {
                return false;
            }
            let old = stack.pop_back().unwrap();
            if open.iter().position(|&x| x == old) != close.iter().position(|&y| y == c) {
                return false;
            }
        }
    }
    if stack.len() > 0 {
        return false;
    }
    return true;
}
