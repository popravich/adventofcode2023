pub fn main(data: &str) -> anyhow::Result<(i64, i64)> {
    // let data = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    let mut result1 = 0;
    for line in data.lines() {
        let mut numbers = line
            .split(' ')
            .map(|w| w.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        for k in (0..numbers.len()).rev() {
            for i in 1..=k {
                let a = numbers[i - 1];
                let b = numbers[i];
                numbers[i - 1] = b - a;
            }
            if numbers[0..k].iter().all(|x| *x == 0) {
                result1 += numbers.iter().sum::<i64>();
                break
            }
        }
    }

    let mut result2 = 0;
    for line in data.lines() {
        let mut numbers = line
            .split(' ')
            .rev()
            .map(|w| w.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        for k in (0..numbers.len()).rev() {
            for i in 1..=k {
                let a = numbers[i - 1];
                let b = numbers[i];
                numbers[i - 1] = b - a;
            }
            if numbers[0..k].iter().all(|x| *x == 0) {
                result2 += numbers.iter().sum::<i64>();
                break
            }
        }
    }

    Ok((result1, result2))
}
