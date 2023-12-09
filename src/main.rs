use std::env;

fn main() -> anyhow::Result<()> {
    let day = env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("Day number is required"))
        .and_then(|s| s.trim().parse().map_err(|e| anyhow::anyhow!("{}", e)))?;
    match day {
        9 => {
            let (answer1, answer2) = advent2023::day9::main(include_str!("../day9/input.txt"))?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        _ => unimplemented!(),
    }
    Ok(())
}
