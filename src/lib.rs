#[derive(Clone, Copy)]
pub enum CellContent {
    None,
    Filled,
    NotFilled,
}

pub struct Grid {
    pub width: u8,
    pub height: u8,
    pub data: [CellContent; 100],
    pub h_groups: Vec<Vec<u8>>,
    pub v_groups: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            width: 10,
            height: 10,
            data: [CellContent::None; 100],
            h_groups: vec![],
            v_groups: vec![],
        }
    }
    pub fn from_array(input_data: [u8; 100]) -> Grid {
        let width = 10;
        let height = 10;

        let mut h_groups: Vec<Vec<u8>> = Vec::with_capacity(width);
        let mut v_groups: Vec<Vec<u8>> = Vec::with_capacity(height);

        let mut acc_h;
        let mut acc_v;
        for i in 0..width {
            acc_h = 0;
            acc_v = 0;

            // Nunca vai precisar mais do que ceil(width/2) elementos
            let mut _h: Vec<u8> = Vec::with_capacity((width + 1) / 2);
            let mut _v: Vec<u8> = Vec::with_capacity((height + 1) / 2);

            for j in 0..height {
                let cell_h = input_data[i * width + j];
                let cell_v = input_data[j * height + i];

                if cell_h == 1 {
                    acc_h += 1;
                } else if acc_h > 0 {
                    _h.push(acc_h);
                    acc_h = 0;
                }

                if cell_v == 1 {
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

        Grid {
            width: width as u8,
            height: height as u8,
            data: input_data.map(|x| match x {
                0 => CellContent::None,
                1 => CellContent::Filled,
                _ => CellContent::NotFilled,
            }),
            h_groups: h_groups,
            v_groups: v_groups,
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
                    CellContent::None => "  ",
                    CellContent::Filled => "██",
                    CellContent::NotFilled => "  ",
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
