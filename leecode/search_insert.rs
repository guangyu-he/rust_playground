fn main() -> () {
    let n: i32 = search_insert(vec![1, 3, 5, 6], 7);
    println!("{}", n);
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if target < nums[0] {
        return 0_i32;
    }

    let mut result: i32 = 0;

    for i in 0..nums.len() {
        if nums[i] == target {
            result = i as i32;
            break;
        } else if nums[i] < target {
            if i + 1 == nums.len() {
                result = nums.len() as i32;
                break;
            } else {
                continue;
            }
        } else {
            result = i as i32;
            break;
        }
    }
    result
}