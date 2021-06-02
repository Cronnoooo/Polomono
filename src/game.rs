use crate::piece::*;
use crate::input::*;
use std::collections::HashMap;

use rand::Rng;

const DAS: u128 = 100;
const ARR: u128 = 0;
const GRAVITY: u128 = 250;

enum MovementAction {
    HardDrop,
    SoftDrop,
    Left,
    Right,
    None,
}

enum RotationAction {
    RotateCW,
    RotateCCW,
    Rotate180,
    None,
}

pub type Matrix = Vec<Vec<PieceColor>>;

pub struct Game {
    pub matrix: Matrix,
    pub piece: Piece,
    piece_data: HashMap<char, PieceType>,
    bag: Vec<char>,

    das: u128,
    arr: u128,
    arr_leftover: u128,
    gravity: u128,
    gravity_timer: u128,
}

impl Game {
    pub fn new(matrix_width: usize, matrix_height: usize) -> Self {
        let matrix = vec![vec![PieceColor::Empty; matrix_width]; matrix_height];
        let piece_data = load_piece_data();

        let mut bag = generate_bag(&piece_data);
        let first_piece = piece_data.get(&bag.pop().unwrap()).unwrap();
        let piece = Piece::new(first_piece.shape.clone(), first_piece.color);

        Self {
            matrix,
            piece,
            piece_data,
            bag,

            das: DAS,
            arr: ARR,
            arr_leftover: 0,
            gravity: GRAVITY,
            gravity_timer: 0,
        }
    }

    pub fn update(&mut self, input: &mut Input, elapsed: u128) {
        let (movement_action, rotation_action) = read_inputs(&input);

        self.gravity_timer += elapsed;
        while self.gravity_timer > self.gravity {
            self.gravity_timer -= self.gravity;
            if !self.piece.movement(&self.matrix, 0, 1) {
                self.piece.lock(&mut self.matrix);
                let remove = filled_rows(&mut self.matrix);
                remove_rows(&mut self.matrix, remove);
                self.next_piece();
                break;
            }
        }

        match movement_action {
            MovementAction::HardDrop => {
                input.hard_drop = false;
                self.piece.hard_drop(&mut self.matrix);
                let remove = filled_rows(&mut self.matrix);
                remove_rows(&mut self.matrix, remove);
                self.next_piece();
            }
            MovementAction::Left => {
                self.handle_piece_movement(input.left_held, elapsed, -1);
            }
            MovementAction::Right => {
                self.handle_piece_movement(input.right_held, elapsed, 1);
            }
            _ => {}
        }

        match rotation_action {
            RotationAction::RotateCW => {
                input.rot_cw = false;
                self.piece.rotate(&self.matrix, 1);
            }
            RotationAction::RotateCCW => {
                input.rot_ccw = false;
                self.piece.rotate(&self.matrix, 3);
            }
            RotationAction::Rotate180 => {
                input.rot_180 = false;
                self.piece.rotate(&self.matrix, 2);
            }
            _ => {}
        }
    }

    fn next_piece(&mut self) {
        if self.bag.is_empty() {
            self.bag = generate_bag(&self.piece_data);
        }
        let new_piece = self.piece_data.get(&self.bag.pop().unwrap()).unwrap().clone();
        self.piece = Piece::new(new_piece.shape.clone(), new_piece.color);
    }

    fn handle_piece_movement(&mut self, time_held: u128, elapsed: u128, direction: i32) {
        if time_held == elapsed {
            self.piece.movement(&self.matrix, direction, 0);
            self.arr_leftover = 0;
        }
        if time_held > self.das {
            let mut time = elapsed + self.arr_leftover;
            while time > self.arr {
                if !self.piece.movement(&self.matrix, direction, 0) {
                    self.arr_leftover = 0;
                    break;
                }
                time -= self.arr;
            }
            self.arr_leftover = time;
        }
    }
}

fn generate_bag(piece_list: &HashMap<char, PieceType>) -> Vec<char> {
    // Get all pieces from the list
    let mut bag: Vec<char> = piece_list.keys().cloned().collect();

    // Suffle the pieces
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
    bag
}

fn filled_rows(matrix: &mut Matrix) -> Vec<usize> {
    let mut remove = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut count = 0;
        for value in row.iter() {
            if *value == PieceColor::Empty {
                break;
            }
            count += 1;
        }
        if count == matrix[0].len() {
            remove.push(i);
        }
    }
    remove
}

fn remove_rows(matrix: &mut Matrix, remove: Vec<usize>) {
    for row in remove.iter() {
        // Empty the row
        for col in 0..matrix[0].len() {
            matrix[*row][col] = PieceColor::Empty;
        }
        // Swap the row upward
        for current in (1..=*row).rev() {
            matrix.swap(current, current-1);
        }
    }
}

fn read_inputs(input: &Input) -> (MovementAction, RotationAction) {
    let movement_action = match (input.hard_drop, input.soft_drop, input.left, input.right) {
        (true, _, _, _) => MovementAction::HardDrop,
        (_, true, _, _) => MovementAction::SoftDrop,
        (_, _, true, false) => MovementAction::Left,
        (_, _, false, true) => MovementAction::Right,
        _ => MovementAction::None,
    };

    let rotation_action = match (input.rot_cw, input.rot_ccw, input.rot_180) {
        (true, false, false) => RotationAction::RotateCW,
        (false, true, false) => RotationAction::RotateCCW,
        (false, false, true) => RotationAction::Rotate180,
        _ => RotationAction::None,
    };
    (movement_action, rotation_action)
}
