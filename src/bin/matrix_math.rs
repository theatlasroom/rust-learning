// Basic 2d matrix calculations

fn main() {
    let a = Matrix2d{ x: 10.0, y: 5.0 };
    let b = Matrix2d{ x: 5.0, y: 20.0 };

    let add = a.add(&b);
    let sub = a.subtract(&b);
    println!("Matrix a ({}, {})", a.x, a.y);
    println!("Matrix b ({}, {})", b.x, b.y);
    println!("a+b ({}, {})", add.x, add.y);
    println!("a-b ({}, {})", sub.x, sub.y);
}

struct Matrix2d {
    x: f64,
    y: f64,
}

impl Matrix2d {
    fn subtract(&self, target: &Matrix2d) -> Matrix2d {
        return Matrix2d {
            x: self.x - target.x,
            y: self.y - target.y,
        };
    }

    fn add(&self, target: &Matrix2d) -> Matrix2d {
        return Matrix2d {
            x: self.x + target.x,
            y: self.y + target.y,
        };
    }

    fn divide(&self, scalar: f64) -> Matrix2d {
        return self.multiply(1.0/scalar);
    }
    // fn transpose(&self, x: ) { unimplemented!() }
    fn multiply(&self, scalar: f64) -> Matrix2d {
        return Matrix2d{ x: self.x * scalar, y: self.y * scalar };
    }

    fn scalar_multiply(&self) -> f64 { unimplemented!() }
    fn determinant(&self) -> f64 { unimplemented!() }
}
