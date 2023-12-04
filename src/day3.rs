use itertools::Itertools;

fn numify(mat: &mut [Box<[char]>], row: usize, col: usize) -> u64 {
    if row >= mat.len() || col >= mat[0].len() || !mat[row][col].is_ascii_digit() {
        return 0;
    }

    let mut res = 0;
    let mut li = col;
    let mut ri = col + 1;

    while li < mat.len() && mat[row][li].is_ascii_digit() {
        res += ((mat[row][li] as u8 - b'0') as u64) * 10_u64.pow((col - li) as _);
        mat[row][li] = '.';
        li -= 1;
    }

    while ri < mat.len() && mat[row][ri].is_ascii_digit() {
        res *= 10;
        res += (mat[row][ri] as u8 - b'0') as u64;
        mat[row][ri] = '.';
        ri += 1;
    }

    res
}

fn gear_ratio(mat: &mut [Box<[char]>], row: usize, col: usize) -> u64 {
    let (a, b) = [
        numify(mat, row, col - 1),
        numify(mat, row, col + 1),
        numify(mat, row - 1, col),
        numify(mat, row + 1, col),
        numify(mat, row - 1, col - 1),
        numify(mat, row - 1, col + 1),
        numify(mat, row + 1, col - 1),
        numify(mat, row + 1, col + 1),
    ]
    .iter()
    .copied()
    .filter(|&x| x != 0)
    .collect_tuple()
    .unwrap_or((0, 0));

    a * b
}
fn sum_numbers(mat: &mut [Box<[char]>], row: usize, col: usize) -> u64 {
    numify(mat, row, col - 1)
        + numify(mat, row, col + 1)
        + numify(mat, row - 1, col)
        + numify(mat, row + 1, col)
        + numify(mat, row - 1, col - 1)
        + numify(mat, row - 1, col + 1)
        + numify(mat, row + 1, col - 1)
        + numify(mat, row + 1, col + 1)
}

pub fn sum_parts(schema: &str) -> u64 {
    let mut mat = schema
        .lines()
        .map(|line| line.chars().collect::<Box<[char]>>())
        .collect::<Box<[_]>>();

    let mut res = 0;

    for i in 0..mat.len() {
        for j in 0..mat.len() {
            if mat[i][j].is_ascii_digit() || mat[i][j] == '.' {
                continue;
            }
            res += sum_numbers(&mut mat, i, j);
        }
    }
    res
}

pub fn sum_gear_ratios(schema: &str) -> u64 {
    let mut mat = schema
        .lines()
        .map(|line| line.chars().collect::<Box<[char]>>())
        .collect::<Box<[_]>>();

    let mut res = 0;

    for i in 0..mat.len() {
        for j in 0..mat.len() {
            if mat[i][j] == '*' {
                res += gear_ratio(&mut mat, i, j);
            }
        }
    }
    res
}
