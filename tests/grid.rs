use fsav_nonogram::{Grid, CellGroup};

#[test]
fn create_10_10_grid() {
    let g = Grid::new();

    assert_eq!(g.width, 10);
    assert_eq!(g.height, 10);
    assert_eq!(g.data.len(), 100);
}

#[test]
fn create_grid_from_data() {
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

    let g = Grid::from_array(data);
    
    let h_groups: Vec<CellGroup> = vec![
        vec![6], 
        vec![4, 1], 
        vec![4, 1, 1], 
        vec![10], 
        vec![2, 1], 
        vec![3, 1], 
        vec![2, 1], 
        vec![1, 2, 1, 1], 
        vec![2, 4, 2], 
        vec![10]
    ];
    assert_eq!(g.h_groups, h_groups);

    let v_groups: Vec<CellGroup> = vec![
        vec![2, 3], 
        vec![3, 2], 
        vec![4, 3, 1], 
        vec![10], 
        vec![2, 3, 2], 
        vec![1, 1, 2], 
        vec![1, 2, 2], 
        vec![1, 2, 3, 1], 
        vec![1, 1, 2], 
        vec![2, 3]
    ];
    
    assert_eq!(g.v_groups, v_groups);
    
}
