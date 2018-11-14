import { Board, Interface } from "hybrid-chess";

var ui = Interface.new();
ui.render();

function set_up_event_handlers() {
  const table = document.getElementsByTagName("tbody")[0];
  table.addEventListener("click", function(event) {
    const td = event.target;
    const tr = td.parentNode;

    const y = Array.from(table.children).indexOf(tr);
    const x = Array.from(tr.children).indexOf(td);

    ui.clicked(x, y);

    event.stopPropagation();
  });

  window.addEventListener("click", function(event) {
    ui.clicked_out();
  });
}

set_up_event_handlers();
