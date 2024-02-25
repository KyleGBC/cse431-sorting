use sort_comparison::{merge_sort, insertion_sort};

fn main()
{
    let mut data = [5, 4, 3, 2, 7];
    merge_sort(&mut data);

    let mut data2 = [5, 4, 3, 2, 7];
    insertion_sort(&mut data2);
}