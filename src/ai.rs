use super::{Board,Loc,Piece};
use rand::prelude::*;


#[allow(dead_code)]
pub fn choose_minimax(start: &Board, white: bool) -> (Loc, Loc) {
  let scored_moves = start.pieces(white)
    .flat_map(|(i, _p)|
              start.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .map(|(from, to)| ((from, to), start.move_(from, to)))
    .filter(|&(_move_, ref board)| !board.is_check(white))
    .map(|(move_, board)| (move_, minimax(&board, 2, false, white)))
    .collect::<Vec<_>>();

  let max = scored_moves.iter().map(|&(_move_, score)| score).max().expect("No moves available");

  scored_moves.into_iter()
    .filter(|&(_move_, score)| score == max)
    .map(|(move_, _score)| move_)
    .choose(&mut rand::thread_rng()).expect("No moves available")
}

fn minimax(start: &Board, depth: usize, maximizing_player: bool, ai_white: bool) -> i32 {
  if depth == 0 { return score_board(start, ai_white); }

  let white = !ai_white ^ maximizing_player;

  if start.is_check_mate(white) { return if maximizing_player { -10000 } else { 10000 }; }
  if start.is_stale_mate(white) { return 0; }

  let children = start.pieces(white)
    .flat_map(|(i, _p)|
              start.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .map(|(from, to)| start.move_(from, to))
    .filter(|board| !board.is_check(white));

  let scores = children
    .map(|board| minimax(&board, depth - 1, !maximizing_player, ai_white));

  if maximizing_player {
    scores.max().expect("No moves")
  } else {
    scores.min().expect("No moves")
  }
}

#[allow(dead_code)]
pub fn choose_best(start: &Board, white: bool) -> (Loc, Loc) {
  let scored_moves = start.pieces(white)
    .flat_map(|(i, _p)|
              start.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .map(|(from, to)| ((from, to), start.move_(from, to)))
    .filter(|&(_move_, ref board)| !board.is_check(white))
    .map(|(move_, board)| (move_, score_board(&board, white)))
    .collect::<Vec<_>>();

  let max = scored_moves.iter().map(|&(_move_, score)| score).max().expect("No moves available");

  scored_moves.into_iter()
    .filter(|&(_move_, score)| score == max)
    .map(|(move_, _score)| move_)
    .choose(&mut rand::thread_rng()).expect("No moves available")
}

fn score_board(board: &Board, white: bool) -> i32 {
  board.pieces(white).map(|(_, p)| score_piece(p)).sum::<i32>() -
    board.pieces(!white).map(|(_, p)| score_piece(p)).sum::<i32>()
}

fn score_piece(piece: Piece) -> i32 {
  let mut res = 0;

  if piece.is_king() { res += 1000; }
  if piece.is_queen() { res += 100; }
  if piece.is_rook() { res += 70; }
  if piece.is_knight() { res += 60; }
  if piece.is_bishop() { res += 50; }
  if piece.is_pawn() { res += 20; }

  res
}

#[allow(dead_code)]
pub fn choose_random(board: &Board, white: bool) -> (Loc, Loc) {
  board.pieces(white)
    .flat_map(|(i, _p)|
              board.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .filter(|&(from, to)| !board.move_(from, to).is_check(white))
    .choose(&mut rand::thread_rng()).expect("No moves available")
}
