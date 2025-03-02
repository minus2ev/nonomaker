use rand::Rng;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();

    ui.set_cols(30);
    ui.set_rows(20);

    let mut rng = rand::rng();
    let cells: Vec<CellState> = (0..30 * 20)
        .map(|_| match rng.random_range(0..3) {
            0 => CellState::Unknown,
            1 => CellState::Empty,
            _ => CellState::Filled,
        })
        .collect();
    let cells_model = std::rc::Rc::new(slint::VecModel::from(cells));
    ui.set_cells(cells_model.clone().into());

    ui.run().unwrap();
}
