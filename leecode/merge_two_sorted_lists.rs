fn main() -> () {
    let a = [5, 3, 7, 9, 1, 4, 2, 0, 6, 8];
    let b = [1, 2, 3];
    println!("{:?}", bubble_sort(append_vec(a.to_vec(), b.to_vec())));
}

fn append_vec(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.into_iter().chain(b.into_iter()).collect()
}

fn bubble_sort(ori_list: Vec<i32>) -> Vec<i32> {
    let mut a = ori_list;
    for i in 0..a.len() {
        for j in 0..a.len() - 1 - i {
            // 交换
            if a[j] > a[j + 1] {
                a[j] = a[j] ^ a[j + 1];
                a[j + 1] = a[j] ^ a[j + 1];
                a[j] = a[j] ^ a[j + 1];
            }
        }
    }
    a
}