// [Hard]
// Given an array of integers, return a new array such that each element at index i of the new array
// is the product of all the numbers in the original array except the one at i.
// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30,24].
// If our input was [3, 2, 1], the expected output would be [2, 3, 6].
// Follow-up: what if you can't use division?

pub fn simple_solution(input_vec: Vec<i32>) -> Vec<i32> {
    if input_vec.is_empty() {
        return Vec::<i32>::new();
    }

    let mut number_of_zeroes = 0;
    let mut product: i32 = 1;

    input_vec.iter().for_each(|x| {
        if *x != 0 {
            product *= x
        } else {
            number_of_zeroes += 1;
        }
    });

    if number_of_zeroes == input_vec.len() {
        return input_vec;
    }

    let output_vec: Vec<i32> = input_vec
        .iter()
        .map(|x| -> i32 {
            if number_of_zeroes == 0 {
                return product / x;
            } else if number_of_zeroes == 1 && *x == 0 {
                return product;
            } else {
                return 0;
            }
        })
        .collect();
    return output_vec;
}

// No good idea yet except more iterations...
// pub fn follow_up_solution(_input_vec: Vec<i32>) -> Vec<i32> {
//     return Vec::<i32>::new();
// }

#[cfg(test)]
mod simple_solution_tests {
    use super::*;

    #[test]
    fn given_example_1() {
        assert_eq!(
            simple_solution(vec![1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );
    }

    #[test]
    fn given_example_2() {
        assert_eq!(simple_solution(vec![3, 2, 1]), vec![2, 3, 6]);
    }

    #[test]
    fn only_one_list_item() {
        assert_eq!(simple_solution(vec![1]), vec![1]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(simple_solution(vec![]), vec![]);
    }

    #[test]
    fn input_contains_one_zero() {
        assert_eq!(simple_solution(vec![0, 1, 2, 3]), vec![6, 0, 0, 0]);
    }

    #[test]
    fn input_contains_multiple_zeroes() {
        assert_eq!(simple_solution(vec![0, 1, 0, 3]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn input_contains_only_zeroes() {
        assert_eq!(simple_solution(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn input_contains_only_one_zero() {
        assert_eq!(simple_solution(vec![0]), vec![0]);
    }
}
