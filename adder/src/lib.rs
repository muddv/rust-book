pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        // assert_eq! and assert_ne! are part of standard lib because 
        // they are common scenarios in assert tests
        assert_eq!(result, 4);
    }
    
   // #[test]
   // fn another() {
   //     // Test fails when code panics
   //     panic!("Make this test fail");
   // }
}
