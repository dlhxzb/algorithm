use std::fmt::Debug;

// 递归版
pub fn quick_sort<T: Ord + Debug>(list: &mut [T]) {
    quick_sort_inner(list, 0, list.len() - 1);
}

fn quick_sort_inner<T: Ord + Debug>(list: &mut [T], start: usize, end: usize) {
    if start >= end {
        return;
    }
    if let Some(idx) = partition(list, start, end) {
        if idx > start {
            quick_sort_inner(list, start, idx - 1);
        }
        quick_sort_inner(list, idx + 1, end);
    }
}

// 迭代版
pub fn quick_sort_no_recursion<T: Ord + Debug>(list: &mut [T]) {
    let mut ranges = vec![(0, list.len() - 1)];
    while !ranges.is_empty() {
        ranges = ranges
            .into_iter()
            .filter_map(|(start, end)| {
                partition(list, start, end).map(|idx| {
                    if idx > start {
                        vec![(start, idx - 1), (idx + 1, end)]
                    } else {
                        vec![(idx + 1, end)]
                    }
                })
            })
            .flatten()
            .collect();
    }
}

// 按首元素大小左右分割，返回分割后首元素位置
fn partition<T: Ord + Debug>(list: &mut [T], start: usize, end: usize) -> Option<usize> {
    if start >= end {
        return None;
    }
    let (mut i, mut j) = (start, end);
    while i < j {
        // 从后向前找到第一个小于start的，到start为止
        while list[j] >= list[start] && j > start {
            j -= 1;
        }
        // 从前向后找到第一个大于start的，到j为止
        while list[i] <= list[start] && i < j {
            i += 1;
        }
        // 交换大于小于值，j没找到的话i=j=start，i没找到的话i=j
        list.swap(i, j);
    }
    // [j]值<=start，就是start该替换的位置
    list.swap(start, j);
    Some(j)
}

#[test]
fn test_quick_sort() {
    let mut arr = [2, 9, 5, 7, 6, 3, 8, 4, 6, 1];
    quick_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}

#[test]
fn test_quick_sort_no_recursion() {
    let mut arr = [2, 9, 5, 7, 6, 3, 8, 4, 6, 1];
    quick_sort_no_recursion(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}
