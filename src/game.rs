use std::vec;

use crate::game;

pub struct GameState {
    pub board: Vec<Vec<i8>>,
    pub my_id: i8,
}



pub enum MOVES {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl MOVES {
    pub fn as_str(&self) -> &'static str {
        match self {
            MOVES::UP => "UP",
            MOVES::DOWN => "DOWN",
            MOVES::LEFT => "LEFT",
            MOVES::RIGHT => "RIGHT",
        }
    }
}

pub struct Coordinates {
    pub x: u8,
    pub y: u8,
}

impl Coordinates {
    pub fn left(&self) -> Coordinates {
        Coordinates{x:self.x -1, y: self.y}
    }

    pub fn right(&self) -> Coordinates {
        Coordinates{x:self.x +1, y: self.y}
    }

    pub fn up(&self) -> Coordinates {
        Coordinates{x:self.x, y: self.y+1}
    }

    pub fn down(&self) -> Coordinates {
        Coordinates{x:self.x, y: self.y-1}
    }
}

pub fn tick(game: &GameState) -> MOVES {

    let my_pos : Coordinates = get_my_pos(&game.board,game.my_id);
    
    let mut valid_moves = check_valid_moves(my_pos, &game.board);

    valid_moves.pop().unwrap_or(MOVES::DOWN)
    
}

fn check_valid_moves(my_pos: Coordinates, board: &Vec<Vec<i8>> ) -> Vec<MOVES> {

    let mut valid_moves = Vec::new();

    if is_free(my_pos.left(), board) {
        valid_moves.push(MOVES::LEFT);
    }

    if is_free(my_pos.right(), board) {
        valid_moves.push(MOVES::RIGHT);
    }

    if is_free(my_pos.up(), board) {
        valid_moves.push(MOVES::UP);
    }

    if is_free(my_pos.down(), board) {
        valid_moves.push(MOVES::DOWN);
    }

    valid_moves
}

fn is_free(pos: Coordinates, board: &Vec<Vec<i8>>) -> bool {    
    board.get(pos.x)
    board[pos.x as usize] [pos.y as usize] == 0
}

fn get_my_pos(board: &Vec<Vec<i8>>, my_id : i8) -> Coordinates{

    let mut my_x =0;
    let mut my_y = 0;

    for y in board {
        for x in y {
            if *x==my_id {
                return Coordinates{x: my_x,y: my_y};
            }
            my_x +=1;
        }

        my_y +=1;
    }

    return Coordinates { x: 0, y: 0 };
}

pub fn mark(pos: Coordinates, id: i8, board: &mut Vec<Vec<i8>>) {
    let x = pos.x as usize;
    let y = pos.y as usize;

    board[x][y] = id;
}

pub fn print_map(board: &Vec<Vec<i8>>) {
    for x in board {
        for y in x {
            print!("{y}\t");
        }
        print!("\n");
    }
}
