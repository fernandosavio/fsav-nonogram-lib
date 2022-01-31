pub type CellValue = bool;
pub type CellGroup = Vec<u8>;


pub struct Grid {
    pub width: u8,
    pub height: u8,
    pub data: [CellValue; 100],
    pub h_groups: Vec<CellGroup>,
    pub v_groups: Vec<CellGroup>,
}

pub struct GameState {
    pub grid: Grid,
    pub current_state: [Option<CellValue>; 100],
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            width: 10,
            height: 10,
            data: [false; 100],
            h_groups: vec![],
            v_groups: vec![],
        }
    }
    pub fn from_array(input_data: [u8; 100]) -> Self {
        let width = 10;
        let height = 10;

        let (h_groups, v_groups) = calculate_groups(&input_data, width, height);

        Grid {
            width: width as u8,
            height: height as u8,
            data: input_data.map(|x| x == 1),
            h_groups,
            v_groups,
        }
    }

    pub fn print(&self) {
        let width = self.width as usize;
        let h_line = "─".repeat(width * 2);

        // Printa a primeira linha
        println!("┌{}┐", h_line);

        for (i, x) in self.data.iter().enumerate() {
            if i % width == 0 {
                print!("│")
            }

            print!(
                "{}",
                match x {
                    true => "██",
                    _ => "  ",
                }
            );

            if i % width == (width - 1) {
                println!("│")
            }
        }

        // Printa a última linha
        println!("└{}┘", h_line);
    }
}

fn calculate_groups(input_data: &[u8], width: usize, height: usize) -> (Vec<CellGroup>, Vec<CellGroup>) {
    let mut h_groups: Vec<CellGroup> = Vec::with_capacity(width);
    let mut v_groups: Vec<CellGroup> = Vec::with_capacity(height);

    let mut acc_h;
    let mut acc_v;

    for i in 0..width {
        acc_h = 0;
        acc_v = 0;

        // Nunca vai precisar mais do que ceil(width/2) elementos
        let mut _h: CellGroup = Vec::with_capacity((width + 1) / 2);
        let mut _v: CellGroup = Vec::with_capacity((height + 1) / 2);

        for j in 0..height {
            let cell_h = &input_data[i * width + j];
            let cell_v = &input_data[j * height + i];

            if *cell_h == 1 {
                acc_h += 1;
            } else if acc_h > 0 {
                _h.push(acc_h);
                acc_h = 0;
            }

            if *cell_v == 1 {
                acc_v += 1;
            } else if acc_v > 0 {
                _v.push(acc_v);
                acc_v = 0;
            }
        }

        if acc_h > 0 {
            _h.push(acc_h);
        }
        if acc_v > 0 {
            _v.push(acc_v);
        }

        h_groups.push(_h);
        v_groups.push(_v);
    }

    (h_groups, v_groups)
}