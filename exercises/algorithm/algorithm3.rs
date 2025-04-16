/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sift_down<T>(array: &mut [T], n: usize, mut i: usize)
    where T: Copy + std::cmp::PartialOrd {
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut ma = i;

        if l < n && array[l] > array[ma] {
            ma = l;
        }

        if r < n && array[r] > array[ma] {
            ma = r;
        }

        if ma == i {
            break
        }

        array.swap(i, ma);
        i = ma;
    }
}


fn sort<T>(array: &mut [T])
    where T: std::cmp::PartialOrd + Copy {
	//TODO
    // for i in 0..(array.len()) {
    //     for j in 0..(array.len()-i-1) {
    //         if array[j] > array[j+1] {
    //             array.swap(j+1, j)
    //         }
    //     }
    // }

    // for i in 1..array.len() {
    //     let mut j = i;
    //     while j > 0 && array[j-1] > array[j] {
    //         array.swap(j-1, j);
    //         j -= 1;
    //     }
    // }

    for i in (0..(array.len() / 2)).rev() {
        sift_down(array, array.len(), i);
    }

    for i in (1 .. (array.len())).rev() {
        array.swap(0, i);
        sift_down(array, i, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}