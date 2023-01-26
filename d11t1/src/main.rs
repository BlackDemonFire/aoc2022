use anyhow::Context;
use std::{cell::RefCell, collections::VecDeque, fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Operation {
    Multiply(isize),
    Add(isize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<isize>,
    operation: Operation,
    operation_count: usize,
    test: isize,
    positive: usize,
    negative: usize,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let raw_monkeys: [Monkey; 8] = vec_of_result_to_result_of_vec(
        input
            .split("\n\n")
            .map(Monkey::from_str)
            .map(|parsed| parsed.context("Error parsing a monkey."))
            .collect::<Vec<_>>(),
    )?
    .try_into()
    .ok()
    .context(anyhow::anyhow!("Failed to pass the monkeys to an Array."))?;
    let monkeys = raw_monkeys.map(RefCell::new);
    for _round in 0..20 {
        for monkey_ref in &monkeys {
            let mut monkey = monkey_ref.borrow_mut();
            for mut item in monkey.items.drain(..).collect::<Vec<_>>() {
                monkey.operation_count += 1;
                match monkey.operation {
                    Operation::Multiply(i) => item *= i,
                    Operation::Add(i) => item += i,
                    Operation::Square => item = item.pow(2),
                }
                item /= 3;

                let other_monkey = if item % monkey.test == 0 {
                    monkey.positive
                } else {
                    monkey.negative
                };
                match &mut monkeys.iter().nth(other_monkey) {
                    Some(it) => Ok(it),
                    None => Err(anyhow::anyhow!("No such monkey!")),
                }.unwrap()
                .borrow_mut()
                .items
                .push_back(item);
            }
        }
    }
    let mut businesses = monkeys.map(|m| m.borrow().operation_count);
    businesses.sort_unstable();
    businesses.reverse();
    assert!(businesses.len() >= 2);
    let first_item = businesses
        .first()
        .ok_or_else(|| anyhow::anyhow!("First Item is empty"))?;
    let second_item = businesses
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Second Item is empty"))?;
    let business_level = *first_item * *second_item;
    println!("{business_level}");

    Ok(())
}
fn parse_monkey(input: &str) -> anyhow::Result<Monkey> {
    let mut tokens = input.lines();
    let _id = tokens.next().context("No line specified as Monkey id")?;
    let starting_items_input = tokens
        .next()
        .context("No line specified as starting item")?;
    let operation_input = tokens
        .next()
        .context("No line specified as operator")?
        .replace("  Operation: new = old ", "");
    let test_input = tokens.next().context("No line specified as test")?;
    let positive_input = tokens
        .next()
        .context("No line specified as positive output")?;
    let negative_input = tokens
        .next()
        .context("No line specified as negative output")?;
    let starting_items = VecDeque::from(vec_of_result_to_result_of_vec(
        starting_items_input
            .replace("  Starting items: ", "")
            .split(", ")
            .map(str::parse::<isize>)
            .collect::<Vec<_>>(),
    )?);
    let operation = operation_input
        .split_once(' ')
        .context("Operator input ain't correct")?;
    let operation = match operation {
        ("+", i) => Ok(Operation::Add(i.parse()?)),
        ("*", "old") => Ok(Operation::Square),
        ("*", i) => Ok(Operation::Multiply(i.parse()?)),
        (a, b) => Err(anyhow::anyhow!("{a},{b}")),
    }?;
    let test = test_input.replace("  Test: divisible by ", "").parse()?;
    let positive = positive_input
        .replace("    If true: throw to monkey ", "")
        .parse()?;
    let negative = negative_input
        .replace("    If false: throw to monkey ", "")
        .parse()?;
    Ok(Monkey {
        items: starting_items,
        operation,
        operation_count: 0,
        test,
        positive,
        negative,
    })
}

impl FromStr for Monkey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_monkey(s)
    }
}

fn vec_of_result_to_result_of_vec<T, E>(v: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    v.into_iter().collect()
}

