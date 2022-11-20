pub fn factorial_recursive(num: i32) -> i32 {
    if num <= 1 {
        return 1;
    } else {
        return factorial_recursive(num - 1)
            + factorial_recursive(num - 2);
    }
}