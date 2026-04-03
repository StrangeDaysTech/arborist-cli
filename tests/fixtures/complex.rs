fn simple_function() {
    println!("no complexity");
}

fn complex_function(x: i32, y: i32) -> i32 {
    if x > 0 {
        if y > 0 {
            if x > y {
                return x - y;
            } else if y > x {
                return y - x;
            } else {
                return 0;
            }
        } else {
            for i in 0..x {
                if i % 2 == 0 {
                    println!("{}", i);
                } else {
                    println!("odd: {}", i);
                }
            }
        }
    } else if x < 0 {
        match y {
            0 => return -x,
            1..=10 => return x + y,
            _ => return x * y,
        }
    }
    x + y
}

fn moderate_function(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for item in items {
        if *item > 0 {
            result.push(*item * 2);
        } else {
            result.push(0);
        }
    }
    result
}
