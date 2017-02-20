fn main() {
    let r: Rect = RectBuilder::new()
                    .width(10.0)
                    .height(10.0)
                    .finalize();

    let p = r.scale(2.0)
     .scale(0.25)
     .scale(5.0)
     .scale(0.7)
     .scale(0.5)
     .scale(0.125)
     .scale(5.0);

    println!("Rectangle w: {:?} h: {:?} area: {:?}", p.w, p.h, p.area());
}

// Rect struct
struct Rect {
    w: f64,
    h: f64,
}

// Define a builder struct, this can be used to construct our Rects
struct RectBuilder {
    w: f64,
    h: f64
}


impl Rect {
    fn scale(&self, factor: f64) -> Rect {
        Rect {w: self.w * factor, h: self.h * factor, }
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }
}

impl RectBuilder {
    fn new() -> RectBuilder {
        RectBuilder { w: 0.0, h: 0.0 }
    }

    // Each function for the builder returns a mutable reference to the builder we are using
    fn width(&mut self, w: f64) -> &mut RectBuilder {
        self.w = w;
        self
    }

    fn height(&mut self, h: f64) -> &mut RectBuilder {
        self.h = h;
        self
    }

    // the finalize method returns the type we have constructed
    fn finalize(&mut self) -> Rect {
        Rect {w: self.w, h: self.h }
    }
}
