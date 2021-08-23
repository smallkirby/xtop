use xtop::{resource};

fn main() {
    let state = resource::process::Char2ProcState('R');
    println!("{:?}", state);
}
