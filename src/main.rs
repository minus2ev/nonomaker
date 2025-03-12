use std::sync::
    { Arc, Mutex };
use slint::
    { ModelRc, VecModel };
use nonomaker::nonogram::Nonogram;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    let nonogram = Arc::new(Mutex::new(Nonogram::new()));

    let nono = Arc::clone(&nonogram);
    let ui_weak = ui.as_weak();
    ui.on_parse_puzzle(move || {
        let ui = ui_weak.unwrap();
        let code = ui.get_puzzle_code();
        let mut nono = nono.lock().unwrap();
        nono.parse(&code);
        ui.set_cols(nono.n_cols() as i32);
        ui.set_rows(nono.n_rows() as i32);
        ui.set_max_col_hints(nono.get_max_col_hints() as i32);
        ui.set_max_row_hints(nono.get_max_row_hints() as i32);
        ui.set_col_hints(nono.get_col_hints());
        ui.set_row_hints(nono.get_row_hints());
        ui.set_cells(ModelRc::new(VecModel::from(
            vec![CellState::Unknown; nono.n_cols() * nono.n_rows()])
        ));
    });

    ui.run().unwrap();
}
