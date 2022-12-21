use std::fs::read_to_string;

use anyhow::Context;

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    task_1(input.as_str())?;
    Ok(())
}

/// # Errors
///
/// Will return `Err` if addx isn't followed by a number
pub fn task_1(input: &str) -> anyhow::Result<()> {
    let mut output = 0isize;
    let mut x_register = 1isize;
    let mut inc = 0isize;
    let mut wait = 0u8;
    let mut lines = input.lines();
    for clock_cycle in 0..220 {
        if clock_cycle % 40 == 19 {
            output += x_register * isize::try_from(clock_cycle + 1)?;
        }
        if wait > 0 {
            if wait == 1 {
                x_register += inc;
                inc = 0;
            }
            wait -= 1;
            continue;
        }
        if let Some(line) = lines.next() {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("noop") => {}
                Some("addx") => {
                    let amount = tokens
                        .next()
                        .context(format!("No amount specified in line {}", clock_cycle))?;
                    wait = 1;
                    inc = amount
                        .parse()
                        .context(format!("Amount is not an integer in line {}", clock_cycle))?;
                }
                _ => unreachable!(),
            }
        }
    }
    println!("{}", output);
    Ok(())
}
