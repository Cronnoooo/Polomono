#![allow(non_snake_case)]

use std::collections::HashMap;

pub type PieceType = [Vec<(i32, i32)>; 4];

pub fn create_piece_map() -> HashMap<char, PieceType> {
    let mut piece_list = HashMap::new();
    piece_list.insert(
        'I',
        [
            vec!((0,0), (0,1), (0,2), (0,3)),
            vec!((0,2), (1,2), (2,2), (3,2)),
            vec!((1,0), (1,1), (1,2), (1,3)),
            vec!((0,1), (1,1), (2,1), (3,1)),
        ]);

    piece_list.insert(
        'T',
        [
            vec!((0,1), (1,0), (1,1), (1,2)),
            vec!((0,1), (1,1), (1,2), (2,1)),
            vec!((1,0), (1,1), (1,2), (2,1)),
            vec!((0,1), (1,0), (1,1), (2,1)),
        ]);

    piece_list.insert(
        'O',
        [
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
        ]);

    piece_list.insert(
        '2',
        [
            vec!((0,0), (0,1)),
            vec!((0,1), (1,1)),
            vec!((1,0), (1,1)),
            vec!((0,0), (1,0)),
        ]);


    piece_list
}
