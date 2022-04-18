use std::vec;

// https://leetcode.com/problems/richest-customer-wealth/

fn main() {
    let accounts: Vec<Vec<i32>> = vec![
        vec![1,1,1,1],
        vec![1,1,1,2],
        vec![2,1,1,1]
    ];

    println!("Maximum wealth is {}", maximum_wealth(accounts));
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for user in accounts {
        let mut maximum = 0;
        for account in user {
            maximum += account;
        }

        if maximum > result {
            result = maximum;
        }
    }
    result
}
