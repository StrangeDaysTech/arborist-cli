pub fn compute(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn transform(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}
