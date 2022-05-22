/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

struct Dollar {
    amount: i32
}

impl Dollar {
    fn times(&self, multiplier: i32) -> Dollar {
        Dollar {amount: self.amount*multiplier}
    }
}
fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let mut five = Dollar {amount: 5};
    let prod1 = five.times(2);
    assert_eq!(10, prod1.amount);
    let prod2 = five.times(3);
    assert_eq!(15, prod2.amount);
}
