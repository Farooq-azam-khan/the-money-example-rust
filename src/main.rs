/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

#[derive(PartialEq, Debug)]
struct Dollar {
    amount: i32
}

impl Dollar {
    fn times(&self, multiplier: i32) -> Dollar {
        Dollar {amount: self.amount*multiplier}
    }

    fn equals(&self, dollar: Dollar) -> bool {
        self.amount == dollar.amount
    }
}
fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let five = Dollar {amount: 5};
    let prod1 = five.times(2);
    //assert_eq!(10, prod1.amount);
    assert_eq!(Dollar {amount: 10}, five.times(2)); // prod1);
    let prod2 = five.times(3);
    //assert_eq!(15, prod2.amount);
    assert_eq!(Dollar {amount: 15}, five.times(3)); // prod2);
}

#[test]
fn test_equality() {
    assert!((Dollar {amount: 5}).equals(Dollar {amount: 5}));
    //assert_eq!(Dollar {amount: 5}, Dollar {amount: 5}); // implement partial equal (builtin
    //equality)
    assert!(!(Dollar {amount: 5}).equals(Dollar {amount: 6}));
}
