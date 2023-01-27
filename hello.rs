use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct MinMax(i32, i32);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Point2D {
    x: f64,
    y: f64
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)

    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({r}, {g}, {b}) 0x{r:0>2X}{g:0>2X}{b:0>2X}", r=self.red, g=self.green, b=self.blue)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Hello world!");

    println!("`x` is {}", 5 + 5);
    let x = 10;
    println!("{x}");

    println!("{}", MinMax(1, 5));
    println!("{}", Point2D { x: 12.5, y: 64.5 });
    println!("{}", Complex { real: 3.3, imag: 7.2 });
    println!("{:?}", Complex { real: 3.3, imag: 7.2 });

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 }
    ].iter() {
        println!("{:?}", *color);
        println!("{}", *color);
    }
}
