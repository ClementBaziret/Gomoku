struct GridColumnIterator<
    'a,
    T,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    inner_grid: &'a [[T; WIDTH]; HEIGHT],
    index: usize,
    selected_col: usize,
}

impl<'a, T, const W: usize, const H: usize>
    GridColumnIterator<'a, T, W, H>
{
    fn new(grid: &'a [[T; W]; H], col: usize) -> Self {
        Self {
            inner_grid: grid,
            selected_col: col,
            index: 0,
        }
    }
}

impl<'a, T, const W: usize, const HEIGHT: usize> Iterator
    for GridColumnIterator<'a, T, W, HEIGHT>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= HEIGHT {
            None
        } else {
            let ret = &self.inner_grid[self.index][self.selected_col];
            self.index += 1;

            Some(ret)
        }
    }
}

impl<'a, T, const W: usize, const H: usize> GridColumns<'a, T, W, H> {
    fn new(grid: &'a [[T; W]; H]) -> Self {
        Self {
            inner_grid: grid,
            current_column: 0,
        }
    }
}

struct GridColumns<'a, T, const WIDTH: usize, const HEIGHT: usize> {
    inner_grid: &'a [[T; WIDTH]; HEIGHT],
    current_column: usize,
}

impl<'a, T, const WIDTH: usize, const H: usize> Iterator
    for GridColumns<'a, T, WIDTH, H>
{
    type Item = GridColumnIterator<'a, T, WIDTH, H>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= WIDTH {
            None
        } else {
            let ret = GridColumnIterator::new(
                self.inner_grid,
                self.current_column,
            );

            self.current_column += 1;

            Some(ret)
        }
    }
}

#[test]
fn test_column_iterator() {
    let test_grid = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];

    let mut col_it = GridColumnIterator::new(&test_grid, 0);

    assert_eq!(*col_it.next().unwrap(), 1);
    assert_eq!(*col_it.next().unwrap(), 2);
    assert_eq!(*col_it.next().unwrap(), 3);
    assert_eq!(col_it.next(), None);

    let mut col_it = GridColumnIterator::new(&test_grid, 1);

    assert_eq!(*col_it.next().unwrap(), 4);
    assert_eq!(*col_it.next().unwrap(), 5);
    assert_eq!(*col_it.next().unwrap(), 6);
    assert_eq!(col_it.next(), None);
}

#[test]
fn test_columns_iterator() {
    let test_grid = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];
    let mut expected = 0;

    for col in GridColumns::new(&test_grid) {
        for val in col {
            expected += 1;
            assert_eq!(expected, *val);
        }
    }
    assert_eq!(expected, 9);
}
