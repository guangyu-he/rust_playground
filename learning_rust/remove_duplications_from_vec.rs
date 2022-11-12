use itertools::Itertools;

fn main() -> () {
    let nums = ["b", "a", "b", "c"];
    let rem_du_list: Vec<_>;
    rem_du_list = duplicate_item_in_array(nums.to_vec());
    println!("{:?}", rem_du_list);
}

fn duplicate_item_in_array(input: Vec<&str>) -> Vec<&str> {
    let output: Vec<_> = input.into_iter().unique().collect();
    output
}