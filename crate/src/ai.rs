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
    .map(|(move_, board)| (move_, minimax(&board, 3, std::i32::MIN, std::i32::MAX, false, white)))
    .collect::<Vec<_>>();

  let max = scored_moves.iter().map(|&(_move_, score)| score).max().expect("No moves available");

  scored_moves.into_iter()
    .filter(|&(_move_, score)| score == max)
    .map(|(move_, _score)| move_)
    .choose(&mut rand::thread_rng()).expect("No moves available")
}

fn minimax(start: &Board, depth: usize, mut alpha: i32, mut beta: i32, maximizing_player: bool, ai_white: bool) -> i32 {
  if depth == 0 { return score_board(start, ai_white); }

  let white = !ai_white ^ maximizing_player;

  if start.is_check_mate(white) { return if maximizing_player { std::i32::MIN } else { std::i32::MAX }; }
  if start.is_stale_mate(white) { return 0; }

  let children = start.pieces(white)
    .flat_map(|(i, _p)|
              start.moves_from(i).unwrap().into_iter()
              .map(move |m| (i, m)))
    .map(|(from, to)| start.move_(from, to))
    .filter(|board| !board.is_check(white));

  if maximizing_player {
    let mut value = std::i32::MIN;
    for board in children {
      value = value.max(minimax(&board, depth - 1, alpha, beta, !maximizing_player, ai_white));
      alpha = alpha.max(value);
      if alpha >= beta { break; }
    }
    value
  } else {
    let mut value = std::i32::MAX;
    for board in children {
      value = value.min(minimax(&board, depth - 1, alpha, beta, !maximizing_player, ai_white));
      beta = beta.min(value);
      if alpha >= beta { break; }
    }
    value
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

  if piece.is_queen() && piece.is_rook() { res -= 20 };
  if piece.is_queen() && piece.is_bishop() { res -= 20 };
  if piece.is_queen() && piece.is_pawn() { res -= 10 };

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
