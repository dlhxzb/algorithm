use std::fmt::Debug;

// 递归版
pub fn quick_sort<T: Ord + Debug>(list: &mut [T]) {
    if let Some(idx) = partition(list) {
        quick_sort(&mut list[..idx]);
        quick_sort(&mut list[idx..]);
    }
}

// 迭代版，自上而下地分割排序
pub fn quick_sort_no_recursion<T: Ord + Debug>(list: &mut [T]) {
    let mut ranges = vec![(0, list.len())];
    // 因为不能[&mut list[..],&mut list[..]]，所以将分割后的数组范围存入ranges
    while !ranges.is_empty() {
        ranges = ranges
            .into_iter()
            .filter_map(|(start, end)| {
                partition(&mut list[start..end])
                    .map(|idx| [(start, start + idx), (start + idx, end)])
            })
            .flatten()
            .collect();
    }
}

// 按首元素大小左右分割，返回分割后首元素位置
fn partition<T: Ord + Debug>(list: &mut [T]) -> Option<usize> {
    let len = list.len();
    if len <= 1 {
        return None;
    }
    let (mut i, mut j) = (1, len - 1);
    while i < j {
        // [i] > [0] > [j]时，交换[i],[j]
        if list[i] > list[0] && list[j] < list[0] {
            list.swap(i, j);
        }
        // [i]左侧都要<[0]
        if list[i] < list[0] {
            i += 1;
        }
        // [j]右侧都要>=[0]，避免i,j都不步进，卡在==[0]
        if list[j] >= list[0] {
            j -= 1;
        }
    }
    // [j]值经过-=后不确定是否<[0]
    if list[j] < list[0] {
        list.swap(0, j);
    }
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
