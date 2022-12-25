fn parse_snafu(snafu: &str) -> i64 {
    (0..snafu.len()).rev().zip(snafu.chars()).fold(0, |acc, (i, c)| {
        acc + match c {
            '=' => -2i64,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!()
        } * 5i64.pow(i as _)
    })
}

const SNAFU_CHARS: [char; 5] = ['0', '1', '2', '=', '-'];

fn encode_snafu(mut val: i64) -> String {
    let mut i = 0;
    let mut carry = 0;
    let mut res = String::new();
    while val > 0 {
        let pow = 5i64.pow(i);
        let a = (val / pow) % 5;
        let b = (a + carry) % 5;
        let c = SNAFU_CHARS[b as usize];
        res.insert(0, c);
        if (carry == 1 && a > 1) || (carry == 0 && a > 2) {
            carry = 1;
        } else {
            carry = 0;
        }
        val -= a * pow;
        i += 1;
    }
    if carry == 1 {
        res.insert(0, '1');
    }
    res
}

fn main() {
    let result = include_str!("day_25_input.txt").trim_end()
        .lines()
        .map(parse_snafu)
        .sum::<i64>();
    let part1 = encode_snafu(result);
    println!("part1: {part1}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_snafu("1"), 1);
        assert_eq!(parse_snafu("2"), 2);
        assert_eq!(parse_snafu("1="), 3);
        assert_eq!(parse_snafu("1-"), 4);
        assert_eq!(parse_snafu("10"), 5);
        assert_eq!(parse_snafu("11"), 6);
        assert_eq!(parse_snafu("12"), 7);
        assert_eq!(parse_snafu("2="), 8);
        assert_eq!(parse_snafu("2-"), 9);
        assert_eq!(parse_snafu("20"), 10);
        assert_eq!(parse_snafu("1=-0-2"), 1747);
        assert_eq!(parse_snafu("12111"), 906);
        assert_eq!(parse_snafu("2=0="), 198);
        assert_eq!(parse_snafu("21"), 11);
        assert_eq!(parse_snafu("2=01"), 201);
        assert_eq!(parse_snafu("111"), 31);
        assert_eq!(parse_snafu("20012"), 1257);
        assert_eq!(parse_snafu("112"), 32);
        assert_eq!(parse_snafu("1=-1="), 353);
        assert_eq!(parse_snafu("1-12"), 107);
        assert_eq!(parse_snafu("12"), 7);
        assert_eq!(parse_snafu("1="), 3);
        assert_eq!(parse_snafu("122"), 37);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode_snafu(1), "1");
        assert_eq!(encode_snafu(2), "2");
        assert_eq!(encode_snafu(3), "1=");
        assert_eq!(encode_snafu(4), "1-");
        assert_eq!(encode_snafu(5), "10");
        assert_eq!(encode_snafu(6), "11");
        assert_eq!(encode_snafu(7), "12");
        assert_eq!(encode_snafu(8), "2=");
        assert_eq!(encode_snafu(9), "2-");
        assert_eq!(encode_snafu(10), "20");
        assert_eq!(encode_snafu(1747), "1=-0-2");
        assert_eq!(encode_snafu(906), "12111");
        assert_eq!(encode_snafu(198), "2=0=");
        assert_eq!(encode_snafu(11), "21");
        assert_eq!(encode_snafu(201), "2=01");
        assert_eq!(encode_snafu(31), "111");
        assert_eq!(encode_snafu(1257), "20012");
        assert_eq!(encode_snafu(32), "112");
        assert_eq!(encode_snafu(353), "1=-1=");
        assert_eq!(encode_snafu(107), "1-12");
        assert_eq!(encode_snafu(7), "12");
        assert_eq!(encode_snafu(3), "1=");
        assert_eq!(encode_snafu(37), "122");
    }
}
