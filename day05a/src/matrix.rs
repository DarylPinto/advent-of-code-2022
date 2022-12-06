/// Produce a new matrix that's the transpose of the input matrix
///
/// Credit to Fawad Ghafoor for this algorithm: stackoverflow.com/a/17428705
pub fn transpose<T: Clone + Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    matrix[0]
        .iter()
        .enumerate()
        .map(|(i, _)| matrix.iter().map(|row| row[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

/// Produce a new matrix that's rotated 90 degrees clockwise
pub fn rotate<T: Clone + Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    transpose(matrix)
        .iter()
        .map(|row| row.iter().cloned().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod transform {
    #[test]
    fn transpose() {
        #[rustfmt::skip]
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
        ];

        #[rustfmt::skip]
        let expected = vec![
            vec!['a', 'd'],
            vec!['b', 'e'],
            vec!['c', 'f'],
        ];

        assert_eq!(super::transpose(&matrix), expected);
    }

    #[test]
    fn rotate() {
        #[rustfmt::skip]
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];

        #[rustfmt::skip]
        let expected = vec![
            vec!['g', 'd', 'a'],
            vec!['h', 'e', 'b'],
            vec!['i', 'f', 'c'],
        ];

        assert_eq!(super::rotate(&matrix), expected);
    }
}
