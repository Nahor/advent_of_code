use rustc_hash::FxHashMap;

pub const EPSILON: f64 = 0.0001;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,

    dim: (usize, usize),
}

impl Matrix {
    pub fn swap_row(&mut self, row1: usize, row2: usize) -> &Self {
        self.data.swap(row1, row2);
        self
    }

    pub fn gaussian_elimination(mut self) -> GaussianMatrix {
        let mut h = 0;
        let mut k = 0;

        let m = self.dim.0;
        let n = self.dim.1;

        while h < m && k < n {
            let (i_max, pivot_v) = self.data[h..m]
                .iter()
                .enumerate()
                .map(|(i, row)| (i + h, row[k].abs()))
                .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
                .unwrap();

            if pivot_v < EPSILON {
                // No pivot in this column, pass to next column
                k += 1;
            } else {
                self.swap_row(h, i_max);

                // Reduce the row to make the pivot value 1.0 (for "reduced row echelon")
                let f = self.data[h][k];
                self.data[h][k..n].iter_mut().for_each(|v| *v /= f);

                // Do for all rows below pivot (also do the rows above for a
                // "reduced row echelon")
                for i in 0..m {
                    if i == h {
                        continue;
                    }

                    // Because we reduced the pivot row, the pivot cell is 1.0
                    // so no need for a division
                    let f = self.data[i][k];

                    // Fill with zeros the lower part of pivot column
                    // (and the upper part for the "reduced row echelon")
                    self.data[i][k] = 0.0;

                    // Do for all remaining elements in current row
                    for j in (k + 1)..n {
                        self.data[i][j] -= self.data[h][j] * f;
                    }
                }
                // Increase pivot row and column
                h += 1;
                k += 1;
            }
        }
        GaussianMatrix {
            data: self.data,
            dim: self.dim,
        }
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(data: Vec<Vec<f64>>) -> Self {
        let m = data.len();
        let n = if m != 0 { data[0].len() } else { 0 };
        assert!(&data[1..].iter().all(|row| row.len() == n));

        Self { data, dim: (m, n) }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.dim == other.dim
    }
}

#[derive(Debug, Default, Clone)]
pub struct GaussianMatrix {
    data: Vec<Vec<f64>>,
    dim: (usize, usize),
}

impl GaussianMatrix {
    pub fn get_independent_vars(&self) -> Vec<usize> {
        // The last column is not a variable but the constant vector
        let var_count = self.dim.1 - 1;
        let mut vars = Vec::with_capacity(var_count);

        let mut row = 0;
        let mut col = 0;
        while (row < self.dim.0) && (col < var_count) {
            if self.data[row][col].abs() < EPSILON {
                vars.push(col);
                col += 1;
            } else {
                row += 1;
                col += 1;
            }
        }
        vars.extend(self.dim.0..var_count);

        vars
    }

    pub fn get_var_value(&self, var: usize, independent_var: &FxHashMap<usize, f64>) -> f64 {
        // Asking for the value of an independent var
        if let Some(&v) = independent_var.get(&var) {
            return v;
        }

        // Find the row
        // (by construction, anything below the diagonal is 0, so the row can't be below `var`)
        let (row_idx, row) = self.data[0..=var]
            .iter()
            .enumerate()
            .find(|(_, row)| (row[var] - 1.0).abs() < EPSILON)
            .unwrap_or_else(|| panic!("no row found for {var}"));
        let lhs = row[0..(self.dim.1 - 1)]
            .iter()
            .enumerate()
            .skip(var + 1)
            .filter(|(_, val)| val.abs() > EPSILON)
            .map(|(idx, val)| {
                val * independent_var.get(&idx).copied().unwrap_or_else(|| {
                    panic!("var {var} (row {row_idx}) depends on other dependent var {idx}")
                })
            })
            .sum::<f64>();

        row[self.dim.1 - 1] - lhs
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn rounding_eq(matrix1: &GaussianMatrix, matrix2: &Matrix) -> bool {
        matrix1
            .data
            .iter()
            .flat_map(|row| row.iter())
            .zip(matrix2.data.iter().flat_map(|row| row.iter()))
            .all(|(v1, v2)| (v1 - v2).abs() < EPSILON)
    }

    #[rstest]
    // #[case(
    //     vec![vec![1.0, 3.0, 1.0, 9.0], vec![1.0, 1.0, -1.0, 1.0], vec![3.0, 11.0, 5.0, 35.0]],
    //     vec![vec![1.0, 0.0, -2.0, -3.0], vec![0.0, 1.0, 1.0, 4.0], vec![0.0, 0.0, 0.0, 0.0]]
    // )]
    // #[case(
    //     vec![vec![2.0, 1.0, -1.0, 8.0], vec![-3.0, -1.0, 2.0, -11.0], vec![-2.0, 1.0, 2.0, -3.0]],
    //     vec![vec![2.0, 1.0, -1.0, 8.0], vec![0.0, 0.5, 0.5, 1.0], vec![0.0, 0.0, -1.0, 1.0]]
    // )]
    // #[case(
    //     vec![vec![-3.0, -1.0, 2.0, -11.0], vec![-2.0, 1.0, 2.0, -3.0], vec![2.0, 1.0, -1.0, 8.0]],
    //     vec![vec![2.0, 1.0, -1.0, 8.0], vec![0.0, 0.0, -1.0, 1.0], vec![0.0, 0.5, 0.5, 1.0]]
    // )]
    #[case(
        vec![
            vec![1.0, 2.0, -3.0, -1.0, 0.0],
            vec![0.0, -3.0, 2.0, 6.0, -8.0],
            vec![-3.0, -1.0, 3.0, 1.0, 0.0],
            vec![2.0, 3.0, 2.0, -1.0, -8.0],
        ],
        vec![
            vec![1.0, 0.0, 0.0, 0.0, -1.0],
            vec![0.0, 1.0, 0.0, 0.0, -2.0],
            vec![0.0, 0.0, 1.0, 0.0, -1.0],
            vec![0.0, 0.0, 0.0, 1.0, -2.0],
        ]
    )]
    #[case(
        vec![
            vec![3.0, -1.0, 7.0, 1.0],
            vec![6.0, 0.0, 1.0, 2.0],
        ],
        vec![
            vec![1.0, 0.0, 1.0/6.0, 1.0/3.0],
            vec![0.0, 1.0, -13.0/2.0, 0.0],
        ]
    )]
    fn single(#[case] input: Vec<Vec<f64>>, #[case] expected: Vec<Vec<f64>>) {
        let matrix = Matrix::from(input);
        let expected = Matrix::from(expected);

        let gaussian = matrix.clone().gaussian_elimination();

        assert!(
            rounding_eq(&gaussian, &expected),
            "not equals:\n\t{gaussian:?}\n\t{expected:?}"
        );
    }
}
