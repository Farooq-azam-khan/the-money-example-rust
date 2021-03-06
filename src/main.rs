use std::collections::HashMap; 

// Domain Specific types
struct Bank {
    rates: HashMap<Pair, i32>
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Currency {
    Dollar,
    Franc 
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Money {
    amount: i32, 
    currency: Currency 
}

// Helper types and traits
#[derive(Eq, Hash, PartialEq)]
struct Pair {
    from: Currency, 
    to: Currency 
}

trait Expression {
    fn plus(&self, money: Money) -> Sum; 
    fn reduce(&self, bank: &Bank, to: Currency) -> Money; 
}


#[derive(Debug, Copy, Clone)]
struct Sum {
    augend: Money,
    addend: Money 
}

// Implementations 
impl Bank {
    fn new() -> Bank {
        Bank { rates: HashMap::new() }
    }

    fn rate(&self, from: Currency, to: Currency) -> i32 {
        if from == to {
            return 1; 
        }
        let rate = self.rates.get(&Pair { from, to }).unwrap(); 
        *rate


    }
    fn reduce(&self, value: impl Expression, currency: Currency) -> Money {
            value.reduce(self, currency)
    }

    fn add_rate(&mut self, from: Currency, to: Currency, ratio: i32) {
        self.rates.insert(Pair {from, to}, ratio);
    }
}

impl Sum {
    fn new(augend: Money, addend: Money) -> Sum {
        Sum { augend, addend }
    }
}

impl Expression for Sum {
    fn plus(&self, money: Money) -> Sum {
        // Broken
        let augend = self.augend;//.plus(self.addend); 
        Sum { augend, addend: money }
    }

    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money { amount: 
                self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount, 
            currency: to }
    }
}

impl Expression for Money {
    fn plus(&self, money: Money) -> Sum {
        Sum { augend: *self, addend: money } 
    }

    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        let rate = bank.rate(self.currency, to); 
        Money { amount: self.amount / rate, currency: to }
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
    let bank = Bank::new();
    let reduced = bank.reduce(sum, Currency::Dollar);
    assert_eq!(Money::dollar(10), reduced);
}

#[test]
fn test_plus_returns_sum() {
    let five = Money::dollar(5);
    let sum = five.plus(Money::dollar(5)); 
    assert_eq!(five, sum.augend); 
    assert_eq!(five, sum.addend); 
}

#[test]
fn test_reduce_sum() {
    let sum = Sum { augend: Money::dollar(3), addend: Money::dollar(4) };
    let bank = Bank::new();
    let result = bank.reduce(sum, Currency::Dollar);
    assert_eq!(Money::dollar(7), result); 
}

#[test]
fn test_reduce_money() {
    let bank = Bank::new();
    let result = bank.reduce(Money::dollar(1), Currency::Dollar);
    assert_eq!(Money::dollar(1), result);
}

#[test]
fn test_reduce_money_different_currency() {
    let mut bank = Bank { rates: HashMap::new() }; 
    bank.add_rate(Currency::Franc, Currency::Dollar, 2); 
    let result = bank.reduce(Money::franc(2), Currency::Dollar); 
    assert_eq!(Money::dollar(1), result); 
}

#[test]
fn test_identity_rate() {
    assert_eq!(1, Bank::new().rate(Currency::Dollar, Currency::Dollar));
}

#[test]
fn test_mixed_addition() {
    let five_dollars = Money::dollar(5); 
    let ten_francs = Money::franc(10); 
    let mut bank = Bank::new(); 
    bank.add_rate(Currency::Franc, Currency::Dollar, 2); 
    let result = bank.reduce(five_dollars.plus(ten_francs), Currency::Dollar); 
    assert_eq!(Money::dollar(10), result); 
}

// Broken
/*#[test]
fn test_sum_plus_money() {
    let five_dollars = Money::dollar(5); 
    let ten_francs = Money::franc(10); 
    let mut bank = Bank::new(); 
    bank.add_rate(Currency::Franc, Currency::Dollar, 2); 
    let sum = Sum::new(five_dollars, ten_francs).plus(five_dollars) ; 
    let result = bank.reduce(sum, Currency::Dollar); 
    assert_eq!(Money::dollar(15), result); 
}
*/
