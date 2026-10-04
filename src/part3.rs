/*
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    Don't need to worry about this in rust because the compiler catches it for you.
    You can't get away with unsafe memory manipulation; it forbids you from 
    borrowing a value as mutable more than once at the same time.

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    std::mem::swap(x1, x2);
}

#[test]
fn test_swap_ints() {
    let mut a = 5;
    let mut b = 10;
    swap_ints(&mut a, &mut b);
    assert_eq!(a, 10);
    assert_eq!(b, 5);
}
// #[test]
// fn test_swap_same() {
//     let mut x = 5;
//     swap_ints(&mut x, &mut x);
//     assert_eq!(x, 5);
// }

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?

/*
    Q1. What went wrong?
    The test fails because `str1` is moved to `str2`, and `str1` is no longer valid after the move.
    String has to be moved because str1 is String type, which doesn't implement copy trait. Thus,
    when we try to use `str1` after the move, the compiler will give an error.
*/

// Q2. How come it works fine here?
/*
    Q2. How come it works fine here?
    The test works fine because integers implement the Copy trait, so when we assign `i1` to `i2`, a copy is made rather than a move.
*/
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    for _ in 0..times {
        result.push(s.to_string());
    }
    result
}

#[test]
fn test_duplicate_string() {
    let s = "foo";
    let result = duplicate_string(s, 3);
    assert_eq!(result, vec!["foo".to_string(), "foo".to_string(), "foo".to_string()]);
}
/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string() -> &'static String {
/*
    The reason why we cannot return a reference to a String created inside the function
    is that the String will be dropped at the end of the function, making the reference invalid --
    even though the data itself is allocated on the heap, the metadata lives on the stack,
    which disappears once the function returns.
*/
// }

fn new_ref_str() -> &'static str {
    return "Hello";
}

//The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

#[test]
fn test_pick_longest2() {
    let s1 = "hello";
    let s2 = "hi";
    assert_eq!(pick_longest2(s1, s2), s1);
}

#[test]
fn test_new_ref_str() {
    assert_eq!(new_ref_str(), "Hello");
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

        It would be the lifetime of the data in the vector,like the following 
        fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str

    Q2. What are the pros and cons of v1 and v2?

    pick_longest_in_v1:	
    - Pros: clean ownership,doesn't need lifetime annotations.
    - Cons: Consumes vector, caller can't use again
    pick_longest_in_v2:	zero allocations, fast, but the entire vector and its contents are still consumed.
    - Pros: zero allocations, which is faster
    - Cons: because the Vec isn't a reference, the container itself is still consumed, meaning the caller cannot use it again.
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    let mut longest = String::new();
    for s in v{
        if s.len() > longest.len() {
            longest = s;
        }
    }
    longest
}

fn pick_longest_in_v2(v: Vec<&str>) -> &str {
   let mut longest: &str = "";
   for s in v{
       if s.len() > longest.len() {
           longest = s;
       }
   }
   longest
}

#[test]
fn test_pick_longest_in_v1() {
    let v = vec!["hello".to_string(), "hi".to_string()];
    assert_eq!(pick_longest_in_v1(v), "hello");
}

#[test]
fn test_pick_longest_in_v2() {
    let v = vec!["hello", "hi"];
    assert_eq!(pick_longest_in_v2(v), "hello");
}
/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?

    I prefer pad_with_zeros_v3 because it modifies the existing vector
    in place. v1 and v3 are the most efficient
    because both can reuse the vector's existing allocation,
    while v2 must allocate a new vector and copy the slice.
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    if v.len() > desired_len {
        panic!("Vector is larger than the desired length");
    }
    let mut result = v;
    result.resize(desired_len, 0);
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    if slice.len() > desired_len {
        panic!("Vector is larger than the desired length");
    }
    let mut result = slice.to_vec();
    result.resize(desired_len, 0);
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    if v.len() > desired_len {
        panic!("Vector is larger than the desired length");
    }
    v.resize(desired_len, 0);
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    if grid.len() == 0 {
        return false
    }
    else if grid[0] == row{
        return true
    }
    false
}

#[test]
fn test_append_row(){
    let mut matrix: Vec<Vec<bool>> = vec![
        vec![true, false, true],
        vec![false, true, true],
    ];
    let result: Vec<Vec<bool>> = vec![
        vec![true, false, true],
        vec![false, true, true],
        vec![true,true,true]
    ];
    let v:Vec<bool> = vec![true,true,true];
    append_row(&mut matrix, v);
    assert_eq!(matrix, result);
}

#[test]
fn test_is_first_row(){
    let matrix: Vec<Vec<bool>> = vec![
        vec![true, false, true],
        vec![false, true, true],
    ];
    let empty: Vec<Vec<bool>> = vec![
    ];
    let v1:Vec<bool> = vec![true,true,true];
    let v2:Vec<bool> = vec![true,false,true];

    assert!(is_first_row(&matrix, &v2));
    assert!(!is_first_row(&matrix, &v1));
    assert!(!is_first_row(&empty, &v1));
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    return v.iter().cloned().collect::<HashMap<i32, String>>();
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    h.retain(|k, _| *k >= 0);
}

#[test]
fn test_vector_to_hashmap() {
    let v = vec![
        (1, "cat".to_string()),
        (2, "dog".to_string()),
    ];

    let hash = vector_to_hashmap(&v);

    let mut test = HashMap::new();
    test.insert(1, "cat".to_string());
    test.insert(2, "dog".to_string());

    assert_eq!(hash, test);
}

#[test]
fn test_delete_negative_keys() {
    let mut h = HashMap::new();
    h.insert(1, 2);
    h.insert(2, 4);
    h.insert(-1, 2);

    delete_negative_keys(&mut h);

    let mut result = HashMap::new();
    result.insert(1, 2);
    result.insert(2, 4);

    assert_eq!(h, result);
}
        
/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k,v) in add{
        merged.entry(k)
        .and_modify(|e| { e.push_str(&v) })
        .or_insert(v);
    }
}

#[test]
fn test_merge_maps() {
    let mut merged = HashMap::new();
    merged.insert("a".to_string(), "cat".to_string());

    let add = {
        let mut m = HashMap::new();
        m.insert("a".to_string(), "dog".to_string());
        m.insert("b".to_string(), "mouse".to_string());
        m
    };

    merge_maps(&mut merged, add);

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), "catdog".to_string());
    expected.insert("b".to_string(), "mouse".to_string());

    assert_eq!(merged, expected);
}
