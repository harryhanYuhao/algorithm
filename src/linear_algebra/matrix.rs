use std::usize;

#[derive(Clone)]
pub struct Matrix2D {
    size: (u32, u32), // m * n matrix
    val: Vec<f64>,    // the a,b th entry is accessed by a*m + b
}

impl Matrix2D {
    pub fn new(m: u32, n: u32) -> Matrix2D {
        Matrix2D {
            size: (m, n),
            val: vec![0.0; (m * n) as usize],
        }
    }

    pub fn get(&self, a: u32, b: u32) -> f64 {
        self.val[(a * self.size.1 + b) as usize]
    }

    pub fn set(&mut self, a: u32, b: u32, val: f64) {
        self.val[(a * self.size.1 + b) as usize] = val;
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    pub fn transpose(&self) -> Matrix2D {
        let mut res = Matrix2D::new(self.size.1, self.size.0);
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                res.set(j, i, self.get(i, j));
            }
        }
        res
    }

    pub fn mul(&self, other: &Matrix2D) -> Matrix2D {
        assert_eq!(self.size.1, other.size.0);
        let mut res = Matrix2D::new(self.size.0, other.size.1);
        for i in 0..self.size.0 {
            for j in 0..other.size.1 {
                let mut sum = 0.0;
                for k in 0..self.size.1 {
                    sum += self.get(i, k) * other.get(k, j);
                }
                res.set(i, j, sum);
            }
        }
        res
    }

    pub fn add(&self, other: &Matrix2D) -> Matrix2D {
        assert_eq!(self.size, other.size);
        let mut res = Matrix2D::new(self.size.0, self.size.1);
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                res.set(i, j, self.get(i, j) + other.get(i, j));
            }
        }
        res
    }

    pub fn identity(n: u32) -> Matrix2D {
        let mut res = Matrix2D::new(n, n);
        for i in 0..n {
            res.set(i, i, 1.0);
        }
        res
    }

    pub fn random(m: u32, n: u32) -> Matrix2D {
        // let mut rng = rand::thread_rng();
        let mut res = Matrix2D::new(m, n);
        for i in 0..m {
            for j in 0..n {
                res.set(i, j, rand::random::<f64>());
            }
        }
        res
    }

    pub fn mul_scalar(&self, scalar: f64) -> Matrix2D {
        let mut res = Matrix2D::new(self.size.0, self.size.1);
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                res[(i, j)] = self[(i, j)] * scalar;
            }
        }
        res
    }

    pub fn slice(
        &self,
        start_index: (u32, u32),
        end_index: (u32, u32),
    ) -> Matrix2D {
        let mut res = Matrix2D::new(end_index.0 - start_index.0 + 1, end_index.1 - start_index.1 + 1);
        for i in start_index.0..=end_index.0 {
            for j in start_index.1..=end_index.1 {
                res[(i - start_index.0, j - start_index.1)] = self[(i, j)];
            }
        }
        res
    }
}

impl std::ops::Index<(u32, u32)> for Matrix2D {
    type Output = f64;

    fn index(&self, index: (u32, u32)) -> &f64 {
        &self.val[(index.0 * self.size.1 + index.1) as usize]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Matrix2D {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut f64 {
        &mut self.val[(index.0 * self.size.1 + index.1) as usize]
    }
}

impl std::ops::Mul for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, other: Matrix2D) -> Matrix2D {
        Matrix2D::mul(&self, &other)
    }
}

impl std::ops::Add for Matrix2D {
    type Output = Matrix2D;

    fn add(self, other: Matrix2D) -> Matrix2D {
        Matrix2D::add(&self, &other)
    }
}

impl std::fmt::Debug for Matrix2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                write!(f, "{:.5} ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
