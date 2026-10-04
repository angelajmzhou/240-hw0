/*
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    return n * 2;
}

pub fn double_v2(n: &i32) -> i32 {
    return n << 1;
}

pub fn double_v3(n: &mut i32) -> i32 {
    return *n << 2;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
// #[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}
#[test]
fn test_double_v3() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    let mut m: usize = n;
    let mut result: usize = 0;
    let mut bit: usize = 1 << (usize::BITS >> 2);
    while bit > m {
        bit >>= 2;
    }
    while bit != 0 {
        if m >= result + bit {
            m -= result + bit;
            result = (result >> 1) + bit;
        } else {
            result >>= 1;
        }
        bit >>= 2;
    }
    result
}

// Remember to write unit tests here (and on all future functions)

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(0), 0);
    assert_eq!(sqrt(1), 1);
    assert_eq!(sqrt(4), 2);
    assert_eq!(sqrt(9), 3);
    assert_eq!(sqrt(16), 4);
    assert_eq!(sqrt(25), 5);
    assert_eq!(sqrt(38), 6);
    assert_eq!(sqrt(55), 7);
    assert_eq!(sqrt(64), 8);
    assert_eq!(sqrt(81), 9);
    assert_eq!(sqrt(100), 10);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?

    I prefer sum_v1 bveacuse it doesn't need to implement the copy trait and is more computationally efficient.
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut result: i32=0;
    for &v in slice {
        result += v;
    }
    result
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut result: i32 = 0;
    for v in slice {
        result += v;
    }
    result
}

#[test]
fn test_sum_v1() {
    assert_eq!(sum_v1(&[]), 0);
    assert_eq!(sum_v1(&[1]), 1);
    assert_eq!(sum_v1(&[1, 2, 3]), 6);
}

#[test]
fn test_sum_v2() {
    assert_eq!(sum_v2(&[]), 0);
    assert_eq!(sum_v2(&[1]), 1);
    assert_eq!(sum_v2(&[1, 2, 3]), 6);
}
/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for &v in slice {
        if !result.contains(&v) {
            result.push(v);
        }
    }
    result
}

#[test]
fn test_unique() {
    assert_eq!(unique(&[]), vec![]);
    assert_eq!(unique(&[1]), vec![1]);
    assert_eq!(unique(&[1, 2, 2, 3]), vec![1, 2, 3]);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for &v in slice {
        if pred(v) {
            result.push(v);
        }
    }
    result
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(out_size);
    if out_size > 0 {
        result.push(n1);
        if out_size > 1 {
            result.push(n2);
        }
    }
    for i in 2..out_size {
        let next = result[i - 1] + result[i - 2];
        result.push(next);
    }
    result
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(0, 1, 0), vec![]);
    assert_eq!(fibonacci(0, 1, 1), vec![0]);
    assert_eq!(fibonacci(0, 1, 2), vec![0, 1]);
    assert_eq!(fibonacci(0, 1, 5), vec![0, 1, 1, 2, 3]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?

    The second function is not effienct because it transfers ownership of the input strings to the function;
    furthermore, if called in a hot loop, the allocated strings will have to be deallocated.
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

#[test]
fn test_str_concat() {
    assert_eq!(str_concat("Hello", " World"), "Hello World");
}

pub fn string_concat(s1: String, s2: String) -> String {
    let mut result = s1;
    result.push_str(&s2);
    result
}

fn test_string_concat() {
    assert_eq!(string_concat("Hello".to_string(), " World".to_string()), "Hello World");
}
/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut result = String::new();
    for s in v {
        result = string_concat(result, s);
    }
    result
}

#[test]
fn test_concat_all() {
    assert_eq!(concat_all(vec!["Hello".to_string(), " World".to_string(), "!".to_string()]), "Hello World!");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for s in v {
        result.push(s.parse().unwrap());
    }
    result
}

#[test]
fn test_parse_all() {
    assert_eq!(parse_all(vec!["1".to_string(), "2".to_string()]), vec![1, 2]);
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for s0 in v {
        let s = format!("{}", s0);
        result.push(s);
    }
    result
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let fibonacci = fibonacci(1, 1, n);
    let is_even = |x: i32| x % 2 == 0;
    let even_fibonacci = filter(&fibonacci, is_even);
    let fib_str = print_all(even_fibonacci);
    concat_all(fib_str)
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
