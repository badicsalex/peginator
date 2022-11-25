# Peginator macro

Proc-macro support for [peginator](https://github.com/badicsalex/peginator). Please find
doucmentation and other goodies on the main project.

Usage:
```rust
use peginator_macro::peginate;
use peginator_runtime::PegParser;

peginate!("
@export
PizzaRule =
    'Pizza' 'with'
    toppings:Topping
    {',' toppings:Topping}
    ['and' toppings:Topping]
;
@string
Topping = 'sausage' | 'pineapple' | 'bacon' | 'cheese';
");

fn main() {
    let result = PizzaRule::parse("Pizza with sausage, bacon and cheese").unwrap();
    println!("{:?}", result.toppings);
}
```
