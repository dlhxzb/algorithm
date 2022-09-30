use std::fmt::Debug;

pub fn shell_sort<T: Ord + Debug>(list: &mut [T]) {
    let len = list.len();
    let mut step = len;
    while step > 1 {
        step >>= 1;
        // 0..step为每组第一个值不需要排序，直接插入
        for i in step..len {
            let mut j = i;
            // 当前值之前已排序，从当前值向前，比它大的都向后窜
            // 2,3,4,`1` -> `1`,2,3,4
            while j >= step && list[j] < list[j - step] {
                list.swap(j, j - step);
                j -= step;
            }
        }
    }
}

#[test]
fn test_shell_sort() {
    let mut arr = [2, 9, 5, 7, 6, 3, 8, 4, 6, 1];
    shell_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}
