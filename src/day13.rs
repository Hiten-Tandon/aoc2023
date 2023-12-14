pub fn evaluate_point_of_incidence(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut chunks: Vec<Box<[Box<[char]>]>> = vec![];

    while lines.clone().count() != 0 {
        chunks.push(
            lines
                .by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| x.chars().collect())
                .collect(),
        )
    }

    let mut total = 0;
    for chunk in chunks {
        for r in 1..chunk.len() {
            let above = &chunk[..r];
            let below = &chunk[r..];

            if above.iter().rev().zip(below.iter()).all(|(a, b)| a == b) {
                total += r as u64 * 100;
                break;
            }
        }

        for c in 1..chunk[0].len() {
            let left = &chunk.iter().map(|x| &x[..c]).collect::<Box<[_]>>();
            let right = &chunk.iter().map(|x| &x[c..]).collect::<Box<[_]>>();

            if left
                .iter()
                .map(|x| x.iter().rev())
                .zip(right.iter().map(|y| y.iter()))
                .all(|(a, b)| a.zip(b).all(|(x, y)| x == y))
            {
                total += c as u64;
                break;
            }
        }
    }
    total
}

pub fn evaluate_point_of_incidence_after_removing_smudge(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut chunks: Vec<Box<[Box<[char]>]>> = vec![];

    while lines.clone().count() != 0 {
        chunks.push(
            lines
                .by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| x.chars().collect())
                .collect(),
        )
    }

    let mut total = 0;
    for chunk in chunks {
        for r in 1..chunk.len() {
            let above = &chunk[..r];
            let below = &chunk[r..];

            if above
                .iter()
                .rev()
                .zip(below.iter())
                .map(|(a, b)| a.iter().zip(b.iter()).filter(|(x, y)| x != y).count())
                .sum::<usize>()
                == 1
            {
                total += r as u64 * 100;
                break;
            }
        }

        for c in 1..chunk[0].len() {
            let left = &chunk.iter().map(|x| &x[..c]).collect::<Box<[_]>>();
            let right = &chunk.iter().map(|x| &x[c..]).collect::<Box<[_]>>();

            if left
                .iter()
                .map(|x| x.iter().rev())
                .zip(right.iter().map(|y| y.iter()))
                .map(|(a, b)| a.zip(b).filter(|(x, y)| x != y).count())
                .sum::<usize>()
                == 1
            {
                total += c as u64;
                break;
            }
        }
    }
    total
}
