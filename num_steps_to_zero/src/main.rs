use std::vec;

fn number_of_steps(num: i32) -> i32 {
    let mut num = num;
    let mut step;

    step = 0;
    while num > 0 {
        if num % 2 == 0 {
            num /= 2;
        }
        else {
            num -= 1;
        }
        step += 1;
    }
    step
}

fn main() {
    let test_set: Vec<i32> = vec![8, 123];
    println!("Counting number of step until 0");
    for num in test_set {
        let steps = number_of_steps(num);
        println!("{} required {} steps", num, steps);
    }
}
