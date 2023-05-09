use std::fmt::Debug;

// 为所有权转移到小顶堆方便，使用Vec非[]
pub fn heap_sort<T: Ord + Debug>(list: &mut Vec<T>) {
    let mut q = Vec::with_capacity(list.len());
    while let Some(data) = list.pop() {
        insert_end(&mut q, data);
    }
    while let Some(data) = delete_top(&mut q) {
        list.push(data)
    }
}

fn insert_end<T: Ord + Debug>(q: &mut Vec<T>, data: T) {
    q.push(data);
    let mut new_idx = q.len() - 1;
    while new_idx > 0 {
        let parent = (new_idx - 1) / 2;
        if q[new_idx] < q[parent] {
            q.swap(new_idx, parent);
            new_idx = parent;
        } else {
            break;
        }
    }
}

fn delete_top<T: Ord + Debug>(q: &mut Vec<T>) -> Option<T> {
    if q.is_empty() {
        return None;
    }
    let len = q.len() - 1;
    // 首尾互换，避免移除顶部后窜行导致叶子节点比父节点大
    q.swap(0, len);
    let top = q.pop();
    let mut idx = 0;
    // 左叶子节点存在
    while idx * 2 + 1 < len {
        let left = idx * 2 + 1;
        let right = left + 1;
        if right < len && q[right] < q[left] && q[right] < q[idx] {
            q.swap(right, idx);
            idx = right;
        } else if q[left] < q[idx] {
            q.swap(left, idx);
            idx = left;
        } else {
            // idx比左右叶子都小
            break;
        }
    }
    dbg!(q);

    top
}

#[test]
fn test_heap_sort() {
    let mut arr = vec![2, 9, 5, 7, 6, 3, 8, 4, 6, 1];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9]);
}
