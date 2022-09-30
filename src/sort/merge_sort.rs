use std::collections::VecDeque;
use std::fmt::Debug;

// 递归版
pub fn merge_sort<T: Ord + Debug>(mut list: VecDeque<T>) -> VecDeque<T> {
    if list.len() < 2 {
        return list;
    }
    let mut right = list.split_off(list.len() >> 1);
    // 递归分割到2个一组，再开始排序
    list = merge_sort(list);
    right = merge_sort(right);
    merge(list, right)
}

// 迭代版
pub fn merge_sort_no_recursion<T: Ord + Debug>(mut list: VecDeque<T>) -> VecDeque<T> {
    if list.len() <= 1 {
        return list;
    }
    let mut step = 1;
    // 从1个元素一组开始，分组排序，分组数量循环倍增
    while step < list.len() {
        let mut sorted = VecDeque::with_capacity(list.len());
        // 按step切割分组，两两组merge排序，排好的进sorted
        while !list.is_empty() {
            // 够分两组
            if list.len() > step {
                let mut list1 = list;
                list = list1.split_off(step);
                let mut list2 = list;
                list = list2.split_off(std::cmp::min(list2.len(), step));
                sorted.append(&mut merge(list1, list2));
            } else {
                // 只够一组
                sorted.append(&mut list);
            }
        }
        list = sorted;
        step <<= 1;
    }
    list
}

fn merge<T: Ord + Debug>(mut list1: VecDeque<T>, mut list2: VecDeque<T>) -> VecDeque<T> {
    let mut merged = VecDeque::with_capacity(list1.len() + list2.len());
    loop {
        match (list1.front(), list2.front()) {
            (Some(front1), Some(front2)) => {
                if front1 < front2 {
                    merged.push_back(list1.pop_front().unwrap());
                } else {
                    merged.push_back(list2.pop_front().unwrap())
                }
            }
            (Some(_), None) => {
                merged.append(&mut list1);
                break;
            }
            (None, Some(_)) => {
                merged.append(&mut list2);
                break;
            }
            (None, None) => break,
        }
    }
    return merged;
}

#[test]
fn test_merge_sort() {
    let arr = [2, 9, 5, 7, 6, 3, 8, 4, 6, 1].into();
    let arr = merge_sort(arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}

#[test]
fn test_merge_sort_no_recursion() {
    let arr = [2, 9, 5, 7, 6, 3, 8, 4, 6, 1].into();
    let arr = merge_sort_no_recursion(arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}
