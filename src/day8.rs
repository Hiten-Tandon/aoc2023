use {itertools::Itertools, num::Integer};

fn encode(s: &str) -> u32 {
    s.bytes()
        .map(|x| x - b'A')
        .map(u32::from)
        .fold(0, |acc, x| (acc << 5) | x)
}

pub fn get_path_length(input: &str) -> u64 {
    const START: u32 = 0; //AAA => A => 0
    const END: u32 = 0b110011100111001; //ZZZ => Z => 25 => 11001 => D
    let mut lines = input.lines();
    let instructions = lines
        .by_ref()
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == 'L' { 0_usize } else { 1_usize })
        .collect::<Vec<_>>();
    let mut map: [[u32; 2]; 32768] = [[0; 2]; 32768];

    lines
        .skip(1)
        .filter_map(|line| line.split(" = ").collect_tuple())
        .map(|(src, dest)| {
            let src = encode(src);
            let (left, right) = dest.split(',').map(|x| x.trim()).collect_tuple().unwrap();
            let left = encode(&left[1..]);
            let right = encode(&right[..3]);

            (src, [left, right])
        })
        .for_each(|(k, v)| map[k as usize] = v);

    let mut curr_node = START;
    let mut s_count = 0;

    for instruction in instructions.into_iter().cycle() {
        s_count += 1;
        curr_node = map[curr_node as usize][instruction];
        if curr_node == END {
            return s_count;
        }
    }

    u64::max_value()
}

pub fn get_ghost_path_length(input: &str) -> u64 {
    let mut curr_nodes: Vec<u32> = vec![];
    let mut lines = input.lines();
    let mut map: [[u32; 2]; 32768] = [[0; 2]; 32768];

    let instructions = lines
        .by_ref()
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == 'L' { 0_usize } else { 1_usize })
        .collect::<Box<[_]>>();

    lines
        .skip(1)
        .filter_map(|line| line.split(" = ").collect_tuple())
        .map(|(src, dest)| {
            let src = encode(src);
            let (left, right) = dest.split(',').map(|x| x.trim()).collect_tuple().unwrap();
            let left = encode(&left[1..]);
            let right = encode(&right[..3]);

            if src & 31 == 0 {
                curr_nodes.push(src);
            }
            (src, [left, right])
        })
        .for_each(|(k, v)| map[k as usize] = v);

    curr_nodes
        .into_iter()
        .map(|mut curr_node| {
            let mut s_count = 0;
            while curr_node & 31 != 25 {
                s_count += 1;
                curr_node =
                    map[curr_node as usize][instructions[(s_count - 1) % instructions.len()]];
            }
            s_count as u64
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}
