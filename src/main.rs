use fsav_nonogram::Grid;

fn main() {
    let data: [u8; 100] = [
        0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
        0, 1, 1, 1, 1, 0, 0, 0, 1, 0,
        1, 1, 1, 1, 0, 0, 0, 1, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
        0, 0, 0, 1, 1, 0, 1, 0, 0, 0,
        0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 
        0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 
        1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 
        1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let g: Grid = Grid::from_array(data);
    g.print();

    println!("Grupos horizontais: {:?}", g.h_groups);
    println!("Grupos verticais: {:?}", g.v_groups);
}