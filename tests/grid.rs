use fsav_nonogram::Grid;

#[test]
fn create_10_10_grid() {
    let g: Grid = Grid::new();

    assert_eq!(g.width, 10);
    assert_eq!(g.height, 10);
    assert_eq!(g.data.len(), 100);
}

#[test]
fn create_grid_from_data() {
    let data: [u8; 100] = [
        0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
        0, 1, 1, 1, 1, 0, 0, 0, 1, 1,
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
    g.print()
}
