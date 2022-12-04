use std::cmp::{Ord, Ordering};
use std::env;
use std::fs;
fn main() {
    solve();
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn compare_score(&self, other: &Choice) -> usize {
        if self > other {
            return self.score() + 6;
        };
        if self == other {
            return self.score() + 3;
        };
        self.score()
    }
}

impl Into<Choice> for &str {
    fn into(self) -> Choice {
        match self {
            "A" => Choice::Rock,
            "X" => Choice::Rock,
            "B" => Choice::Paper,
            "Y" => Choice::Paper,
            "C" => Choice::Scissors,
            "Z" => Choice::Scissors,
            c => panic!("invalid choice {}", c),
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            &Choice::Rock if other == &Choice::Rock => return Some(Ordering::Equal),
            &Choice::Rock if other == &Choice::Paper => return Some(Ordering::Less),
            &Choice::Rock if other == &Choice::Scissors => return Some(Ordering::Greater),

            &Choice::Paper if other == &Choice::Rock => return Some(Ordering::Greater),
            &Choice::Paper if other == &Choice::Paper => return Some(Ordering::Equal),
            &Choice::Paper if other == &Choice::Scissors => return Some(Ordering::Less),

            &Choice::Scissors if other == &Choice::Rock => return Some(Ordering::Less),
            &Choice::Scissors if other == &Choice::Paper => return Some(Ordering::Greater),
            &Choice::Scissors if other == &Choice::Scissors => return Some(Ordering::Equal),

            _ => panic!("invalid comparision {:?} {:?}", self, other),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
enum Result {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Result {
    fn from(val: &str) -> Self {
        match val {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid Result string: {}", val),
        }
    }
}

struct Play {
    opponent: Choice,
    me: Choice,
}

impl Play {
    fn from_result_line(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("invalid line: {}", line);
        };
        let result = Result::from(parts[1]);
        let opponent: Choice = parts[0].into();
        Self::from_result(opponent, &result)
    }

    fn from_result(opponent: Choice, result: &Result) -> Self {
        if result == &Result::Draw {
            let me = opponent.clone();
            return Self {
                opponent: opponent,
                me: me,
            };
        };
        match opponent {
            Choice::Rock if result == &Result::Win => Self {
                opponent: opponent,
                me: Choice::Paper,
            },
            Choice::Rock if result == &Result::Lose => Self {
                opponent: opponent,
                me: Choice::Scissors,
            },

            Choice::Paper if result == &Result::Win => Self {
                opponent: opponent,
                me: Choice::Scissors,
            },
            Choice::Paper if result == &Result::Lose => Self {
                opponent: opponent,
                me: Choice::Rock,
            },

            Choice::Scissors if result == &Result::Win => Self {
                opponent: opponent,
                me: Choice::Rock,
            },
            Choice::Scissors if result == &Result::Lose => Self {
                opponent: opponent,
                me: Choice::Paper,
            },

            _ => unreachable!(),
        }
    }

    fn my_score(&self) -> usize {
        let me = &self.me;
        me.compare_score(&self.opponent)
    }

    fn opponent_score(&self) -> usize {
        let opponent = &self.opponent;
        opponent.compare_score(&self.me)
    }
}

impl Into<Play> for &str {
    fn into(self) -> Play {
        let parts = self.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("invalid line: {}", self);
        };
        Play {
            opponent: parts[0].into(),
            me: parts[1].into(),
        }
    }
}

struct Match {
    plays: Vec<Play>,
    my_score: usize,
    opponent_score: usize,
}

impl Match {
    fn add(&mut self, play: Play) {
        self.my_score = self.my_score + play.my_score();
        self.opponent_score = self.opponent_score + play.opponent_score();
        self.plays.push(play);
    }
}

fn solve() {
    let input = env::args().collect::<Vec<String>>();
    let input = &input[1];
    let content = fs::read_to_string(input).unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();
    let mut m = Match {
        plays: Vec::new(),
        my_score: 0,
        opponent_score: 0,
    };
    let mut i = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        // let play: Play = line.into();
        let play = Play::from_result_line(line);
        m.add(play);
        i = i + 1;
    }
    println!("my total score {}", m.my_score)
}
