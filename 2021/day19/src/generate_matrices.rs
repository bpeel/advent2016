use std::collections::HashSet;

fn sin(angle: usize) -> i32 {
    [0, 1, 0, -1][angle]
}

fn cos(angle: usize) -> i32 {
    sin((angle + 1) % 4)
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Matrix {
    parts: [i32; 9],
}

impl Matrix {
    fn rotate_x(angle: usize) -> Matrix {
        let s = sin(angle);
        let c = cos(angle);
        Matrix { parts: [1, 0, 0, 0, c, s, 0, -s, c] }
    }

    fn rotate_y(angle: usize) -> Matrix {
        let s = sin(angle);
        let c = cos(angle);
        Matrix { parts: [c, 0, -s, 0, 1, 0, s, 0, c] }
    }

    fn rotate_z(angle: usize) -> Matrix {
        let s = sin(angle);
        let c = cos(angle);
        Matrix { parts: [c, s, 0, -s, c, 0, 0, 0, 1] }
    }

    fn multiply(&mut self, other: &Matrix) {
        let mut temp = Matrix { parts: [0; 9] };

        for x in 0..3 {
            for y in 0..3 {
                let mut sum = 0;

                for i in 0..3 {
                    sum += self.parts[i * 3 + y] * other.parts[x * 3 + i];
                }

                temp.parts[x * 3 + y] = sum;
            }
        }

        *self = temp;
    }
}

fn main() {
    let mut matrices = HashSet::new();

    for i in 0..4 * 4 * 4 {
        let mut matrix = Matrix::rotate_x(i & 3);
        matrix.multiply(&Matrix::rotate_y((i >> 2) & 3));
        matrix.multiply(&Matrix::rotate_z((i >> 4) & 3));

        matrices.insert(matrix);
    }

    let mut matrices: Vec<Matrix> = matrices.into_iter().collect();

    matrices.sort_unstable();

    println!("static TRANSFORMATIONS: [[i32; 9]; N_ORIENTATIONS] = [");

    for matrix in matrices.into_iter() {
        print!("    [");

        for (i, part) in matrix.parts.into_iter().enumerate() {
            print!("{}", part);
            if i < 8 {
                print!(", ");
            }
        }

        println!("],");
    }

    println!("];");
}
