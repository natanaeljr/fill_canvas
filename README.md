FillCanvas
===

_\[Utility Rust Crate\]_

FillCanvas is a wrapper around a matrix with special methods to fill the cells.

FillCanvas provides methods to fill individual cells with a value, or multiple cells from a row/column or just all cells
within an area at once.

The filled cells have a T value != the Default::default().

PS: Initially developed to have EntityIDs (from ECS) filled into a matrix based on the space they occupy on a canvas grid.

Example
---

```rust
fn test() {
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
}
```
