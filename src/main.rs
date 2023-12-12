use std::env;

macro_rules! match_day {
    ($arg:expr, $($day:literal => $func:path),+) => {
        match $arg {
            $($day => {
                let (answer1, answer2) = $func(include_str!(concat!("../day", $day, "/input.txt")))?;
                println!("#1: {}", answer1);
                println!("#2: {}", answer2);
            })+
            _ => unimplemented!(),
        }
    };
}

fn main() -> anyhow::Result<()> {
    let day = env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("Day number is required"))
        .and_then(|s| s.trim().parse().map_err(|e| anyhow::anyhow!("{}", e)))?;
    match_day!(day,
        9 => advent2023::day9::main,
        10 => advent2023::day10::main,
        11 => advent2023::day11::main
    );
    Ok(())
}
