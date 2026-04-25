use std::collections::HashMap;

fn order_vec(mut v: Vec<i32>) -> Vec<i32> {
    // Take in a Vec<i32> and then sort them using reverse iteration
    // where we look at j and j-1 and if j-1 >= j then we break the
    // reverse loop, as the previous section was already ordered
    use std::cmp::Ordering;

    // Iter through our values from the front to the back
    for i in  1..v.len() {

        // Check that there is nothing larger then our previous elements
        for j in (0..i-1).rev() {
            match v[j+1].cmp(&v[j]) {
                Ordering::Less => {
                    let prev: i32 = v[j];
                    v[j] = v[j+1];
                    v[j+1] = prev;
                }
                // If j+1 is >= j then we can stop going backwards
                // as we have already sorted that area
                Ordering::Greater | Ordering::Equal => break,
            }
        }
    }
    return v
}

fn count_occurences_report_top(v: &Vec<i32>) -> i32 {
    // Pass in a Vec<i32> and iterate through the vector
    // and report the top number that appears in our Vec
    let mut count: HashMap<i32, u32> = HashMap::new();

    for num in v {
        let count_i = count.entry(*num).or_insert(0);
        *count_i += 1;
    }

    let mut max_v: u32 = 0;
    let mut max_k: i32 = 0;
    for (k, v) in &count {
        if *v > max_v {
            max_k = *k;
            max_v = *v;
        }
    }
    return max_k
}

fn return_median(v: &Vec<i32>) -> i32 {
    // Grab the median value from a Vec<i32>
    let mut vec_len: usize = v.len();
    let denomin: usize = 2;
    let middle_value: usize;
    if vec_len % 2 != 0 {
        vec_len += 1
    }
    middle_value = vec_len / denomin;
    return v[middle_value]
}

fn main() {
    let full_vec: Vec<i32> = vec![0,3,5,6,3,2,6,7,3,2,4,5];
    println!("Unordered Vector: {full_vec:?}");
    let ordered_vec: Vec<i32> = order_vec(full_vec);
    println!("Ordered Vector: {ordered_vec:?}");
    let mode: i32 = count_occurences_report_top(&ordered_vec);
    println!("Mode: {}", mode);
    let median: i32 = return_median(&ordered_vec);
    println!("Median: {}", median);
}
