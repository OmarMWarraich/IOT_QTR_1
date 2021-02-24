#[derive(Debug)]
enum Direction {
    Forward,
    Left,
    Backward,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction{
        // use Direction::*;
        match self {
            Direction::Forward => Direction::Left,
            Direction::Left => Direction::Backward,
            Direction::Backward => Direction::Right,
            Direction::Right => Direction::Forward,
            
        }
    }
}

fn main() {
    let mut mymove = Direction::Forward;
    for temp in 1..=10 {
        println!("{:?}",mymove);
        mymove = mymove.turn();
    }
}
    // use Direction::*;

    // for i in &[Forward, Left, Backward, Right,Forward,Left,Backward,Right,Forward,Right] {
    //     println!("{:?} -> {:?}", i, i.turn());
    // }
// }