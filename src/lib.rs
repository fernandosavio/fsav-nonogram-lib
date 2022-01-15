#[derive(Clone, Copy)]
pub enum CellContent {
    None,
    Filled,
    NotFilled,
}


pub struct Grid{
    pub width: u8,
    pub height: u8,
    pub data: [CellContent; 100]
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            width: 10,
            height: 10,
            data: [CellContent::None; 100]
        }
    }
    pub fn from_array(input_data: [u8; 100]) -> Grid {
        Grid {
            width: 10,
            height: 10,
            data: input_data.map(|x| {
                match x {
                    0 => CellContent::None,
                    1 => CellContent::Filled,
                    _ => CellContent::NotFilled
                }
            })
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

            print!("{}", match x {
                CellContent::None => "  ",
                CellContent::Filled => "██",
                CellContent::NotFilled => "  ",
            });

            if i % width == (width - 1){
                println!("│")
            }
        }

        // Printa a última linha
        println!("└{}┘", h_line);
    }
}
