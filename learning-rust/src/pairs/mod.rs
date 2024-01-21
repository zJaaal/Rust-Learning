pub fn pairs(arr: &Vec<i32>) -> usize {
    let mut count = 0;

    for chunk in arr.chunks(2) {
        if chunk.len() == 2 && (chunk[0] - chunk[1]).abs() == 1 {
            count += 1;
        }
    }

    // Alt  xs.chunks_exact(2).filter(|xy| (xy[0] - xy[1]).abs() == 1).count()

    count
}
