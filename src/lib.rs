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

#[repr(i32)]
#[derive(Clone, Copy, PartialEq)]
enum PieceType {
  King = 0,
  Queen = 1,
  Rook = 2,
  Bishop = 3,
  Knight = 4,
  Pawn = 5
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
  Black,
  White,
}

#[derive(Clone, Copy)]
struct Piece {
  type_: PieceType,
  color: Color, 
}

#[wasm_bindgen]
pub struct Board {
  pieces: [[Option<Piece>; 8]; 8],
}

#[wasm_bindgen]
impl Board {
  pub fn fresh() -> Self{
    Self{
      pieces: [
        [
          Some(Piece{ type_: PieceType::Rook, color: Color::Black }),
          Some(Piece{ type_: PieceType::Knight, color: Color::Black }),
          Some(Piece{ type_: PieceType::Bishop, color: Color::Black }),
          Some(Piece{ type_: PieceType::Queen, color: Color::Black }),
          Some(Piece{ type_: PieceType::King, color: Color::Black }),
          Some(Piece{ type_: PieceType::Bishop, color: Color::Black }),
          Some(Piece{ type_: PieceType::Knight, color: Color::Black }),
          Some(Piece{ type_: PieceType::Rook, color: Color::Black }),
        ],
        [Some(Piece{ type_: PieceType::Pawn, color: Color::Black }); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece{ type_: PieceType::Pawn, color: Color::White }); 8],
        [
          Some(Piece{ type_: PieceType::Rook, color: Color::White }),
          Some(Piece{ type_: PieceType::Knight, color: Color::White }),
          Some(Piece{ type_: PieceType::Bishop, color: Color::White }),
          Some(Piece{ type_: PieceType::Queen, color: Color::White }),
          Some(Piece{ type_: PieceType::King, color: Color::White }),
          Some(Piece{ type_: PieceType::Bishop, color: Color::White }),
          Some(Piece{ type_: PieceType::Knight, color: Color::White }),
          Some(Piece{ type_: PieceType::Rook, color: Color::White }),
        ],
      ]
    }
  }

  fn piece(&self, loc: Loc) -> Option<Piece> {
    self.pieces[loc.y()][loc.x()]
  }

  pub fn move_(&self, from: i32, to: i32) -> Self {
    let to = Loc(to);
    let from = Loc(from);

    let mut new_pieces = self.pieces.clone();

    let piece = std::mem::replace(&mut new_pieces[from.y()][from.x()], None);
    assert!(piece.is_some());
    new_pieces[to.y()][to.x()] = piece;

    // TODO: capture
    // TODO: merge

    Self{
      pieces: new_pieces,
    }
  }

  pub fn piece_at(&self, loc: i32) -> Option<u32> {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    piece.map(|p| p.type_ as u32)
  }

  pub fn is_white_at(&self, loc: i32) -> bool {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    piece.map(|p| p.color == Color::White).unwrap_or(false)
  }

  pub fn moves_from(&self, loc: i32) -> Option<Box<[i32]>> {
    let loc = Loc(loc);
    let piece = self.piece(loc);

    piece.map(|p| {
      let mut dests = vec![];

      match p.type_ {
        PieceType::Pawn => {
          let dy = match p.color {
            Color::White => -1,
            Color::Black => 1,
          };
          
          // Forward movement
          if let Some(new_loc) = loc.d(0, dy) {
            if self.piece(new_loc).is_none() {
              dests.push(new_loc);

              // Double move from starting position
              if match p.color {
                Color::White => loc.y() == 6,
                Color::Black => loc.y() == 1,
              } {
                if let Some(new_loc) = loc.d(0, dy * 2) {
                  if self.piece(new_loc).is_none() {
                    dests.push(new_loc);
                  }
                }
              }
            }
          }
          
          // Attack
          for &dx in &[-1, 1] {
            if let Some(new_loc) = loc.d(dx, dy) {
              if self.piece(new_loc).is_some() {
                dests.push(new_loc);
              }
            } 
          }
        },
        PieceType::Rook => {
          let ds = [(1, 0), (-1, 0), (0, 1), (0, -1)];

          for &(dx, dy) in &ds {
            let mut loc = loc;

            loop {
              if let Some(new_loc) = loc.d(dx, dy) {
                loc = new_loc;
                dests.push(loc);
                if self.piece(new_loc).is_some() { break; }                
              } else {
                break
              }
            }
          }
        },
        PieceType::Bishop => {
          let ds = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

          for &(dx, dy) in &ds {
            let mut loc = loc;

            loop {
              if let Some(new_loc) = loc.d(dx, dy) {
                loc = new_loc;
                dests.push(new_loc);
                if self.piece(new_loc).is_some() { break; }                
              } else {
                break
              }
            }
          }
        },
        PieceType::Queen => {
          let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

          for &(dx, dy) in &ds {
            let mut loc = loc;

            loop {
              if let Some(new_loc) = loc.d(dx, dy) {
                loc = new_loc;
                dests.push(loc);
                if self.piece(new_loc).is_some() { break; }                
              } else {
                break
              }
            }
          }
        },
        PieceType::King => {
          let ds = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

          for &(dx, dy) in &ds {
            if let Some(new_loc) = loc.d(dx, dy) {
              dests.push(new_loc);
            }
          }
        },
        PieceType::Knight => {
          let ds = [(1, 2), (2, 1),
                    (-1, 2), (2, -1),
                    (1, -2), (-2, 1),
                    (-1, -2), (-2, -1)];

          for &(dx, dy) in &ds {
            if let Some(new_loc) = loc.d(dx, dy) {
              dests.push(new_loc);
            }
          }
        },
      }

      dests.into_iter().map(|i| i.0).collect::<Vec<_>>().into_boxed_slice()
    })
  }
}
