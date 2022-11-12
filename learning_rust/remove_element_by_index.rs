fn main() -> () {
    let nums = ["a", "b", "c"];
    println!("{:?}", remove_element_by_index(nums.to_vec(), 1))
}

fn remove_element_by_index(mut input: Vec<&str>, index: usize) -> Vec<&str> {
    input.remove(index);
    input
}