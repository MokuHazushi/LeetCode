use std::vec;

// https://leetcode.com/problems/baseball-game/

fn cal_points(ops: Vec<String>) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut result;

    for s in ops {
        match s.parse() {
            Ok(num) => nums.push(num),
            Err(_) => match s.as_str() {
                "+" => nums.push(nums[nums.len()-1]+nums[nums.len()-2]),
                "D" => nums.push(nums[nums.len()-1]*2),
                "C" => { nums.pop(); }
                &_ => { continue; }
            }
        }
    }

    result = 0;
    for n in nums {
        result += n;
    }

    result    
}

fn main() {
    let test: Vec<String> = vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string()];
    println!("Calculating baseball game score following weird rules");

    print!("Score tab: [");
    for s in &test {
        print!("{},", s);
    }
    println!("] output result {}", cal_points(test));
}
