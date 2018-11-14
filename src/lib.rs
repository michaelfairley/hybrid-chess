extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Loc(i32);

impl Loc {
  fn x(self) -> usize { (self.0 % 8) as usize }
  fn y(self) -> usize { (self.0 / 8) as usize }

  fn d(self, dx: i32, dy: i32) -> Option<Self> {
    let nx = self.x() as i32 + dx;
    let ny = self.y() as i32 + dy;

    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
      None
    } else {
      Some(Loc(ny * 8 + nx))
    }
  }
}

type Piece = u8;

const KING: u8       = 1;
const QUEEN: u8      = 2;
const ROOK: u8       = 4;
const BISHOP: u8     = 8;
const KNIGHT: u8     = 16;
const PAWN: u8       = 32;

const WHITE: u8      = 0x80;
const BLACK: u8      = 0x00;
const COLOR_MASK: u8 = 0x80;


#[wasm_bindgen]
pub struct Board {
  pieces: [[Piece; 8]; 8],
}

#[wasm_bindgen]
impl Board {
  pub fn fresh() -> Self{
    Self{
      pieces: [
        [
          ROOK | BLACK,
          KNIGHT | BLACK,
          BISHOP | BLACK,
          QUEEN | BLACK,
          KING | BLACK,
          BISHOP | BLACK,
          KNIGHT | BLACK,
          ROOK | BLACK,
        ],
        [PAWN | BLACK; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [PAWN | WHITE; 8],
        [
          ROOK | WHITE,
          KNIGHT | WHITE,
          BISHOP | WHITE,
          QUEEN | WHITE,
          KING | WHITE,
          BISHOP | WHITE,
          KNIGHT | WHITE,
          ROOK | WHITE,
        ],
      ]
    }
  }

  fn piece(&self, loc: Loc) -> Piece {
    self.pieces[loc.y()][loc.x()]
  }

  pub fn move_(&self, from: i32, to: i32) -> Self {
    let to = Loc(to);
    let from = Loc(from);

    let mut new_pieces = self.pieces.clone();

    let piece = std::mem::replace(&mut new_pieces[from.y()][from.x()], 0);
    assert!(piece != 0);
    let dest_piece = std::mem::replace(&mut new_pieces[to.y()][to.x()], piece);
    if dest_piece != 0 && ((dest_piece & COLOR_MASK) == (piece & COLOR_MASK)) {
      new_pieces[to.y()][to.x()] |= dest_piece;
    }

    // TODO: capture

    Self{
      pieces: new_pieces,
    }
  }

  pub fn piece_at(&self, loc: i32) -> u8 {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    piece
  }

  pub fn is_white_at(&self, loc: i32) -> bool {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    Self::is_white(piece)
  }

  fn is_white(piece: Piece) -> bool {
    (piece & COLOR_MASK) == WHITE
  }

  pub fn moves_from(&self, loc: i32) -> Option<Box<[i32]>> {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    if piece == 0 { return None };

    let mut dests = vec![];

    if piece & PAWN != 0 {
      let dy = if Self::is_white(piece) { -1 } else { 1 };
      
      // Forward movement
      if let Some(new_loc) = loc.d(0, dy) {
        if self.piece(new_loc) == 0 {
          dests.push(new_loc);

          // Double move from starting position
          let starting_row = if Self::is_white(piece) { 6 } else { 1 };
          if starting_row == loc.y() {
            if let Some(new_loc) = loc.d(0, dy * 2) {
              if self.piece(new_loc) == 0 {
                dests.push(new_loc);
              }
            }
          }
        }
      }
      
      // Attack
      for &dx in &[-1, 1] {
        if let Some(new_loc) = loc.d(dx, dy) {
          if self.piece(new_loc) != 0 {
            dests.push(new_loc);
          }
        } 
      }
    }
    if piece & ROOK != 0 {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(loc);
            if self.piece(new_loc) != 0 { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece & BISHOP != 0 {
      let ds = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(new_loc);
            if self.piece(new_loc) != 0 { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece & QUEEN != 0 {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(loc);
            if self.piece(new_loc) != 0 { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece & KING != 0 {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        if let Some(new_loc) = loc.d(dx, dy) {
          dests.push(new_loc);
        }
      }
    }
    if piece & KNIGHT != 0 {
      let ds = [(1, 2), (2, 1),
                (-1, 2), (2, -1),
                (1, -2), (-2, 1),
                (-1, -2), (-2, -1)];

      for &(dx, dy) in &ds {
        if let Some(new_loc) = loc.d(dx, dy) {
          dests.push(new_loc);
        }
      }
    }

    Some(dests.into_iter().map(|i| i.0).collect::<Vec<_>>().into_boxed_slice())
  }
}
