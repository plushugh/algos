fn main() {
    let mut test = vec![11036,3123,5,99,1,4,7956,567];

    quick_sort(&mut test);

    assert_eq!(test, vec![1,4,5,99,567,3123,7956,11036]);
}

fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[0];
    let mut i = 1;
    let mut j = 1;

    while j < arr.len() {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    arr.swap(0, i - 1);
    quick_sort(&mut arr[..i - 1]);
    quick_sort(&mut arr[i..]);
} 