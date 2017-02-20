fn main() {
    let r: Rect = Rect { w: 10.0, h: 10.0 };
    let p = r.scale(2.0)
     .scale(0.25)
     .scale(5.0)
     .scale(0.7)
     .scale(0.5)
     .scale(0.125)
     .scale(5.0);

    println!("Rectangle w: {:?} h: {:?} area: {:?}", p.w, p.h, p.area());
}

struct Rect {
    w: f64,
    h: f64,
}

impl Rect {
    fn scale(&self, factor: f64) -> Rect {
        Rect {w: self.w * factor, h: self.h * factor, }
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }
}
