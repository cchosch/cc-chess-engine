pub mod c_engine{
    use std::cmp::Ordering;
    use std::fmt;

    enum BoardState{
        empty,
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
                BoardState::empty => return 0,
                _ => return 3,
            }
        }
    }
    
    struct Board{
        board : [BoardState; 64],
    }
    
    impl fmt::Display for BoardState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self{
                BoardState::king => write!(f, "Kn"),
                BoardState::queen => write!(f, "Qn"),
                BoardState::rook => write!(f, "Rk"),
                BoardState::pawn => write!(f, "Pn"),
                BoardState::knight => write!(f, "Kn"),
                BoardState::bishop => write!(f, "Bp"),
                _ => write!(f, "  ")
            }
        }
    }
    
    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut c;
            for s in 0..self.board.len(){
                c = s % 7;
                write!(f, "| {} |", self.board[s]);
                if c == 0 && s / 7 >= 1{
                    writeln!(f, "");
                }
            }
            write!(f, "")
        }
    }
    
    impl Default for Board {
        fn default() -> Board {
            let mut board = []
            for i in 0..64{
                
            }
        }
    }
    
    impl Board {
        fn init(&mut self) {
            let mut r;
            let mut c;
            for s in 0..self.board.len(){
                r = s / 8;
                c = s % 8;
                if r == 0 || r == 7{
                    self.board[s] == BoardState::pawn;
                } 
            }
        }
    }
    
    impl std::cmp::PartialOrd for BoardState{
       fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
           return None
       }
        fn lt(&self, other : &Self) -> bool{
            return self.get_val() < other.get_val();
        }
        fn gt(&self, other : &Self) -> bool{
            return self.get_val() > other.get_val();
        }
        fn le(&self, other : &Self) -> bool{
            return self.get_val() <= other.get_val();
        }
        fn ge(&self, other : &Self) -> bool{
            return self.get_val() >= other.get_val();
        }
    }
    
    impl std::cmp::PartialEq for BoardState{
        fn eq(&self, other: &Self) -> bool{
            return self.get_val() == other.get_val()
        }
    }
}