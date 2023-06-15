pub fn check_if_addable_to_k(k: i32, list: &[i32]) -> bool {
    return list
        .iter()
        .filter(|&x: &&i32| list.contains(&(k - x)))
        .take(1)
        .collect::<Vec<&i32>>()
        .len()
        > 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example() {
        assert_eq!(check_if_addable_to_k(17, &[10, 15, 3, 7]), true);
    }

    #[test]
    fn should_return_false() {
        assert_eq!(check_if_addable_to_k(17, &[10, 15, 3, 8]), false);
    }
}
