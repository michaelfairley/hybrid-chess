use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::{Board,Loc};

enum State {
  Playing,
  Selected(Loc, Vec<Loc>),
}

#[wasm_bindgen]
pub struct Interface {
  state: State,
  board: Board,
}

#[wasm_bindgen]
impl Interface {
  pub fn new() -> Self {
    Interface{
      state: State::Playing,
      board: Board::fresh(),
    }
  }
  
  pub fn render(&self) {
    ::utils::set_panic_hook();
    
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");

    let table = document.get_element_by_id("chess-board").expect("#chess-board").first_element_child().expect("tbody");

    for y in 0..8 {
      let tr = table.children().get_with_index(y).expect("tr");
      
      for x in 0..8 {
        let td = tr.children().get_with_index(x).expect("td");

        let loc = Loc((y * 8 + x) as i32);

        let cell_color = if ((x + y) % 2) == 0 { "white" } else { "black" };
        td.set_class_name(cell_color);

        if let State::Selected(selected_loc, ref available_moves) = self.state {
          if loc == selected_loc {
            td.class_list().add_1("selected").unwrap();
          } else if available_moves.contains(&loc) {
            td.class_list().add_1("available-move").unwrap();
          }
        }

        let td = td.dyn_into::<web_sys::HtmlElement>().unwrap();

        let style = td.style();

        style.remove_property("background-image").unwrap();

        let piece = self.board.piece(loc);

        if piece.is_empty() { continue; }

        let mut images = vec![];
        let color = if piece.is_white() { "white" } else { "black" };
        let image_url = |piece| { format!("url('images/{}_{}.svg')", color, piece) };
        
        if piece.is_king() { images.push(image_url("king")) };
        if piece.is_queen() { images.push(image_url("queen")) };
        if piece.is_rook() { images.push(image_url("rook")) };
        if piece.is_bishop() { images.push(image_url("bishop")) };
        if piece.is_knight() { images.push(image_url("knight")) };
        if piece.is_pawn() { images.push(image_url("pawn")) };

        let background_image = images.join(", ");
        style.set_property("background-image", &background_image).unwrap();

        if images.len() > 1 {
          td.class_list().add_1("hybrid").unwrap();
        }
      }
    }
  }

  pub fn clicked(&mut self, x: i32, y: i32) {
    let loc = Loc(y * 8 + x);

    let new_state = match self.state {
      State::Playing => {
        let available_moves = self.board.moves_from(loc);

        if let Some(available_moves) = available_moves {
          Some(State::Selected(loc, available_moves))
        } else { None }
      },
      State::Selected(selected_loc, ref available_moves) => {
        if available_moves.contains(&loc) {
          self.board = self.board.move_(selected_loc, loc);
          Some(State::Playing)
        } else if selected_loc == loc {
          Some(State::Playing)
        } else { None }
      },
    };

    if let Some(new_state) = new_state {
      self.state = new_state;
      self.render();
    }
  }

  pub fn clicked_out(&mut self) {
    if matches!(self.state, State::Selected(_, _)) {
      self.state = State::Playing;
      self.render();
    }
  }
}
