import { Board } from "hybrid-chess";

var board = Board.fresh();
var state = {mode: "playing"};

const PIECES = {
  king: 1,
  queen: 2,
  rook: 4,
  bishop: 8,
  knight: 16,
  pawn: 32,
};

const COLOR_MASK = 0x80;
const WHITE = 0x80;
const BLACK = 0x00;

function render() {
  const table = document.getElementsByTagName("tbody")[0];
  for (var y = 0; y < 8; y++) {
    const tr = table.children[y];
    for (var x = 0; x < 8; x++) {
      const td = tr.children[x];
      
      const loc = y * 8 + x;

      td.className = tile_color(x, y);

      if (state.mode == "selected" && state.available_moves.includes(loc)) {
        td.classList.add("available-move");
      }

      td.style.backgroundImage = "";

      // TODO: just access the data off the heap directly
      const piece = board.piece_at(loc);
      if (piece != 0) {
        const white = ((piece & COLOR_MASK) == WHITE);

        for (const piece_name in PIECES) {
          if (PIECES.hasOwnProperty(piece_name)) {
            const piece_bit = PIECES[piece_name];

            if ((piece & piece_bit) != 0) {
              const url = 'url("images/' + (white ? "white" : "black") + '_' + piece_name + '.svg")';

              if (td.style.backgroundImage == "") {
                td.style.backgroundImage = url;
              } else {
                td.style.backgroundImage += ', ' + url;
                td.classList.add("hybrid");
              }
            }
          }
        }

        if (state.mode === "selected") {
          if (state.piece == loc) {
            td.classList.add("selected");
          }
        }
      }
    }
  }
}

function tile_color(x, y) {
  if ((x + y) % 2 == 0) {
    return "white";
  } else {
    return "black";
  }
}

function clicked(x, y) {
  const loc = y * 8 + x;

  const origState = state;

  if (state.mode === "playing") {
    const moves_from = board.moves_from(loc);

    if (moves_from != null) {
      state = {
        mode: "selected",
        piece: loc,
        available_moves: moves_from,
      };
    }
  } else if (state.mode === "selected") {
    if (state.available_moves.includes(loc)) {
      board = board.move_(state.piece, loc);
      state = {mode: "playing"};
    } else if (state.piece == loc) {
      state = {mode: "playing"};
    }
  }

  if (state != origState) { render(); }
}

function set_up_event_handlers() {
  const table = document.getElementsByTagName("tbody")[0];
  table.addEventListener("click", function(event) {
    const td = event.target;
    const tr = td.parentNode;

    const y = Array.from(table.children).indexOf(tr);
    const x = Array.from(tr.children).indexOf(td);

    clicked(x, y);

    event.stopPropagation();
  });

  window.addEventListener("click", function(event) {
    if (state.mode == "selected") {
      state = {mode: "playing"};
      render();
    }
  });
}

set_up_event_handlers();
render();
