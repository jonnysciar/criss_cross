use std::fmt;
use self::private::PrivateCrissCross;

pub struct Board {
    board: [char; 9],
    r_c_size: u8,
}

mod private {
    pub trait PrivateCrissCross {
        fn add(&mut self, sym: char, x: u8, y: u8) -> bool;
    }
}

pub trait CrissCross: private::PrivateCrissCross {
    fn check_winner(&self) -> bool;
    fn add_x(&mut self, x: u8, y: u8) -> bool;
    fn add_o(&mut self, x: u8, y: u8) -> bool;
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: ['N'; 9],
            r_c_size: 3,
        }
    }
}

impl private::PrivateCrissCross for Board {
    fn add(&mut self, sym: char, x: u8, y: u8) -> bool {
        let size = (self.r_c_size*y + x) as usize;

        if size > 8 || self.board[size] != 'N' {
            return false;
        }

        self.board[size] = sym;
        return true;
    }
    
}

impl CrissCross for Board {
    fn check_winner(&self) -> bool {
        return true;
    }

    fn add_x(&mut self, x: u8, y: u8) -> bool {
        self.add('X', x, y)
    }

    fn add_o(&mut self, x: u8, y: u8) -> bool {
        self.add('O', x, y)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = Default::default();
        let mut i = 0u8;
        output += &"───────\n".to_string();
        for c in self.board {
            output += &"|".to_string();
            if c != 'X' && c != 'O' {
                output += &" ".to_string();
            } else {
                output += &c.to_string();
            }
            if i%self.r_c_size == 2 {
                output += &"|\n".to_string();
            }
            i += 1;
        }
        output += &"───────\n".to_string();
        write!(f, "{}", output)
    }
}