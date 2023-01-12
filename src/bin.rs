pub mod ast;
pub mod ast_tests;
pub mod tokenizer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn type_constructors() {}
}

pub fn main() {
    println!("Hello, world!");
}
