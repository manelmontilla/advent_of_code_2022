use std::env;
use std::fs;
fn main() {
    solve1();
}

struct Item(u8);
impl Item {
    fn priority(&self) -> u8 {

    }
}

impl From<u8> for Item {
    fn from(val: u8) -> Self {
        if  val <= b'Z' && val >= b'A'  {
                return Item(val)
        };
        if  val <= b'z' && val >= b'a'  {
            return Item(val)
        };
        panic!("invalid item type {}", val.to_string())
    }
}

struct RuckSack {
    
}

fn solve1() {
    let input = env::args().collect::<Vec<String>>();
    let input = &input[1];
    let content = fs::read_to_string(input).unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();
    let au = b'A';
    let al = b'a';
    let zu = b'Z';
    let zl = b'z';
    println!("a priority: {}, z priority: {}",  al - al+ 1,  zl-al + 1);
    println!("A priority: {}, Z priority: {}",  26 + au - au + 1, zu - au + 1 + 26, );
}
