use sscanf::sscanf;
use sscanf::RegexRepresentation;
use std::num::ParseIntError;
use std::str::FromStr;

type Item = usize;

enum Op {
    Times,
    Plus,
}

impl Op {
    fn on(&self, item1: Item, item2: Item) -> Item {
        match self {
            Op::Times => item1 * item2,
            Op::Plus => item1 + item2,
        }
    }
}

impl RegexRepresentation for Op {
    const REGEX: &'static str = r"[*+]";
}

impl FromStr for Op {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Times),
            "+" => Ok(Op::Plus),
            x => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Invalid operation {x}"),
            )),
        }
    }
}

enum Expr {
    Num(Item),
    Old,
}

impl Expr {
    pub fn or(&self, old: Item) -> Item {
        match self {
            Expr::Num(item) => *item,
            Expr::Old => old,
        }
    }
}

impl RegexRepresentation for Expr {
    const REGEX: &'static str = r"old|\d+";
}

impl FromStr for Expr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Expr::Old,
            x => Expr::Num(x.parse::<Item>()?),
        })
    }
}

struct ThrownItem {
    item: Item,
    to_monkey: usize,
}

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test_mod: usize,
    num_inspections: u32,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

impl Monkey {
    pub fn new(instring: &str) -> Result<Self, sscanf::Error> {
        let lines: Vec<_> = instring.lines().collect();
        let items_str =
            sscanf!(lines[1], "  Starting items: {str}").expect("There should be an items list");
        let (expr1, op, expr2) = sscanf!(lines[2], "  Operation: new = {Expr} {Op} {Expr}")
            .expect("There should be an operation");
        let test_mod =
            sscanf!(lines[3], "  Test: divisible by {usize}").expect("There should be a test");
        let true_monkey_index = sscanf!(lines[4], "    If true: throw to monkey {usize}")
            .expect("There should be a true monkey");
        let false_monkey_index = sscanf!(lines[5], "    If false: throw to monkey {usize}")
            .expect("There should be a false monkey");

        Ok(Monkey {
            items: items_str
                .split(", ")
                .map(str::parse::<Item>)
                .map(Result::unwrap)
                .collect(),
            operation: Box::new(move |old: Item| op.on(expr1.or(old), expr2.or(old))),
            test_mod,
            num_inspections: 0,
            true_monkey_index,
            false_monkey_index,
        })
    }

    pub fn inspect_next(&mut self, modulo: usize) -> Option<ThrownItem> {
        let old = self.items.pop()?;
        let new = (self.operation)(old) % modulo;

        self.num_inspections += 1;

        Some(ThrownItem {
            item: new,
            to_monkey: match new % self.test_mod == 0 {
                true => self.true_monkey_index,
                false => self.false_monkey_index,
            },
        })
    }

    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
}

/// Calculates the level of monkey business and returns it
/// # Examples
/// ```
/// use advent_of_code_2022_11::monkey_business;
///
/// assert_eq!(
///     2713310158,
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
/// ), 10000));
/// ```
pub fn monkey_business(input: &str, n_rounds: u32) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Monkey::new)
        .map(|m| m.expect("This should produce a valid Monkey"))
        .collect();

    let modulo: usize = monkeys.iter().map(|m| m.test_mod).product();

    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            let (left, big_right) = monkeys.split_at_mut(i);
            let (monkey, right) = big_right.split_at_mut(1);
            let mut other_monkey: &mut Monkey;
            while let Some(ThrownItem { item, to_monkey }) = monkey[0].inspect_next(modulo) {
                if to_monkey < i {
                    other_monkey = &mut left[to_monkey]
                } else {
                    other_monkey = &mut right[to_monkey - (i + 1)]
                }

                other_monkey.catch(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.num_inspections as usize).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0..=1].iter().product()
}
