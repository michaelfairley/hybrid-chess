use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::{Board,Loc};
use super::ai;

static mut THE_INTERFACE: Option<Interface> = None;

#[derive(Clone)]
enum State {
  Setup,
  Playing,
  Selected {
    selected_loc: Loc,
    available_moves: Vec<Loc>,
    check_moves: Vec<Loc>,
  },
  Checkmate(bool),
  Stalemate(bool),
  AiMove,
}

pub struct Interface {
  state: State,
  board: Board,
  white_turn: bool,
  prev_move: Option<(Loc, Loc)>,
  white_ai: bool,
  black_ai: bool,
}

impl Interface {
  pub fn setup() -> Self {
    Interface{
      state: State::Setup,
      board: Board::empty(),
      white_turn: true,
      prev_move: None,
      white_ai: false,
      black_ai: false,
    }
  }

  pub fn new(white_ai: bool, black_ai: bool) -> Self {
    Interface{
      state: if white_ai { State::AiMove } else { State::Playing },
      board: Board::fresh(),
      white_turn: true,
      prev_move: None,
      white_ai,
      black_ai,
    }
  }

  pub fn render(&self) {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");

    let message = document.get_element_by_id("message").expect("#message");
    message.set_text_content(None);

    let table = document.get_element_by_id("chess-board").expect("#chess-board").first_element_child().expect("tbody");

    for y in 0..8 {
      let tr = table.children().get_with_index(y).expect("tr");

      for x in 0..8 {
        let td = tr.children().get_with_index(x).expect("td");

        let loc = Loc((y * 8 + x) as i32);

        let cell_color = if ((x + y) % 2) == 0 { "white" } else { "black" };
        td.set_class_name(cell_color);

        if let Some((from, to)) = self.prev_move {
          if from == loc || to == loc {
            td.class_list().add_1("prev-move").unwrap();
          }
        }

        if let State::Selected{selected_loc, ref available_moves, ref check_moves} = self.state {
          if loc == selected_loc {
            td.class_list().add_1("selected").unwrap();
          } else if available_moves.contains(&loc) {
            td.class_list().add_1("available-move").unwrap();
          } else if check_moves.contains(&loc) {
            td.class_list().add_1("check-move").unwrap();
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

        if piece.is_king() && self.board.is_check(piece.is_white()) {
          message.set_text_content(Some("Check!"));
          td.class_list().add_1("check").unwrap();
        }
      }
    }

    if let State::Checkmate(white) = self.state {
      let c = if white { "White" } else { "Black" };
      let m = format!("Checkmate! {} wins", c);
      message.set_text_content(Some(&m));
    } else if let State::Stalemate(_) = self.state {
      message.set_text_content(Some("Stalemate!"));
    } else if let State::AiMove = self.state {
      message.set_text_content(Some("AI is thinking..."));
    }

    let setup = document.get_element_by_id("setup").expect("#setup");
    let setup_class = if matches!(self.state, State::Setup) { "" } else { "hidden" };
    setup.set_class_name(setup_class);
  }

  fn set_state(&mut self, new_state: State) {
    self.state = new_state;
    self.render();
  }

  pub fn do_ai_move(&mut self) {
    assert!(matches!(self.state, State::AiMove));

    let ai_move = ai::choose_minimax(&self.board, self.white_turn);
    self.board = self.board.move_(ai_move.0, ai_move.1);
    self.prev_move = Some(ai_move);

    self.post_move();
  }

  fn post_move(&mut self) {
    if let Some(mate_state) = self.mate_state() {
      self.set_state(mate_state);
    } else {
      self.white_turn = !self.white_turn;

      if (self.white_turn && self.white_ai) || (!self.white_turn && self.black_ai) {
        self.set_state(State::AiMove);
        Self::schedule_ai_move();
      } else {
        self.set_state(State::Playing);
      }
    }
  }

  pub fn clicked(&mut self, x: i32, y: i32) {
    let loc = Loc(y * 8 + x);

    match self.state.clone() {
      State::Playing => {
        let piece = self.board.piece(loc);

        if piece.is_white() == self.white_turn {
          let available_moves = self.board.moves_from(loc);

          if let Some(available_moves) = available_moves {
            let (check_moves, available_moves) = available_moves.into_iter().partition(|&to| self.board.move_(loc, to).is_check(piece.is_white()));
            self.set_state(State::Selected{selected_loc: loc, available_moves, check_moves});
          }
        }
      },
      State::Selected{selected_loc, ref available_moves, ..} => {
        if available_moves.contains(&loc) {
          self.board = self.board.move_(selected_loc, loc);
          self.prev_move = Some((selected_loc, loc));

          self.post_move();
        } else if selected_loc == loc {
          self.set_state(State::Playing);
        }
      },
      State::Checkmate(_) => {},
      State::Stalemate(_) => {},
      State::AiMove => {},
      State::Setup => {},
    };
  }

  fn schedule_ai_move() {
    static mut CALLBACK: Option<wasm_bindgen::prelude::Closure<dyn std::ops::Fn()>> = None;

    let callback = unsafe{
      CALLBACK.get_or_insert_with(|| Closure::wrap(Box::new(move ||{ the_interface().do_ai_move(); }) as Box<Fn()>))
    };

    let window = web_sys::window().expect("window");
    window.set_timeout_with_callback(callback.as_ref().unchecked_ref()).unwrap();
  }

  fn mate_state(&self) -> Option<State> {
    if self.board.is_check_mate(!self.white_turn) {
      Some(State::Checkmate(self.white_turn))
    } else if self.board.is_stale_mate(!self.white_turn) {
      Some(State::Stalemate(self.white_turn))
    } else {
      None
    }
  }

  pub fn clicked_out(&mut self) {
    if matches!(self.state, State::Selected{..}) {
      self.state = State::Playing;
      self.render();
    }
  }
}

pub fn the_interface() -> &'static mut Interface {
  unsafe{ THE_INTERFACE.as_mut().unwrap() }
}

#[wasm_bindgen]
pub fn init() {
  ::utils::set_panic_hook();

  unsafe{ THE_INTERFACE = Some(Interface::setup()); }

  let window = web_sys::window().expect("window");
  let document = window.document().expect("document");

  {
    let clicked_out_callback = Closure::wrap(Box::new(move ||{ the_interface().clicked_out(); }) as Box<Fn()>);
    window.add_event_listener_with_callback("click", clicked_out_callback.as_ref().unchecked_ref()).unwrap();
    clicked_out_callback.forget();
  }

  {
    let table = document.get_element_by_id("chess-board").expect("#chess-board").first_element_child().expect("tbody");
    let clicked_callback = Closure::wrap(Box::new(move |event: web_sys::Event|{
      let td = event.target().unwrap();
      let td = td.dyn_into::<web_sys::HtmlElement>().unwrap();
      let tr = td.parent_element().unwrap();

      let table_children = table.children();
      let y = (0..table_children.length()).map(|i| table_children.get_with_index(i).unwrap()).position(|e| e.is_same_node(Some(&tr))).expect("y");
      let tr_children = tr.children();
      let x = (0..tr_children.length()).map(|i| tr_children.get_with_index(i).unwrap()).position(|e| e.is_same_node(Some(&td))).expect("x");

      the_interface().clicked(x as i32, y as i32);

      event.stop_propagation();
    }) as Box<Fn(web_sys::Event)>);
    let table = document.get_element_by_id("chess-board").expect("#chess-board").first_element_child().expect("tbody");
    table.add_event_listener_with_callback("click", clicked_callback.as_ref().unchecked_ref()).unwrap();
    clicked_callback.forget();
  }

  fn start_new_game(white_ai: bool, black_ai: bool) {
    unsafe{ THE_INTERFACE = Some(Interface::new(white_ai, black_ai)); }
    the_interface().render();
    if white_ai { Interface::schedule_ai_move(); }
  }

  let modes = [
    ("play-as-white", false, true),
    ("play-as-black", true, false),
    ("human-vs-human", false, false),
    ("ai-vs-ai", true, true),
  ];

  for &(button_id, white_ai, black_ai) in &modes {
    let button = document.get_element_by_id(button_id).expect(button_id);
    let callback = Closure::wrap(Box::new(move || start_new_game(white_ai, black_ai)) as Box<Fn()>);
    button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).unwrap();
    callback.forget();
  }

  the_interface().render();
}
