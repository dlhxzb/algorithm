use std::mem::MaybeUninit;
use std::ops::Deref;

pub struct VecArray<T, const N: usize> {
    len: usize,
    array: [MaybeUninit<T>; N],
}

impl<T, const N: usize> VecArray<T, N> {
    pub fn new() -> Self {
        VecArray {
            len: 0,
            array: MaybeUninit::uninit_array(),
        }
    }

    pub fn push(&mut self, data: T) -> Result<(), String> {
        if self.len < N {
            self.array[self.len] = MaybeUninit::new(data);
            self.len += 1;
            Ok(())
        } else {
            Err("VecArray is full".to_owned())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let last = std::mem::replace(&mut self.array[self.len - 1], MaybeUninit::<T>::uninit());
            self.len -= 1;
            // SAFETY: array[len-1] is available
            Some(unsafe { last.assume_init() })
        } else {
            None
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        let len = self.len;
        if new_len < len {
            self.len = new_len;
            // SAFETY: array[..len] is available
            unsafe {
                std::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(
                    &mut self.array[new_len..len],
                ));
            }
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }
}

impl<T, const N: usize> Drop for VecArray<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Deref for VecArray<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        // SAFETY: array[..len] is available
        unsafe { MaybeUninit::slice_assume_init_ref(&self.array[..self.len]) }
    }
}

#[test]
#[should_panic]
fn test_vec_array() {
    let mut arr = VecArray::<u32, 5>::new();
    assert!(arr.pop().is_none());
    arr.push(1).unwrap();
    arr.push(2).unwrap();
    arr.push(3).unwrap();
    arr.push(4).unwrap();
    arr.push(5).unwrap();
    assert!(arr.push(6).is_err());
    assert_eq!(arr[4], 5);
    assert_eq!(arr.pop(), Some(5));
    let v = arr.iter().cloned().collect::<Vec<_>>();
    assert_eq!(v, vec![1, 2, 3, 4]);
    arr[4];
}

#[test]
#[should_panic]
fn test_vec_array_truncate() {
    let mut arr = VecArray::<u32, 5>::new();
    arr.push(1).unwrap();
    arr.push(2).unwrap();
    arr.push(3).unwrap();
    arr.push(4).unwrap();
    arr.push(5).unwrap();
    arr.truncate(4);
    arr[4];
}

/// cargo t test_vec_array_drop -- --nocapture
#[test]
fn test_vec_array_drop() {
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            dbg!("Drop Foo");
        }
    }

    let mut arr = VecArray::<Foo, 5>::new();
    arr.push(Foo).unwrap();
    arr.push(Foo).unwrap();
    arr.push(Foo).unwrap();
}
