use aoc_runner_derive::aoc;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "day19p2.pest"]
pub struct TextParserP2;

//#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> usize {
    // split into grammar and input
    let grammar = input.split("\n\n").next().unwrap();
    //let text = input.split("\n\n").skip(1).next().unwrap();

    // A bit unorthodox approach: lets create a PEG grammar out of supplied one
    // This is very dirty - no time to make it look nice, but it does the job. Sort of.
    grammar.lines().for_each(|l| {
        //eprintln!("line: {}", l);
        let mut gi = l.split(":").take(2);
        let ruleno = gi.next().unwrap();
        let rule = gi.next().unwrap();
        eprint!("Rule{} = {{ (", ruleno);
        rule.split(" ").for_each(|r| match r {
            " " | "" => {}
            r#""a""# => eprint!(r#""a""#),
            r#""b""# => eprint!(r#""b""#),
            "|" => eprint!(r#") | ("#),
            _ => eprint!(" ~ rule{}", r),
        });
        eprintln!(" ) }}");
    });
    0
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let text = input.split("\n\n").skip(1).next().unwrap();
    text.lines().fold(0, |acc, l| {
        // Try to manually match rule8 from the start and then rule 11 on the remainder of the string and only increment if both rules match
        // This is to take care of cases where one and the same input matches on multiple variations of rules
        let mut pos: usize = 0;
        let mut v8: Vec<usize> = Vec::new();
        while let Ok(p) = TextParserP2::parse(Rule::rule8, &l[pos..]) {
            v8.push(pos);
            pos += p.concat().len();
        }
        acc + v8.iter().fold(0, |acc11, &p| {
            if p != 0 {
                if let Ok(p11) = TextParserP2::parse(Rule::rule11, &l[p..]) {
                    if p11.concat().len() + p == l.len() {
                        acc11 + 1
                    } else {
                        acc11
                    }
                } else {
                    acc11
                }
            } else {
                acc11
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = r#"18: 48 48
25: 48 81 | 41 7
48: "b"
4: 131 48 | 70 41
20: 61 48 | 57 41
89: 41 41 | 41 48
74: 41 107 | 48 124
98: 41 48
99: 97 48 | 92 41
91: 34 48
100: 48 41 | 67 48
6: 48 100 | 41 132
40: 81 48 | 7 41
124: 83 48 | 130 41
50: 7 41 | 7 48
68: 64 41 | 24 48
60: 30 41 | 86 48
75: 89 41 | 39 48
103: 67 67
58: 41 22 | 48 111
71: 67 34
56: 34 48 | 39 41
122: 48 120 | 41 89
12: 41 18 | 48 98
95: 34 41 | 103 48
93: 110 41 | 34 48
13: 43 41 | 69 48
44: 101 48 | 114 41
69: 106 41 | 32 48
67: 48 | 41
45: 7 48
117: 48 120 | 41 39
46: 48 29 | 41 82
121: 48 49 | 41 47
130: 103 48 | 89 41
132: 41 48 | 48 41
94: 41 4 | 48 76
14: 9 48 | 93 41
26: 41 72 | 48 81
79: 67 1
115: 67 132
15: 41 20 | 48 63
47: 120 41 | 81 48
27: 100 41 | 7 48
11: 42 31
113: 49 41 | 56 48
31: 48 133 | 41 127
131: 90 41 | 28 48
81: 48 48 | 67 41
23: 84 41 | 27 48
84: 18 48
107: 79 41 | 33 48
83: 67 89
49: 81 41 | 7 48
108: 102 41 | 60 48
37: 41 7 | 48 120
120: 48 41
32: 41 96 | 48 95
2: 48 85 | 41 128
102: 48 62 | 41 50
5: 41 110
61: 41 37 | 48 6
97: 41 120 | 48 34
85: 120 41 | 89 48
80: 120 48 | 98 41
92: 48 103 | 41 34
65: 16 48 | 58 41
112: 48 71 | 41 123
70: 87 48 | 115 41
39: 41 67 | 48 41
41: "a"
38: 2 48 | 77 41
110: 48 41 | 41 41
88: 7 48 | 89 41
52: 41 73 | 48 104
96: 100 48 | 103 41
66: 41 126 | 48 121
77: 40 48 | 47 41
3: 48 118 | 41 25
126: 75 48 | 27 41
1: 41 41 | 48 48
19: 48 72 | 41 18
42: 41 68 | 48 105
129: 48 110 | 41 120
72: 48 48 | 41 67
7: 41 41
59: 41 47 | 48 125
73: 1 48 | 103 41
114: 41 117 | 48 73
118: 89 41 | 132 48
51: 41 1 | 48 81
101: 129 48 | 93 41
133: 48 15 | 41 13
104: 48 1 | 41 81
123: 110 48 | 98 41
0: 8 11
55: 48 1 | 41 18
30: 89 41 | 18 48
76: 48 14 | 41 35
43: 52 48 | 112 41
24: 48 109 | 41 38
29: 116 48 | 3 41
106: 88 41
9: 89 41 | 100 48
125: 41 120 | 48 132
22: 103 48 | 18 41
21: 48 83 | 41 51
64: 48 44 | 41 74
111: 103 67
54: 41 36 | 48 55
119: 48 12 | 41 19
35: 123 48 | 91 41
127: 41 94 | 48 10
116: 41 93 | 48 122
8: 42
128: 48 72 | 41 1
28: 41 132 | 48 120
63: 41 59 | 48 17
87: 103 48 | 72 41
36: 48 81 | 41 100
17: 62 48 | 26 41
62: 100 48 | 98 41
34: 48 41 | 48 48
82: 23 48 | 21 41
78: 41 65 | 48 66
33: 120 41
109: 48 113 | 41 99
57: 41 92 | 48 80
86: 48 89 | 41 39
53: 54 48 | 119 41
10: 108 48 | 53 41
90: 81 41 | 34 48
16: 48 5 | 41 45
105: 46 41 | 78 48"#;

    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT), 0);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&INPUT), 2);
    }
}
