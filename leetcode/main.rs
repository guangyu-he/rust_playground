fn main() -> () {
    let nums = [0, 1, 1, 2, 3, 3, 4, 4, 5];
    let du_nums: i32;
    let rem_du_list: Vec<i32>;
    (du_nums, rem_du_list) = duplicate_item_in_array(nums.to_vec());
    println!("{}, {:?}", du_nums, rem_du_list);
}

fn duplicate_item_in_array(nums: Vec<i32>) -> (i32, Vec<i32>) {
    let mut output: Vec<i32> = vec![nums[0]];

    match nums.is_empty() {
        true => (0, output),
        false => {
            let mut duplicate_item: i32 = 0;
            let mut prev_item: i32 = nums[0];
            for mut i in 1..nums.len() {
                // println!("{},{}", prev_item, nums[i]);
                if prev_item == nums[i] {
                    duplicate_item += 1;
                    i += 1;
                    if i == nums.len() - 1 {
                        break;
                    } else {}
                } else {
                    prev_item = nums[i];
                    output.push(prev_item);
                }
            }
            (duplicate_item, output)
        }
    }
}