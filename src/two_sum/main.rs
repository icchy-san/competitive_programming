// Question URL: https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

fn main() -> Result<(), String> {
    // ========== Input data ==========
    // This case will return [0, 1].
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    // =================================

    let mut res_stored: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let key: i32 = target - num;
        match res_stored.get(&key) {
            Some(_v) => {
                println!("Fined! {:?}", vec![res_stored[&key] as i32, i as i32]);
                break;
            },
            None => {
                res_stored.insert(*num, i as i32);
            }
        }
    }

    // return Result type val, Tuple.
    return Ok(());
}
