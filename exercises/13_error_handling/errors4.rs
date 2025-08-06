#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Using `if`
        // if value > 0 {
        //     Ok(PositiveNonzeroInteger(value as u64))
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Err(CreationError::Negative)
        // }

        // Or 
        // by chatgpt.com
        // match value {
        //     n if n > 0 => Ok(PositiveNonzeroInteger(n as u64)),
        //     0 => Err(CreationError::Zero),
        //     _ => Err(CreationError::Negative),
        // }

        // From sol
        use std::cmp::Ordering;
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
