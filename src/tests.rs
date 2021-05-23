use crate::FillCanvas;

#[test]
fn test1() {
    let mut canvas = FillCanvas::<u64>::new(5, 10);
    assert!(canvas.fill_row(1, 4).is_some());
    assert!(canvas.fill_cell(2, 2, 5).is_some());
    assert!(canvas.fill_cell(3, 3, 9).is_some());
    assert!(canvas.fill_area(4, 0, 0, 1, 3).is_some());
    assert!(canvas.fill_col(5, 7).is_some());

    let expected = vec![
        4, 4, 4, 4, 0, 0, 0, 5, 0, 0, //
        4, 4, 4, 4, 0, 0, 0, 5, 0, 0, //
        0, 0, 0, 0, 0, 2, 0, 5, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, 5, 0, 3, //
        1, 1, 1, 1, 1, 1, 1, 5, 1, 1, //
    ];

    println!("{}", canvas);
    assert_eq!(expected, canvas.matrix);

    assert_eq!(Some(&4), canvas.get(0, 3));
    assert_eq!(Some(&4), canvas.get(1, 2));
    assert_eq!(Some(&5), canvas.get(3, 7));
    assert_eq!(Some(&3), canvas.get(3, 9));
    assert_eq!(Some(&1), canvas.get(4, 9));
    assert_eq!(Some(&0), canvas.get(3, 0));
    assert_eq!(Some(&0), canvas.get(0, 9));

    assert_eq!(Some(&4), canvas.get_filled(0, 3));
    assert_eq!(Some(&4), canvas.get_filled(1, 2));
    assert_eq!(Some(&5), canvas.get_filled(3, 7));
    assert_eq!(Some(&3), canvas.get_filled(3, 9));
    assert_eq!(Some(&1), canvas.get_filled(4, 9));
    assert_eq!(None, canvas.get_filled(3, 0));
    assert_eq!(None, canvas.get_filled(0, 9));

    //
    // Invalid indices:
    //

    assert!(canvas.fill_row(1, 6).is_none());
    assert!(canvas.fill_row(1, 10).is_none());
    assert!(canvas.fill_cell(2, 2, 10).is_none());
    assert!(canvas.fill_cell(3, 5, 9).is_none());
    assert!(canvas.fill_area(4, 4, 5, 1, 3).is_none());
    assert!(canvas.fill_area(4, 1, 3, 6, 9).is_none());
    assert!(canvas.fill_area(4, 1, 3, 2, 11).is_none());
    assert!(canvas.fill_col(5, 10).is_none());
    assert!(canvas.fill_col(5, 15).is_none());

    assert_eq!(expected, canvas.matrix);

    assert_eq!(None, canvas.get(6, 9));
    assert_eq!(None, canvas.get(0, 10));
    assert_eq!(None, canvas.get(10, 5));
    assert_eq!(None, canvas.get(10, 10));

    assert_eq!(None, canvas.get_filled(6, 9));
    assert_eq!(None, canvas.get_filled(0, 10));
    assert_eq!(None, canvas.get_filled(10, 5));
    assert_eq!(None, canvas.get_filled(10, 10));
}