use super::{Board,Loc};
use rand::prelude::*;


pub fn choose_move(board: &Board, white: bool) -> (Loc, Loc) {
  let options = board.pieces(white)
    .flat_map(|(i, _p)|
              board.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .filter(|&(from, to)| !board.move_(from, to).is_check(white))
    .collect::<Vec<_>>();

  *options.choose(&mut rand::thread_rng()).expect("No moves available")
}
