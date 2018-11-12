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

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hybrid-chess!");
}

#[repr(u32)]
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

  fn piece(&self, loc: u32) -> Option<Piece> {
    let x = (loc % 8) as usize;
    let y = (loc / 8) as usize;

    self.pieces[y][x]
  }

  pub fn piece_at(&self, loc: u32) -> Option<u32> {
    let piece = self.piece(loc);

    piece.map(|p| p.type_ as u32)
  }

  pub fn is_white_at(&self, loc: u32) -> bool {
    let piece = self.piece(loc);

    piece.map(|p| p.color == Color::White).unwrap_or(false)
  }

  pub fn moves_from(&self, loc: u32) -> Option<Box<[u32]>> {
    let piece = self.piece(loc);

    piece.map(|p| {
      let mut dests = vec![19];

      dests.push(19u32);

      dests.into_boxed_slice()
    })
  }
}
