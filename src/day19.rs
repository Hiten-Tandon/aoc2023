use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = s
            .split(',')
            .map(|x| {
                x.chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<u64>()
            })
            .filter_map(Result::ok)
            .collect_tuple()
            .unwrap();
        Ok(Self { x, m, a, s })
    }
}

#[inline(always)]
fn count(p1: Part, p2: Part) -> u64 {
    p2.x.saturating_sub(p1.x)
        * p2.m.saturating_sub(p1.m)
        * p2.a.saturating_sub(p1.a)
        * p2.s.saturating_sub(p1.s)
}

pub fn sum_good_part_ids(input: &str) -> u64 {
    let workflows: HashMap<_, _> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .filter_map(|x| x.split('{').collect_tuple())
        .filter_map(|(k, v)| {
            Some((
                k,
                v[..v.len() - 1]
                    .split(',')
                    .map(|x| {
                        if !x.contains(':') {
                            ("x<", 4001, x)
                        } else {
                            let (p1, command) = x.split(':').collect_tuple().unwrap();
                            let cond = &p1[..2];
                            let threshold = p1[2..].parse::<u64>().unwrap();
                            (cond, threshold, command)
                        }
                    })
                    .collect_vec(),
            ))
        })
        .collect();

    let parts: Box<[Part]> = input
        .lines()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .map(str::parse::<Part>)
        .filter_map(Result::ok)
        .collect();

    let mut frontier: Vec<(Part, Part, &str)> = vec![(
        Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        },
        Part {
            x: u64::max_value(),
            m: u64::max_value(),
            a: u64::max_value(),
            s: u64::max_value(),
        },
        "in",
    )];

    let mut ranges: Vec<(Part, Part)> = vec![];
    while let Some((mut start, mut end, workflow_id)) = frontier.pop() {
        workflows.get(workflow_id).unwrap().into_iter().for_each(
            |&(cond, threshold, action)| match (cond, action) {
                ("x<", "A") => {
                    ranges.push((
                        start,
                        Part {
                            x: end.x.min(threshold),
                            ..end
                        },
                    ));

                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x<", "R") => {
                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x<", action) => {
                    frontier.push((
                        start,
                        Part {
                            x: end.x.min(threshold),
                            ..end
                        },
                        action,
                    ));
                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x>", "A") => {
                    ranges.push((
                        Part {
                            x: start.x.max(threshold + 1),
                            ..start
                        },
                        end,
                    ));

                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("x>", "R") => {
                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("x>", action) => {
                    frontier.push((
                        Part {
                            x: start.x.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));
                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("m<", "A") => {
                    ranges.push((
                        start,
                        Part {
                            m: end.m.min(threshold),
                            ..end
                        },
                    ));

                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m<", "R") => {
                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m<", action) => {
                    frontier.push((
                        start,
                        Part {
                            m: end.m.min(threshold),
                            ..end
                        },
                        action,
                    ));
                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m>", "A") => {
                    ranges.push((
                        Part {
                            m: start.m.max(threshold + 1),
                            ..start
                        },
                        end,
                    ));

                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("m>", "R") => {
                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("m>", action) => {
                    frontier.push((
                        Part {
                            m: start.m.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));

                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("a<", "A") => {
                    ranges.push((
                        start,
                        Part {
                            a: end.a.min(threshold),
                            ..end
                        },
                    ));

                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a<", "R") => {
                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a<", action) => {
                    frontier.push((
                        start,
                        Part {
                            a: end.a.min(threshold),
                            ..end
                        },
                        action,
                    ));

                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a>", "A") => {
                    ranges.push((
                        Part {
                            a: start.a.max(threshold + 1),
                            ..start
                        },
                        end,
                    ));

                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("a>", "R") => {
                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("a>", action) => {
                    frontier.push((
                        Part {
                            a: start.a.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));

                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("s<", "A") => {
                    ranges.push((
                        start,
                        Part {
                            s: end.s.min(threshold),
                            ..end
                        },
                    ));

                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s<", "R") => {
                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s<", action) => {
                    frontier.push((
                        start,
                        Part {
                            s: end.s.min(threshold),
                            ..end
                        },
                        action,
                    ));

                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s>", "A") => {
                    ranges.push((
                        Part {
                            s: start.s.max(threshold + 1),
                            ..start
                        },
                        end,
                    ));

                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                ("s>", "R") => {
                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                ("s>", action) => {
                    frontier.push((
                        Part {
                            s: start.s.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));
                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                _ => (),
            },
        );
    }

    parts
        .iter()
        .copied()
        .filter(|&p| {
            ranges
                .iter()
                .find(|(start, end)| {
                    (start.x..end.x).contains(&p.x)
                        && (start.m..end.m).contains(&p.m)
                        && (start.a..end.a).contains(&p.a)
                        && (start.s..end.s).contains(&p.s)
                })
                .is_some()
        })
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum()
}

pub fn count_good_part_ids(input: &str) -> u64 {
    let workflows: HashMap<_, _> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .filter_map(|x| x.split('{').collect_tuple())
        .filter_map(|(k, v)| {
            Some((
                k,
                v[..v.len() - 1]
                    .split(',')
                    .map(|x| {
                        if !x.contains(':') {
                            ("x<", 4001, x)
                        } else {
                            let (p1, command) = x.split(':').collect_tuple().unwrap();
                            let cond = &p1[..2];
                            let threshold = p1[2..].parse::<u64>().unwrap();
                            (cond, threshold, command)
                        }
                    })
                    .collect_vec(),
            ))
        })
        .collect();

    let mut res = 0;

    let mut frontier: Vec<(Part, Part, &str)> = vec![(
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4001,
            m: 4001,
            a: 4001,
            s: 4001,
        },
        "in",
    )];

    while let Some((mut start, mut end, workflow_id)) = frontier.pop() {
        workflows.get(workflow_id).unwrap().into_iter().for_each(
            |&(cond, threshold, action)| match (cond, action) {
                ("x<", "A") => {
                    res += count(
                        start,
                        Part {
                            x: end.x.min(threshold),
                            ..end
                        },
                    );

                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x<", "R") => {
                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x<", action) => {
                    frontier.push((
                        start,
                        Part {
                            x: end.x.min(threshold),
                            ..end
                        },
                        action,
                    ));
                    start = Part {
                        x: start.x.max(threshold),
                        ..start
                    }
                }
                ("x>", "A") => {
                    res += count(
                        Part {
                            x: start.x.max(threshold + 1),
                            ..start
                        },
                        end,
                    );

                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("x>", "R") => {
                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("x>", action) => {
                    frontier.push((
                        Part {
                            x: start.x.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));
                    end = Part {
                        x: end.x.min(threshold + 1),
                        ..end
                    }
                }
                ("m<", "A") => {
                    res += count(
                        start,
                        Part {
                            m: end.m.min(threshold),
                            ..end
                        },
                    );

                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m<", "R") => {
                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m<", action) => {
                    frontier.push((
                        start,
                        Part {
                            m: end.m.min(threshold),
                            ..end
                        },
                        action,
                    ));
                    start = Part {
                        m: start.m.max(threshold),
                        ..start
                    }
                }
                ("m>", "A") => {
                    res += count(
                        Part {
                            m: start.m.max(threshold + 1),
                            ..start
                        },
                        end,
                    );

                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("m>", "R") => {
                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("m>", action) => {
                    frontier.push((
                        Part {
                            m: start.m.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));

                    end = Part {
                        m: end.m.min(threshold + 1),
                        ..end
                    }
                }
                ("a<", "A") => {
                    res += count(
                        start,
                        Part {
                            a: end.a.min(threshold),
                            ..end
                        },
                    );

                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a<", "R") => {
                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a<", action) => {
                    frontier.push((
                        start,
                        Part {
                            a: end.a.min(threshold),
                            ..end
                        },
                        action,
                    ));

                    start = Part {
                        a: start.a.max(threshold),
                        ..start
                    }
                }
                ("a>", "A") => {
                    res += count(
                        Part {
                            a: start.a.max(threshold + 1),
                            ..start
                        },
                        end,
                    );

                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("a>", "R") => {
                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("a>", action) => {
                    frontier.push((
                        Part {
                            a: start.a.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));

                    end = Part {
                        a: end.a.min(threshold + 1),
                        ..end
                    }
                }
                ("s<", "A") => {
                    res += count(
                        start,
                        Part {
                            s: end.s.min(threshold),
                            ..end
                        },
                    );

                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s<", "R") => {
                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s<", action) => {
                    frontier.push((
                        start,
                        Part {
                            s: end.s.min(threshold),
                            ..end
                        },
                        action,
                    ));

                    start = Part {
                        s: start.s.max(threshold),
                        ..start
                    }
                }
                ("s>", "A") => {
                    res += count(
                        Part {
                            s: start.s.max(threshold + 1),
                            ..start
                        },
                        end,
                    );

                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                ("s>", "R") => {
                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                ("s>", action) => {
                    frontier.push((
                        Part {
                            s: start.s.max(threshold + 1),
                            ..start
                        },
                        end,
                        action,
                    ));
                    end = Part {
                        s: end.s.min(threshold + 1),
                        ..end
                    }
                }
                _ => (),
            },
        );
    }
    res
}
