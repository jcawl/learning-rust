//Given a list of integers, use a vector and return the mean (the average value), median (when sorted,
//the value in the middle position), and mode (the value that occurs most often;
//a hash map will be helpful here) of the list.


use std::collections::HashMap;

fn main() {
    //vector of integers
    let mut nums = vec![10,20,33,45,67,23,12,20,55,66,24,20,77,87];

    //get mean
    let mut mean = 0.0;
    for i in &nums {
        mean += *i as f32;
    }
    println!("Mean: {}", mean / nums.len() as f32);

    //get median
    nums.sort();
    let sz = nums.len() - 1;
    if nums.len() % 2 == 0 {
        let index = sz / 2;
        let median = (nums[index] as f32 + nums[index+1] as f32) / 2.0;
        println!("Median: {}", median);
    } else {
        let median = nums[nums.len() / 2 as usize];
        println!("Median: {}", median);
    }

    //get model index
    let mut num_count = HashMap::new();
    for i in &nums {
        let count = num_count.entry(i).or_insert(0);
        *count += 1;
    }
    println!("Mode: {:?}", num_count);
}