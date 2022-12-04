use std::env;
use std::fs;
use std::process::exit;
fn main() {
    solve();
}

struct Top3 {
    first: (usize, usize),
    second: (usize, usize),
    third: (usize, usize),
}

impl Top3 {
    fn process(&mut self, calories: usize, index: usize) {
        if calories > self.first.0 {
            self.third = self.second;
            self.second = self.first;
            self.first.0 = calories;
            self.first.1 = index;
            return;
        }
        if calories > self.second.0 {
            self.third = self.second;
            self.second.0 = calories;
            self.second.1 = index;
            return;
        }
        if calories > self.third.0 {
            self.third.0 = calories;
            self.third.1 = index;
            return;
        }
    }

    fn total(&self) -> usize {
        self.first.0 + self.second.0 + self.third.0
    }

    fn positions(&self) -> String {
        format!("[{},{},{}]", self.first.1, self.second.1, self.third.1)
    }
}

fn solve() {
    let input = env::args().collect::<Vec<String>>();
    let input = &input[1];
    let content = fs::read_to_string(input).unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();

    let mut i = 0;
    let mut top3 = Top3 {
        first: (0, 0),
        second: (0, 0),
        third: (0, 0),
    };
    let mut current_max = 0;
    let mut current_elf = 0;
    while i < lines.len() {
        let current = lines[i];
        if current == "" {
            println!("calories at elf {}: {}", i + 1, current_max);
            top3.process(current_max, current_elf);
            current_elf += 1;
            current_max = 0;
        } else {
            let calories = match current.parse::<usize>() {
                Ok(cals) => cals,
                Err(err) => {
                    print!(
                        "error pasing line {}, in index {}, error: {}\n",
                        current, i, err
                    );
                    exit(1);
                }
            };
            current_max = current_max + calories;
        }

        i = i + 1;
    }
    print!("max: {}, elfs: {}", top3.total(), top3.positions())
}
