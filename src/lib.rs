use sscanf::sscanf;
use sscanf::RegexRepresentation;
use std::str::FromStr;
use std::num::ParseIntError;

type Item = u32;

enum Op{
    Times,
    Plus
}

impl Op{
    fn on(&self, item1: Item, item2: Item) -> Item{
        match self{
            Op::Times => item1 * item2,
            Op::Plus  => item1 + item2
        }
    }
}

impl RegexRepresentation for Op{
    const REGEX: &'static str = r"[*+]";
}

impl FromStr for Op{
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s{
            "*" => Ok(Op::Times),
            "+" => Ok(Op::Plus),
            x   => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Invalid operation {x}")))
        }
    }
}


enum Expr{
    Num(Item),
    Old
}

impl Expr{
    pub fn or(&self, old: Item) -> Item{
        match self{
            Expr::Num(item) => *item,
            Expr::Old    => old
        }
    }
}

impl RegexRepresentation for Expr{
    const REGEX: &'static str = r"old|\d+";
}

impl FromStr for Expr{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        Ok(match s{
            "old" => Expr::Old,
            x     => Expr::Num(x.parse::<Item>()?)
        })
    }
}

struct ThrownItem{
    item: Item,
    to_monkey: usize
}

struct Monkey{
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test_mod: u32,
    num_inspections: u32,
    true_monkey_index: usize,
    false_monkey_index: usize
}

impl Monkey{
    pub fn new(instring: &str) -> Result<Self, sscanf::Error>{
        let (
                _,
                items_str,
                expr1, op, expr2,
                test_mod,
                true_monkey,
                false_monkey
        ) = sscanf!(
            instring,
            "Monkey {usize}:\n  Starting items: {str}\n  Operation: new = {Expr} {Op} {Expr}\n  Test: divisible by {u32}\n    If true: throw to monkey {usize}\n    If false: throw to monkey {usize}\n\n"
        )?;

        Ok(Monkey{
            items: items_str.split(", ").map(str::parse::<Item>).map(Result::unwrap).collect(),
            operation: Box::new(move |old: Item| {op.on(expr1.or(old), expr2.or(old))}),
            test_mod: test_mod,
            num_inspections: 0,
            true_monkey_index: true_monkey,
            false_monkey_index: false_monkey
        })
    }

    pub fn inspect_next(&mut self) -> Option<ThrownItem>{
        let old = self.items.pop()?;
        let new = (self.operation)(old);

        self.num_inspections += 1;

        Some(ThrownItem{
                item: new,
                to_monkey: match old % self.test_mod == 0{
                    true  => self.true_monkey_index,
                    false => self.false_monkey_index
                }})
    }

    pub fn catch(&mut self, item: Item){
        self.items.push(item);
    }
}

/// Calculates the level of monkey business and returns it
/// # Examples
/// ```
/// use advent_of_code_2022_11::monkey_business;
///
/// assert_eq!(
///     10605,
///     monkey_business(concat!(
///         "Monkey 0:\n",
///         "  Starting items: 79, 98\n",
///         "  Operation: new = old * 19\n",
///         "  Test: divisible by 23\n",
///         "    If true: throw to monkey 2\n",
///         "    If false: throw to monkey 3\n",
///         "\n",
///         "Monkey 1:\n",
///         "  Starting items: 54, 65, 75, 74\n",
///         "  Operation: new = old + 6\n",
///         "  Test: divisible by 19\n",
///         "    If true: throw to monkey 2\n",
///         "    If false: throw to monkey 0\n",
///         "\n",
///         "Monkey 2:\n",
///         "  Starting items: 79, 60, 97\n",
///         "  Operation: new = old * old\n",
///         "  Test: divisible by 13\n",
///         "    If true: throw to monkey 1\n",
///         "    If false: throw to monkey 3\n",
///         "\n",
///         "Monkey 3:\n",
///         "  Starting items: 74\n",
///         "  Operation: new = old + 3\n",
///         "  Test: divisible by 17\n",
///         "    If true: throw to monkey 0\n",
///         "    If false: throw to monkey 1"
/// )));
/// ```
pub fn monkey_business(input: &str) -> usize{
    let monkeys: Vec<Monkey> = 
        input
        .split("\n\n")
        .map(Monkey::new)
        .map(Result::unwrap)
        .collect();
    todo!();
}
