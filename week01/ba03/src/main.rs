//! ba3

fn main() {
    let mut v: Vec<String> = Vec::new();
    for arg in std::env::args().skip(1) {
        v.push(arg);
    }

    if v.len() == 0 {
        return;
    }

    let sorted_v = quick_sort(v);
    for arg in sorted_v {
        println!("{:?}", arg);
    };

}

/// Quick Sort
fn quick_sort<T>(mut v: Vec<T>) -> Vec<T>
where
    T: PartialOrd,
{
    if v.len() < 1 {
        return v;
    }

    let pivot = v.pop().unwrap();
    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();

    for element in v {
        if element <= pivot {
            left.push(element);
        } else {
            right.push(element);
        }
    }

    let mut sorted = quick_sort(left);
    sorted.push(pivot);
    sorted.extend(quick_sort(right));
    sorted
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_sequence() {
        // Given
        let v: Vec<i32> = Vec::new();

        // When
        let sorted_vec = quick_sort(v);

        // Then
        assert_eq!(sorted_vec, Vec::new());
    }

    #[test]
    fn sort_two_int_sequence() {
        // Given
        let v = vec![2, 1];

        // When
        let sorted_vec = quick_sort(v);

        // Then
        assert_eq!(sorted_vec, vec![1, 2]);
    }

    #[test]
    fn sort_multiple_int_sequence() {
        // Given
        let v = vec![2, 1, 3, 0];

        // When
        let sorted_vec = quick_sort(v);

        // Then
        assert_eq!(sorted_vec, vec![0, 1, 2, 3]);
    }

    #[test]
    fn sort_multiple_string_sequence() {
        // Given
        let v = vec![
            "B".to_owned(),
            "A".to_owned(),
            "b".to_owned(),
            "a".to_owned(),
        ];

        // When
        let sorted_vec = quick_sort(v);

        // Then
        let control_vec = [
            "A".to_owned(),
            "B".to_owned(),
            "a".to_owned(),
            "b".to_owned(),
        ];
        assert_eq!(sorted_vec, control_vec);
    }
}
