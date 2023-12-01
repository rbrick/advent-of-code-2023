use std::fs;

const ENGLISH: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn dict_index(s1: &String) -> Option<usize> {
    return ENGLISH.iter().position(|s2| *s1 == s2.to_string());
}

#[derive(Debug, Clone, Copy)]
struct Found {
    index: usize,
    val: i32,
}

impl Found {
    fn from(t: (usize, &str)) -> Self {
        return Found {
            index: t.0,
            val: dict_index(&t.1.to_string()).unwrap() as i32 + 1,
        };
    }
}

fn collect_english_numbers(s: &String) -> Vec<Found> {
    let mut found_numbers = vec![];
    for word in ENGLISH {
        let matched: Vec<_> = s.match_indices(word).into_iter().collect();

        if !matched.is_empty() {

            for m in matched {
                found_numbers.push(Found::from(m));
            }
        }
    }

    found_numbers.sort_by(|a, b| a.index.cmp(&b.index));
    return found_numbers;
}

pub fn day_one() {
    let contents: String = fs::read_to_string("inputs.txt").expect("Should have read file.");
    
    let mut total_sum_one = 0;
    let mut total_sum_two = 0;

    for line in contents.lines() {
        if line.starts_with("#") {
            continue;
        }

        let mut numbers_one = vec![];
        let mut numbers_two = vec![];

        numbers_two.append(&mut collect_english_numbers(&line.to_string()));

        let mut pos = 0;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                let num = (c as i32) - ('0' as i32);
                let f = Found {
                    index: pos,
                    val: num,
                };
                numbers_one.push(f);
                numbers_two.push(f);
            }
            pos += 1;
        }

        numbers_two.sort_by(|a, b| a.index.cmp(&b.index));

        let p1_first = numbers_one.first().unwrap();
        let p1_last = numbers_one.last().unwrap();

        let p2_first = numbers_two.first().unwrap();
        let p2_last = numbers_two.last().unwrap();

        total_sum_one += (p1_first.val * 10) + p1_last.val;
        total_sum_two += (p2_first.val * 10) + p2_last.val;
    }

    println!("part 1: {total_sum_one}\npart 2: {total_sum_two}");
}