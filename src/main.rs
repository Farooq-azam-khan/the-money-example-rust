/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

enum Currency {
    Dollar,
    Franc 
}

struct Money {
    amount: i32, 
    currency: Currency 
}

impl Money {
    fn equals(&self, money: Money) -> bool {
        self.amount == money.amount
    }
}

#[derive(PartialEq, Debug)]
struct Dollar {
    amount: i32
}

impl Dollar {
    fn times(&self, multiplier: i32) -> Dollar {
        Dollar { amount: self.amount * multiplier }
    }
}


#[derive(PartialEq, Debug)]
struct Franc {
    amount: i32
}

impl Franc {
    fn times(&self, multiplier: i32) -> Franc {
        Franc { amount: self.amount * multiplier }
    }
}
fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let five = Dollar {amount: 5};
    assert_eq!(Dollar {amount: 10}, five.times(2)); 
    assert_eq!(Dollar {amount: 15}, five.times(3));
}

#[test]
fn test_equality() {
    assert!((Money {amount: 5, currency: Currency::Dollar}).equals(Money {amount: 5, currency: Currency::Dollar}));
    assert!(!(Money {amount: 5, currency: Currency::Dollar}).equals(Money {amount: 6, currency: Currency::Dollar}));
    assert!((Money {amount: 5, currency: Currency::Franc}).equals(Money {amount: 5, currency: Currency::Franc}));
    assert!(!(Money {amount: 5, currency: Currency::Franc}).equals(Money {amount: 6, currency: Currency::Franc}));

}

#[test]
fn test_franc_multiplication() {
    let five = Franc {amount: 5};
    assert_eq!(Franc {amount: 10}, five.times(2));
    assert_eq!(Franc {amount: 15}, five.times(3));
}


