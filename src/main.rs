/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

#[derive(PartialEq, Copy, Clone, Debug)]
enum Currency {
    Dollar,
    Franc 
}

#[derive(PartialEq, Debug)]
struct Money {
    amount: i32, 
    currency: Currency 
}

impl Money {
    fn times(&self, multiplier: i32) -> Money {
        Money { amount: self.amount * multiplier, currency: self.currency }
    }

    fn equals(&self, money: Money) -> bool {
        self.amount == money.amount
    }
}



fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let five = Money { amount: 5, currency: Currency::Dollar };
    assert_eq!(Money { amount: 10, currency: Currency::Dollar }, five.times(2)); 
    assert_eq!(Money { amount: 15, currency: Currency::Dollar }, five.times(3));
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
    let five = Money {amount: 5, currency: Currency::Franc};
    assert_eq!(Money {amount: 10, currency: Currency::Franc}, five.times(2));
    assert_eq!(Money {amount: 15, currency: Currency::Franc}, five.times(3));
}


