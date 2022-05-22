/* 
 1. Add amounts in two different currencies and convert the result to a set of exchange rates
 2. Need to be able to multiply an amount (price per share) by a number (number of shares) and receive an amount
*/

#[derive(PartialEq, Copy, Clone, Debug)]
enum Currency {
    Dollar,
    Franc 
}

struct Bank {}

impl Bank {
    fn reduce(&self, value: impl Expression, currency: Currency) -> Money {
        value.reduce(currency)
    }

    fn add_rate(&self, c1: Currency, c2: Currency, ratio: i32) {}
}

trait Expression {
    fn plus(&self, money: Money) -> Sum; 
    fn reduce(&self, to: Currency) -> Money; 
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Money {
    amount: i32, 
    currency: Currency 
}

#[derive(Debug)]
struct Sum {
    augend: Money,
    addend: Money
}

impl Expression for Sum {
    fn plus(&self, m: Money) -> Sum {
        Sum { augend: self.augend, addend: self.addend }
    }

    fn reduce(&self, to: Currency) -> Money {
        Money { amount: self.augend.amount + self.addend.amount, currency: to }
    }
}

impl Expression for Money {
    fn plus(&self, money: Money) -> Sum {
        Sum { augend: Money { amount: self.amount, currency: self.currency }, addend: money }
    }

    fn reduce(&self, to: Currency) -> Money {
        match (self.currency, to) {
            (Currency::Dollar, Currency::Dollar) => *self, 
            (Currency::Franc, Currency::Franc) => *self, 
            (Currency::Dollar, Currency::Franc) => Money { amount: self.amount / 1, currency: to }, 
            (Currency::Franc, Currency::Dollar) => Money { amount: self.amount / 2, currency: to }, 
            }
        }
    
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
    let five = Money::dollar(5); 
    let sum = five.plus(Money::dollar(5));  
    let bank = Bank {}; 
    let reduced = bank.reduce(sum, Currency::Dollar);
    assert_eq!(Money::dollar(10), reduced);
}

#[test]
fn test_plus_returns_sum() {
    let five = Money::dollar(5);
    let sum: Sum = five.plus(Money::dollar(5)); 
    assert_eq!(five, sum.augend); 
    assert_eq!(five, sum.addend); 
}

#[test]
fn test_reduce_sum() {
    let sum = Sum { augend: Money::dollar(3), addend: Money::dollar(4) };
    let bank = Bank {};
    let result = bank.reduce(sum, Currency::Dollar);
    assert_eq!(Money::dollar(7), result); 
}

#[test]
fn test_reduce_money() {
    let bank = Bank {};
    let result = bank.reduce(Money::dollar(1), Currency::Dollar);
    assert_eq!(Money::dollar(1), result);
}

#[test]
fn test_reduce_money_different_currency() {
    let bank = Bank {}; 
    bank.add_rate(Currency::Franc, Currency::Dollar, 2); 
    let result = bank.reduce(Money::franc(2), Currency::Dollar); 
    assert_eq!(Money::dollar(1), result); 
}
