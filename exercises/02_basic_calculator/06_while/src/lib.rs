use std::iter::Sum;

// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    let mut sum: u32 = 1;
    let mut i: u32 = n;

    print!("this is {}！", i);

    if i == 0 { 
        print!("result {}!", 1);
        return 1;
    }

    while i != 0 {
        sum = sum * i;
        i = i - 1; 
    }
    print!("result {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
