# Operator Overloading: A Brief Introduction

## What is Operator Overloading?

**Operator overloading** is the ability to define custom behavior for standard operators (`+`, `-`, `*`, `/`, `[]`, `==`, etc.) when applied to your own types.

Instead of writing:
```rust
let total = money1.add(money2);
```

You can write:
```rust
let total = money1 + money2;
```

This makes code more **intuitive** and **readable** by using familiar mathematical notation.

---

## Why Use Operator Overloading?

### Benefits
- **Natural syntax**: `a + b` is clearer than `a.add(b)`
- **Domain modeling**: Models real-world concepts (money, vectors, matrices)
- **Consistency**: Works like built-in types
- **Expressiveness**: Complex operations become readable

### Caution
- Don't overload operators in unexpected ways (e.g., `+` for deletion)
- Keep behavior intuitive and predictable

---

## Example: Money Struct

Let's build a `Money` struct that supports various operations.

### Basic Structure

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Money {
    amount: f64,
    currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Currency {
    USD,
    EUR,
    GBP,
    ARS,
}

impl Money {
    fn new(amount: f64, currency: Currency) -> Self {
        Money { amount, currency }
    }
}
```

---

## 1. Addition Operator (`+`)

**Use case**: Adding money of the same currency

```rust
use std::ops::Add;

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        if self.currency != other.currency {
            panic!("Cannot add money with different currencies: {:?} and {:?}", 
                   self.currency, other.currency);
        }
        Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

// Usage:
let price1 = Money::new(50.0, Currency::USD);
let price2 = Money::new(30.0, Currency::USD);
let total = price1 + price2;  // Money { amount: 80.0, currency: USD }
```

---

## 2. Subtraction Operator (`-`)

**Use case**: Calculating change or differences

```rust
use std::ops::Sub;

impl Sub for Money {
    type Output = Money;

    fn sub(self, other: Money) -> Money {
        if self.currency != other.currency {
            panic!("Cannot subtract money with different currencies");
        }
        Money {
            amount: self.amount - other.amount,
            currency: self.currency,
        }
    }
}

// Usage:
let payment = Money::new(100.0, Currency::EUR);
let cost = Money::new(65.0, Currency::EUR);
let change = payment - cost;  // Money { amount: 35.0, currency: EUR }
```

---

## 3. Multiplication Operator (`*`)

**Use case**: Multiplying money by a scalar (quantity)

```rust
use std::ops::Mul;

// Money * f64
impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, multiplier: f64) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }
}

// f64 * Money (reverse order)
impl Mul<Money> for f64 {
    type Output = Money;

    fn mul(self, money: Money) -> Money {
        Money {
            amount: money.amount * self,
            currency: money.currency,
        }
    }
}

// Usage:
let unit_price = Money::new(25.0, Currency::GBP);
let total = unit_price * 3.0;     // Money { amount: 75.0, currency: GBP }
let total2 = 3.0 * unit_price;    // Money { amount: 75.0, currency: GBP }
```

---

## 4. Division Operator (`/`)

**Use case**: Splitting money or calculating unit price

```rust
use std::ops::Div;

impl Div<f64> for Money {
    type Output = Money;

    fn div(self, divisor: f64) -> Money {
        if divisor == 0.0 {
            panic!("Cannot divide money by zero");
        }
        Money {
            amount: self.amount / divisor,
            currency: self.currency,
        }
    }
}

// Usage:
let total = Money::new(100.0, Currency::USD);
let per_person = total / 4.0;  // Money { amount: 25.0, currency: USD }
```

---

## 5. Comparison Operators (`==`, `<`, `>`, etc.)

**Use case**: Comparing money values

```rust
use std::cmp::Ordering;

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            None  // Cannot compare different currencies
        } else {
            self.amount.partial_cmp(&other.amount)
        }
    }
}

// Usage:
let price1 = Money::new(50.0, Currency::USD);
let price2 = Money::new(30.0, Currency::USD);

if price1 > price2 {
    println!("price1 is more expensive");
}
```

---

## 6. Index Operator (`[]`)

**Use case**: Accessing money by currency in a collection

```rust
use std::ops::Index;

struct Wallet {
    balances: Vec<Money>,
}

impl Index<Currency> for Wallet {
    type Output = Money;

    fn index(&self, currency: Currency) -> &Self::Output {
        self.balances
            .iter()
            .find(|m| m.currency == currency)
            .unwrap_or_else(|| panic!("No balance found for {:?}", currency))
    }
}

// Usage:
let wallet = Wallet {
    balances: vec![
        Money::new(100.0, Currency::USD),
        Money::new(50.0, Currency::EUR),
    ],
};

let usd_balance = &wallet[Currency::USD];  // &Money { amount: 100.0, currency: USD }
```

---

## 7. Display Operator (fmt)

**Use case**: Pretty printing money

```rust
use std::fmt;

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self.currency {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::ARS => "ARS$",
        };
        write!(f, "{}{:.2}", symbol, self.amount)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Usage:
let price = Money::new(42.50, Currency::USD);
println!("{}", price);  // Output: $42.50
```

---

## Complete Example: Using All Operators Together

```rust
fn main() {
    // Create money instances
    let salary = Money::new(5000.0, Currency::USD);
    let rent = Money::new(1500.0, Currency::USD);
    let groceries = Money::new(300.0, Currency::USD);
    
    // Subtraction
    let after_rent = salary - rent;
    println!("After rent: {}", after_rent);  // After rent: $3500.00
    
    // Multiple operations
    let remaining = after_rent - groceries;
    println!("After groceries: {}", remaining);  // After groceries: $3200.00
    
    // Multiplication
    let monthly_savings = Money::new(500.0, Currency::USD);
    let yearly_savings = monthly_savings * 12.0;
    println!("Yearly savings: {}", yearly_savings);  // Yearly savings: $6000.00
    
    // Division
    let bill = Money::new(120.0, Currency::USD);
    let per_person = bill / 3.0;
    println!("Each person pays: {}", per_person);  // Each person pays: $40.00
    
    // Comparison
    if remaining > yearly_savings {
        println!("You have more than your yearly savings goal!");
    }
    
    // This would panic - different currencies:
    // let invalid = Money::new(100.0, Currency::USD) + Money::new(100.0, Currency::EUR);
}
```

---

## Common Rust Operator Traits

| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `+` | `Add` | `add` | `a + b` |
| `-` | `Sub` | `sub` | `a - b` |
| `*` | `Mul` | `mul` | `a * b` |
| `/` | `Div` | `div` | `a / b` |
| `%` | `Rem` | `rem` | `a % b` |
| `==`, `!=` | `PartialEq` | `eq`, `ne` | `a == b` |
| `<`, `>`, `<=`, `>=` | `PartialOrd` | `partial_cmp` | `a < b` |
| `[]` (read) | `Index` | `index` | `a[i]` |
| `[]` (write) | `IndexMut` | `index_mut` | `a[i] = x` |
| `!` | `Not` | `not` | `!a` |
| `&` | `BitAnd` | `bitand` | `a & b` |
| `\|` | `BitOr` | `bitor` | `a \| b` |

---

## Key Takeaways

1. **Operator overloading makes code readable**: `money1 + money2` vs `money1.add(money2)`
2. **Implement traits from `std::ops`**: Each operator has a corresponding trait
3. **Keep it intuitive**: Operators should behave as users expect
4. **Type safety**: Rust ensures you can't mix incompatible types (like different currencies)
5. **Composability**: Overloaded operators work seamlessly with Rust's type system

Operator overloading is a powerful feature that, when used correctly, makes your code more expressive and closer to the domain you're modeling.
