use rand::Rng;
use slint::
    { VecModel, ModelRc };

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

    let mut max_hints: i32 = 0;
    let col_hints: Vec<Vec<i32>> = (0..30)
        .map(|_| {
            let mut hints = vec![];
            for _ in 0..rng.random_range(0..6) {
                hints.push(rng.random_range(1..11));
            }
            if hints.len() as i32 > max_hints {
                max_hints = hints.len() as i32;
            }
            hints
        })
        .collect();
    let col_hints_model: Vec<ModelRc<i32>> = col_hints.into_iter()
        .map(|hints| ModelRc::new(VecModel::from(hints)))
        .collect();
    let col_hints_model_rc: ModelRc<ModelRc<i32>> = ModelRc::new(VecModel::from(col_hints_model));
    ui.set_col_hints(col_hints_model_rc);
    ui.set_max_col_hints(max_hints);

    max_hints = 0;
    let row_hints: Vec<Vec<i32>> = (0..20)
        .map(|_| {
            let mut hints = vec![];
            for _ in 0..rng.random_range(0..6) {
                hints.push(rng.random_range(1..11));
            }
            if hints.len() as i32 > max_hints {
                max_hints = hints.len() as i32;
            }
            hints
        })
        .collect();
    let row_hints_model: Vec<ModelRc<i32>> = row_hints.into_iter()
        .map(|hints| ModelRc::new(VecModel::from(hints)))
        .collect();
    let row_hints_model_rc: ModelRc<ModelRc<i32>> = ModelRc::new(VecModel::from(row_hints_model));
    ui.set_row_hints(row_hints_model_rc);
    ui.set_max_row_hints(max_hints);

    ui.run().unwrap();
}
