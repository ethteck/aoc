use aoc_runner_derive::aoc;

#[inline]
pub fn parse_number(input: &str) -> isize {
    input.parse().unwrap()
}

#[inline]
pub fn parse_input(input: &str) -> [[isize; 8]; 1000] {
    let mut ret = [[0; 8]; 1000];
    let mut numstart = 0;
    let mut bi = 0;
    let mut i = 0;
    let mut j = 0;

    for b in input.as_bytes() {
        if *b == b'\n' {
            ret[i][j] = parse_number(&input[numstart..bi]);
            numstart = bi + 1;
            i += 1;
            j = 0;
        } else if *b == b' ' {
            ret[i][j] = parse_number(&input[numstart..bi]);
            numstart = bi + 1;
            j += 1;
        }
        bi += 1;
    }

    ret
}

#[inline]
pub fn is_okay(data: &[isize]) -> bool {
    let inc: bool = data[0] < data[1];
    for i in 0..data.len() - 1 {
        if data[i + 1] == 0 {
            return true;
        }

        let dist = data[i + 1] - data[i];
        if dist == 0 || dist.abs() > 3 || (inc != (dist > 0)) {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> isize {
    let data = parse_input(input);

    data.iter().filter(|a| is_okay(*a)).count() as isize
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> isize {
    let data = parse_input(input);

    data.iter()
        .filter(|a| {
            is_okay(*a) || {
                for i in 0..8 {
                    if a[i] == 0 {
                        return false;
                    }
                    let sub: Vec<isize> = a[..i]
                        .iter()
                        .chain((*a)[i + 1..].iter())
                        .map(|a| *a)
                        .collect::<Vec<_>>();

                    if is_okay(sub.as_slice()) {
                        return true;
                    }
                }
                false
            }
        })
        .count() as isize
}
