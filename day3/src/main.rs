use std::{env, fs};

fn main() {
    solve1();
}

static A: u8 = b'A';
static Z: u8 = b'Z';
static a: u8 = b'a';
static z: u8 = b'z';

struct Item(u8);
impl Item {
    fn priority(&self) -> u8 {
        let r = 1..57;
        if self.0 <= Z && self.0 >= A {
            return self.0 - A + 27;
        };
        self.0 - a + 1
    }

    fn from_priority(priority: u8) -> Self {
        if priority >= 1 && priority <= 26 {
            return Item(a + (priority - 1));
        };
        Item(A + (priority - 1))
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
