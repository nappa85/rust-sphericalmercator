use std::f64;

//planet radius
//const A: f64 = 6378137.0;//earth
const A: f64 = 3393168.884;//mars
//half planet circumference (radius * PI)
pub const MAXEXTENT: f64 = A * f64::consts::PI;//mars
const D2R: f64 = f64::consts::PI / 180.0;
const R2D: f64 = 180.0 / f64::consts::PI;

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

pub fn forward(c: Coordinate) -> Coordinate {
    Coordinate {
        x: (A * c.x * D2R)
            .max(-MAXEXTENT)
            .min(MAXEXTENT) as f64,
        y: (A * (((f64::consts::PI * 0.25f64) + (0.5f64 * c.y * D2R)).tan()).ln())
            .max(-MAXEXTENT)
            .min(MAXEXTENT) as f64
    }
}

pub fn inverse(c: Coordinate) -> Coordinate {
    Coordinate { 
        x: (c.x * R2D / A) as f64,
        y: ((f64::consts::PI * 0.5) - 2.0 * ((-c.y / A).exp()).atan()) * R2D
    }
}


#[cfg(test)]
mod tests {
    use super::{forward,inverse,Coordinate};

    #[test]
    fn it_works() {
        assert_eq!(
            forward(Coordinate {
                x: -20.0,
                y: -50.0
            }),
            Coordinate {
                //x: -2226389.8158654715,//earth
                //y: -6446275.841017161//earth
                x: -1184439.382040431,//mars
                y: -3429418.747421129//mars
            });
        assert_eq!(
            inverse(Coordinate {
                //x: -2226389.8158654715,//earth
                //y: -6446275.841017161//earth
                x: -1184439.382040431,//mars
                y: -3429418.747421129//mars
            }),
            Coordinate {
                x: -20.0,
                y: -49.99999999999999
            });
    }
}
