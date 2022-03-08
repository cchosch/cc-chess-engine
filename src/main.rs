enum BoardState{
    king,
    queen,
    rook,
    knight,
    bishop,
    pawn 
}

impl BoardState{
    fn get_val(&self) -> i16{
        match self{
            BoardState::king => return 1000,
            BoardState::queen => return 9,
            BoardState::rook => return 5,
            BoardState::pawn => return 1,
            _ => return 3,
        }
    }
}

impl std::cmp::PartialEq for BoardState{
    fn eq(&self, other: &Self) -> bool{
        return self.get_val() == other.get_val()
    }
}

fn main() {
    let mut board : [BoardState; 64];
    if BoardState::queen == BoardState::queen{
        println!("yo");
    }
    println!("Hello, world!");
}
