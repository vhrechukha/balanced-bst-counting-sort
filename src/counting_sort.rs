pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // Find the min and max values in the array
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    let range = (max - min + 1) as usize;
    let mut count = vec![0; range];

    // Store the count of each element
    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    let mut output = vec![0; arr.len()];

    for &num in arr.iter().rev() {
        let index = (num - min) as usize;
        output[count[index] as usize - 1] = num;
        count[index] -= 1;
    }

    arr.copy_from_slice(&output);
}
