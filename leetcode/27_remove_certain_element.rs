fn main() -> () {
    let nums = [0, 1, 2, 2, 3, 0, 4, 2];
    println!("{:?}", duplicate_item_in_array(nums.to_vec(), 4))
}

fn duplicate_item_in_array(nums: Vec<i32>, item: i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];

    if nums.is_empty() {
        output
    } else {
        for i in 0..nums.len() {
            if nums[i] == item {

            } else {
                output.push(nums[i]);
            }
        }
        output
    }
}
