fn encode(s: &str) -> usize {
    s.bytes()
        .map(|x| x - b'a')
        .map(usize::from)
        .fold(0, |acc, x| acc << 5 | x)
}

fn get_cycle(
    node: usize,
    parent: usize,
    adj_map: &[Vec<usize>],
    mut vis: Vec<usize>,
) -> Vec<Box<[usize]>> {
    if vis.contains(&node) {
        vis.push(node);
        return vec![vis.clone().into()];
    }
    vis.push(node);
    adj_map[node]
        .iter()
        .copied()
        .filter(|&x| x != parent)
        .map(|x| get_cycle(x, node, adj_map, vis.clone()))
        .flatten()
        .collect()
}

pub fn get_end_cycle_joining_combinations(input: &str) -> u64 {
    let mut adj_map: Vec<Vec<usize>> = vec![vec![]; 65536];
    let mut s = 0;
    input
        .lines()
        .map(|x| {
            x.split(&[':', ' '])
                .filter(|x| !x.is_empty())
                .map(encode)
                .collect::<Box<[_]>>()
        })
        .for_each(|r| {
            s = r[0];
            for e in r.iter().copied().skip(1) {
                adj_map[s].push(e);
                adj_map[e].push(s);
            }
        });
    let cycles = get_cycle(s, 0, &adj_map, vec![]);
    cycles
        .iter()
        .filter(|x| x.first() == x.last())
        .for_each(|x| println!("{x:?}"));
    0
}
