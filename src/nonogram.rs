use slint::
    { VecModel, ModelRc };

slint::include_modules!();

pub struct Nonogram {
    col_hints: Vec<Vec<i32>>,
    row_hints: Vec<Vec<i32>>,
    solution: Vec<CellState>,
}

impl Nonogram {
    pub fn new() -> Self
    {
        Nonogram {
            col_hints: vec![],
            row_hints: vec![],
            solution: vec![],
        }
    }

    pub fn n_cols(&self) -> usize {
        self.col_hints.len()
    }

    pub fn n_rows(&self) -> usize {
        self.row_hints.len()
    }

    pub fn get_max_col_hints(&self) -> usize {
        self.col_hints.iter().map(|v| v.len()).max().unwrap_or(0)
    }

    pub fn get_max_row_hints(&self) -> usize {
        self.row_hints.iter().map(|v| v.len()).max().unwrap_or(0)
    }

    fn make_2d_model<T: Clone + 'static>(v: Vec<Vec<T>>) -> ModelRc<ModelRc<T>> {
        ModelRc::new(VecModel::from(v.into_iter().map(|h| {
            ModelRc::new(VecModel::from(h))
        }).collect::<Vec<_>>()))
    }

    pub fn get_col_hints(&self) -> ModelRc<ModelRc<i32>> {
        Nonogram::make_2d_model(self.col_hints.clone())
    }

    pub fn get_row_hints(&self) -> ModelRc<ModelRc<i32>> {
        Nonogram::make_2d_model(self.row_hints.clone())
    }

    // sample:
    // 1,2,2,3|1,1,1|2,1,1,1,1,1|1,1,1,1|2,11,1|1,2,1|1,1,3,1,1,2,1|1,2,1|1,10,1|1,2,1|1,1,1,3|1,3|7|5,1|6,1,2,1,1|4,1,1,1,1,3,1,1|6,1,2|5,1,1,1|5,1,1|1/1|1|2,1,1|2,1|1,1|2,1,3|1,1,1,1|5,3|1,1,1,2,2|1,1,6,5|1,10,1|1,1,1,7|1,1,1,1,2,2|1,1,1,1,1,1,1,1|1,1,1,1,3,1|1,1,1,1,1,1,8|1,1,1,1,1|12,2,2|2|1,2,1,1,2,2,2
    pub fn parse(&mut self, code: &str) {
        let (cols, rows) = code.split_once('/').unwrap_or(("", ""));
        self.col_hints = cols.split('|').map(|c| {
            c.split(',').map(|s| s.parse().unwrap_or(0)).collect()
        }).collect();
        self.row_hints = rows.split('|').map(|r| {
            r.split(',').map(|s| s.parse().unwrap_or(0)).collect()
        }).collect();
        self.solution = vec![];
    }
}