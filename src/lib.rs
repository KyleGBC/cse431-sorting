#[inline]
pub fn merge_sort<T: PartialOrd + Clone>(data: &mut [T]) {
    let len = data.len();
    if len > 1 {
        // Divide at midpoint
        let mid = len / 2;
        let (left_slice, right_slice) = data.split_at(mid);

        // Copy data to temp buffers
        let (mut left, mut right) = (Vec::new(), Vec::new());
        left.extend_from_slice(left_slice);
        right.extend_from_slice(right_slice);

        // Recursively sort each
        merge_sort(&mut left);
        merge_sort(&mut right);

        // Copy data back to source buffer
        let (mut l, mut r, mut i) = (0, 0, 0);
        while l < left.len() && r < right.len() {
            if left[l] < right[r] {
                data[i] = left[l].clone();
                l += 1;
            } else {
                data[i] = right[r].clone();
                r += 1;
            }
            i += 1;
        }

        // Copy any remaining data
        if l < left.len() {
            data[i..].clone_from_slice(&left[l..]);
        } else if r < right.len() {
            data[i..].clone_from_slice(&right[r..]);
        }
    }
}

#[inline]
pub fn insertion_sort<T: PartialOrd>(data: &mut [T]) {
    // Iterate over unsorted elements
    for i in 1..data.len() {
        // Swap current unsorted value into place
        let mut search_idx = i;
        while search_idx > 0 && data[search_idx - 1] > data[search_idx] {
            data.swap(search_idx - 1, search_idx);
            search_idx -= 1;
        }
    }
}
