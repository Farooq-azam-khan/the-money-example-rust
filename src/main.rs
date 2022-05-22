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

    fn dollar(amount: i32) -> Money {
        Money { currency: Currency::Dollar, amount }
    }

    fn franc(amount: i32) -> Money {
        Money { currency: Currency::Franc, amount }
    }

    fn equals(&self, money: Money) -> bool {
        self.amount == money.amount && self.currency == money.currency
    }

    fn plus(&self, money: Money) -> Money {
        Money { amount: self.amount + money.amount, currency: self.currency }
    }
}


fn main() {
    println!("Hello, world!");
}


#[test]
fn test_multiplication() {
    let five = Money::dollar(5);
    assert_eq!(Money::dollar(10), five.times(2)); 
    assert_eq!(Money::dollar(15), five.times(3));
}

#[test]
fn test_equality() {
    assert!(Money::dollar(5).equals(Money::dollar(5)));
    assert!(!Money::dollar(5).equals(Money::dollar(6)));
    assert!(Money::franc(5).equals(Money::franc(5)));
    assert!(!Money::franc(5).equals(Money::franc(6)));

    // franc with dollars
    assert!(!Money::franc(5).equals(Money::dollar(5)));

}

#[test]
fn test_franc_multiplication() {
    let five = Money::franc(5);
    assert_eq!(Money::franc(10), five.times(2));
    assert_eq!(Money::franc(15), five.times(3));
}

#[test]
fn test_simple_addition() {
    let sum = Money::dollar(5).plus(Money::dollar(5));
    assert_eq!(Money::dollar(10), sum);
}
