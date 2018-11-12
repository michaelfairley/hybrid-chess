import { Board } from "hybrid-chess";

function render() {
  const table = document.getElementsByTagName("tbody")[0];
  for (var y = 0; y < 8; y++) {
    const tr = table.children[y];
    for (var x = 0; x < 8; x++) {
      const td = tr.children[x];
      
      const loc = y * 8 + x;

      td.className = color(x, y);
      
      const piece = board.piece_at(loc);
      if (piece != null) {
        const white = board.is_white_at(loc);

        const piece_name = {
          0: "king",
          1: "queen",
          2: "rook",
          3: "bishop",
          4: "knight",
          5: "pawn",
        }[piece];

        if (piece_name === undefined) {
          console.log("Don't know what piece " + piece + " is");
        }

        const image_class = (white ? "white" : "black") + "_" + piece_name;

        td.classList.add(image_class);

        // var img = document.createElement("img");
        // img.setAttribute("src", "images/" + image_name);
        // td.appendChild(img);
      }
    }
  }
}

function color(x, y) {
  if ((x + y) % 2 == 0) {
    return "white";
  } else {
    return "black";
  }
}

var board = Board.fresh();
render();
