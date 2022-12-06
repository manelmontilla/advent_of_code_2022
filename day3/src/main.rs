use std::{collections::HashSet, env, fs};

fn main() {
    solve1();
    solve2();
}

static A: u8 = b'A';
static Z: u8 = b'Z';
static a: u8 = b'a';

struct Item(u8);
impl Item {
    fn priority(&self) -> u8 {
        if self.0 <= Z && self.0 >= A {
            return self.0 - A + 27;
        };
        self.0 - a + 1
    }
}

impl From<u8> for Item {
    fn from(val: u8) -> Self {
        if val <= b'Z' && val >= b'A' {
            return Item(val);
        };
        if val <= b'z' && val >= b'a' {
            return Item(val);
        };
        panic!("invalid item type {}", val.to_string())
    }
}

struct Rucksack {
    repeated: Item,
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (first_half, second_half) = s.as_bytes().split_at(s.len() / 2);
        let mut found: [bool; 52] = [false; 52];
        let half_len = first_half.len();
        for i in 0..half_len {
            let item_str = first_half[i];
            let item = Item::from(item_str);
            found[item.priority() as usize - 1] = true;
        }
        for i in 0..half_len {
            let item = Item::from(second_half[i]);
            if found[item.priority() as usize - 1] {
                return Rucksack { repeated: item };
            }
        }
        panic!("no repeated item found in rucksack {}", s);
    }
}

struct ElvesGroup {
    badge: Item,
}

impl From<(&str, &str, &str)> for ElvesGroup {
    fn from((first, second, third): (&str, &str, &str)) -> Self {
        let mut first_chars: HashSet<u8> = HashSet::new();
        for c in first.as_bytes() {
            first_chars.insert(c.clone());
        }
        let mut second_chars: HashSet<u8> = HashSet::new();
        for c in second.as_bytes() {
            second_chars.insert(c.clone());
        }
        let first_union_second_chars: HashSet<_> =
            first_chars.intersection(&second_chars).collect();

        let mut third_chars: HashSet<&u8> = HashSet::new();
        for c in third.as_bytes() {
            third_chars.insert(c);
        }
        let mut badge: HashSet<_> = first_union_second_chars
            .intersection(&third_chars)
            .collect();
        if badge.len() != 1 {
            panic!(
                "invalid number of badges found for {:?}: {:?}",
                (first, second, third),
                badge,
            );
        };
        let item: Vec<&&u8> = badge.drain().collect();
        let item = **item[0];
        ElvesGroup {
            badge: Item::from(item),
        }
    }
}

fn solve1() {
    let input = env::args().collect::<Vec<String>>();
    let input = &input[1];
    let content = fs::read_to_string(input).unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();
    let mut sum_repeated_priorities: usize = 0;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let rucksack = Rucksack::from(line);
        sum_repeated_priorities = sum_repeated_priorities + rucksack.repeated.priority() as usize;
    }
    println!(
        "sum of the repeated priorities in each rucksack {}",
        sum_repeated_priorities
    );
}

fn solve2() {
    let input = env::args().collect::<Vec<String>>();
    let input = &input[1];
    let content = fs::read_to_string(input).unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();
    let mut sum_badges: usize = 0;
    let mut i = 0;
    loop {
        if lines[i] == "" {
            break;
        }
        let first = lines[i];
        i = i + 1;
        let second = lines[i];
        i = i + 1;
        let third = lines[i];
        let group = ElvesGroup::from((first, second, third));
        sum_badges = sum_badges + group.badge.priority() as usize;
        i = i + 1;
    }
    println!(
        "sum of badges item types in each group of three elves is {}",
        sum_badges
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruskstack() {
        let r = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(r.repeated.0, b'p');

        let r = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(r.repeated.0, b'L');
    }

    #[test]
    fn test_priority() {
        let item = Item::from(b'p');
        assert_eq!(item.priority(), 16);

        let item = Item::from(b'L');
        assert_eq!(item.priority(), 38);
    }
}
