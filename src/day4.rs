use aoc_runner_derive::aoc;

const SIZE: usize = 140;

#[inline(always)]
pub fn get_i(r: usize, c: usize) -> usize {
    (r * (SIZE + 1) + c) as usize
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut sum = 0;

    let mut c = 0;
    let mut r = 0;

    for (i, b) in input.iter().enumerate() {
        if b == &b'X' {
            // right
            if c < SIZE - 3 {
                if input[i + 1..i + 4] == *b"MAS" {
                    sum += 1;
                }
            }
            // left
            if c >= 3 {
                if input[i - 3..i] == *b"SAM" {
                    sum += 1;
                }
            }
            // down
            if r < SIZE - 3 {
                if input[get_i(r + 1, c)] == b'M'
                    && input[get_i(r + 2, c)] == b'A'
                    && input[get_i(r + 3, c)] == b'S'
                {
                    sum += 1;
                }
            }
            // up
            if r >= 3 {
                if input[get_i(r - 1, c)] == b'M'
                    && input[get_i(r - 2, c)] == b'A'
                    && input[get_i(r - 3, c)] == b'S'
                {
                    sum += 1;
                }
            }
            // down-right
            if r < SIZE - 3 && c < SIZE - 3 {
                if input[get_i(r + 1, c + 1)] == b'M'
                    && input[get_i(r + 2, c + 2)] == b'A'
                    && input[get_i(r + 3, c + 3)] == b'S'
                {
                    sum += 1;
                }
            }
            // down-left
            if r < SIZE - 3 && c >= 3 {
                if input[get_i(r + 1, c - 1)] == b'M'
                    && input[get_i(r + 2, c - 2)] == b'A'
                    && input[get_i(r + 3, c - 3)] == b'S'
                {
                    sum += 1;
                }
            }
            // up-right
            if r >= 3 && c < SIZE - 3 {
                if input[get_i(r - 1, c + 1)] == b'M'
                    && input[get_i(r - 2, c + 2)] == b'A'
                    && input[get_i(r - 3, c + 3)] == b'S'
                {
                    sum += 1;
                }
            }
            // up-left
            if r >= 3 && c >= 3 {
                if input[get_i(r - 1, c - 1)] == b'M'
                    && input[get_i(r - 2, c - 2)] == b'A'
                    && input[get_i(r - 3, c - 3)] == b'S'
                {
                    sum += 1;
                }
            }
        }

        c += 1;
        if *b == b'\n' {
            r += 1;
            c = 0;
        }
    }

    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut sum = 0;

    let mut c = 1;
    let mut r = 1;

    for b in input.iter().skip(SIZE + 2) {
        if b == &b'A' && c < SIZE - 1 && r < SIZE - 1 {
            let ul = input[get_i(r - 1, c - 1)];
            let ur = input[get_i(r - 1, c + 1)];
            let dl = input[get_i(r + 1, c - 1)];
            let dr = input[get_i(r + 1, c + 1)];

            if ((ul == b'M' && dr == b'S') || (ul == b'S' && dr == b'M'))
                && ((ur == b'M' && dl == b'S') || (ur == b'S' && dl == b'M'))
            {
                sum += 1;
            }
        }

        c += 1;
        if *b == b'\n' {
            r += 1;
            c = 0;
        }
    }

    sum
}
