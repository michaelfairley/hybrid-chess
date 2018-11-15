extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate matches;

mod utils;
pub mod interface;

pub use interface::Interface;

use cfg_if::cfg_if;

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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

const KING: u8       = 1;
const QUEEN: u8      = 2;
const ROOK: u8       = 4;
const BISHOP: u8     = 8;
const KNIGHT: u8     = 16;
const PAWN: u8       = 32;

const WHITE: u8      = 0x80;
const BLACK: u8      = 0x00;
const COLOR_MASK: u8 = 0x80;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct Piece(u8);

impl Piece {
  #[inline] pub fn empty() -> Self { Piece(0) }

  #[inline] pub fn is_empty(self) -> bool { self.0 == 0 }

  #[inline] pub fn is_white(self) -> bool { (self.0 & COLOR_MASK) == WHITE }
  // #[inline] pub fn is_black(self) -> bool { (self.0 & COLOR_MASK) == BLACK }

  #[inline] pub fn is_king(self)   -> bool { (self.0 & KING)   != 0 }
  #[inline] pub fn is_queen(self)  -> bool { (self.0 & QUEEN)  != 0 }
  #[inline] pub fn is_rook(self)   -> bool { (self.0 & ROOK)   != 0 }
  #[inline] pub fn is_bishop(self) -> bool { (self.0 & BISHOP) != 0 }
  #[inline] pub fn is_knight(self) -> bool { (self.0 & KNIGHT) != 0 }
  #[inline] pub fn is_pawn(self)   -> bool { (self.0 & PAWN)   != 0 }

  #[inline] pub fn merge(self, other: Self) -> Self { Piece(self.0 | other.0) }
}

// #[wasm_bindgen]
pub struct Board {
  pieces: [Piece; 64],
}

// #[wasm_bindgen]
impl Board {
  pub fn fresh() -> Self{
    Self{
      pieces: [
        Piece(ROOK | BLACK),
        Piece(KNIGHT | BLACK),
        Piece(BISHOP | BLACK),
        Piece(QUEEN | BLACK),
        Piece(KING | BLACK),
        Piece(BISHOP | BLACK),
        Piece(KNIGHT | BLACK),
        Piece(ROOK | BLACK),
        Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK), Piece(PAWN | BLACK),
        Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0),
        Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0),
        Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0),
        Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0), Piece(0),
        Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE), Piece(PAWN | WHITE),
        Piece(ROOK | WHITE),
        Piece(KNIGHT | WHITE),
        Piece(BISHOP | WHITE),
        Piece(QUEEN | WHITE),
        Piece(KING | WHITE),
        Piece(BISHOP | WHITE),
        Piece(KNIGHT | WHITE),
        Piece(ROOK | WHITE),
      ]
    }
  }

  fn piece(&self, loc: Loc) -> Piece {
    self.pieces[loc.0 as usize]
  }

  pub fn move_(&self, from: Loc, to: Loc) -> Self {
    let mut new_pieces = self.pieces.clone();

    let from_piece = std::mem::replace(&mut new_pieces[from.0 as usize], Piece::empty());
    assert!(!from_piece.is_empty());
    let to_piece = self.pieces[to.0 as usize];
    
    let new_piece = if from_piece.is_white() == to_piece.is_white() {
      from_piece.merge(to_piece)
    } else {
      from_piece
    };
    new_pieces[to.0 as usize] = new_piece;
    
    // TODO: capture

    Self{
      pieces: new_pieces,
    }
  }

  pub fn piece_at(&self, loc: i32) -> u8 {
    self.pieces[loc as usize].0
  }

  pub fn is_check(&self, white: bool) -> bool {
    let king_loc = self.pieces.iter().position(|p| p.is_king() && (p.is_white() == white)).expect("There should be a king") as i32;

    self.pieces.iter()
      .enumerate()
      .filter(|(_, &p)| !p.is_empty())
      .filter(|(_, &p)| p.is_white() != white)
      .any(|(i, _)| self.moves_from(Loc(i as i32)).unwrap().iter().any(|&l| l.0 == king_loc))
  }

  pub fn moves_from(&self, loc: Loc) -> Option<Vec<Loc>> {
    let piece = self.piece(loc);

    if piece.is_empty() { return None };

    let mut dests = vec![];

    if piece.is_pawn() {
      let dy = if piece.is_white() { -1 } else { 1 };
      
      // Forward movement
      if let Some(new_loc) = loc.d(0, dy) {
        if self.piece(new_loc).is_empty() {
          dests.push(new_loc);

          // Double move from starting position
          let starting_row = if piece.is_white() { 6 } else { 1 };
          if starting_row == loc.y() {
            if let Some(new_loc) = loc.d(0, dy * 2) {
              if self.piece(new_loc).is_empty() {
                dests.push(new_loc);
              }
            }
          }
        }
      }
      
      // Attack
      for &dx in &[-1, 1] {
        if let Some(new_loc) = loc.d(dx, dy) {
          if !self.piece(new_loc).is_empty() {
            dests.push(new_loc);
          }
        } 
      }
    }
    if piece.is_rook() {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(loc);
            if !self.piece(new_loc).is_empty() { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece.is_bishop() {
      let ds = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(new_loc);
            if !self.piece(new_loc).is_empty() { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece.is_queen() {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        let mut loc = loc;

        loop {
          if let Some(new_loc) = loc.d(dx, dy) {
            loc = new_loc;
            dests.push(loc);
            if !self.piece(new_loc).is_empty() { break; }                
          } else {
            break
          }
        }
      }
    }
    if piece.is_king() {
      let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

      for &(dx, dy) in &ds {
        if let Some(new_loc) = loc.d(dx, dy) {
          dests.push(new_loc);
        }
      }
    }
    if piece.is_knight() {
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

    dests.sort();
    dests.dedup();

    Some(dests)
  }
}
