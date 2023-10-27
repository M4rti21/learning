fn main() {
    let ans: Vec<i32> = two_sum(vec![2, 7, 11, 15], 9);
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if i != j && nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}
