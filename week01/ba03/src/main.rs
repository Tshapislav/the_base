//! ba3

fn main() {
    let mut v: Vec<String> = Vec::new();
    for arg in std::env::args().skip(1) {
        v.push(arg);
    }

    if v.len() == 0 {
        return;
    }

    quick_sort(&mut v);
    for arg in v {
        println!("{:?}", arg);
    };

}

/// Quick Sort
fn quick_sort<T>(v: &mut[T]) -> &[T]
where
    T: PartialOrd + std::fmt::Debug,
{
    if v.len() <= 1 {
        return v;
    }

    let pivot_index = v.len() - 1;
    let mut border: usize = 0;

    for i in 0..pivot_index {
        if v[i] <= v[pivot_index] {
            v.swap(i, border);
            border += 1;
        };
    }

    v.swap(border, pivot_index);

    let (left, right) = v.split_at_mut(border);
    quick_sort(left);
    quick_sort(&mut right[1..]);

    v
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_two_int_sequence() {
        // Given
        let mut v = vec![2, 1];

        // When
        quick_sort(&mut v);

        // Then
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn sort_multiple_int_sequence() {
        // Given
        let mut v = vec![2, 1, 3, 0];

        // When
        quick_sort(&mut v);

        // Then
        assert_eq!(v, vec![0, 1, 2, 3]);
    }

    #[test]
    fn sort_multiple_string_sequence() {
        // Given
        let mut v = vec![
            "B".to_owned(),
            "A".to_owned(),
            "b".to_owned(),
            "a".to_owned(),
        ];

        // When
        quick_sort(&mut v);

        // Then
        let control_vec = [
            "A".to_owned(),
            "B".to_owned(),
            "a".to_owned(),
            "b".to_owned(),
        ];
        assert_eq!(v, control_vec);
    }
}
