use rand::{Rng, thread_rng};
use tokio::time::Instant;

fn bubble_sort(unsorted_list: Vec<i32>) -> Vec<i32> {
    let mut list = unsorted_list.clone();
    let length = list.len();

    for _ in 0..length {
        for j in 0..length-1 {
            if list[j] >= list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }

    list
}

fn get_long_random_vec(length: &i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec = vec![];
    for _ in 0..*length {
        let random_number = rng.gen_range(1..100000000);
        vec.push(random_number);
    }
    vec
}


fn main() {
    assert_eq!(bubble_sort(vec![5,6,9,3,5]), vec![3,5,5,6,9]);

    let now = Instant::now();
    bubble_sort(get_long_random_vec(&10_000));

    // Sort 10'000 items with bubble sort in 8 seconds
    println!("Bubble sort [{} seconds]", now.elapsed().as_secs());

    let now = Instant::now();
    get_long_random_vec(&10_000_000).sort();

    // Sort 10'000'000 items natively in 10 seconds
    println!("Native sort [{} seconds]", now.elapsed().as_secs());
}

