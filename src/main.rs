/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

struct Dollar {
    amount: i32
}

impl Dollar {
    fn times(&mut self, multiplier: i32) {
        self.amount = self.amount*2;//10;
    }
}
fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let mut five = Dollar {amount: 5};
    five.times(2);
    assert_eq!(10, five.amount)
}
