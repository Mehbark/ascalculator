use std::fmt::{Display, Error, Formatter};
fn main() {
    let test = Graph::new((-5., 5.), (-5., 5.), 0.025, |x, y| x.sin() - y);

    println!("{}", test);
}

type Bounds = (f64, f64);
const GRADIENT: [char; 9] = ['.', '\'', ':', ';', '+', '=', 'x', 'X', '$'];

struct Graph {
    x_bounds: Bounds,
    y_bounds: Bounds,
    step_by: f64,

    equation: fn(f64, f64) -> f64,
}

impl Graph {
    pub fn new(
        x_bounds: Bounds,
        y_bounds: Bounds,
        step_by: f64,
        equation: fn(f64, f64) -> f64,
    ) -> Self {
        Self {
            x_bounds,
            y_bounds,
            step_by,
            equation,
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // let mut display = ((self.x_bounds.0)..(self.y_bounds.1))
        //     .map(|n| n.to_string())
        //     .reduce(|a, b| a + &b)
        //     .unwrap();
        // display.push('\n');
        let mut display = String::new();

        let mut y = self.y_bounds.0;
        while y <= self.y_bounds.1 {
            let mut x = self.x_bounds.0;
            while x <= self.x_bounds.1 {
                display.push(
                    *GRADIENT
                        .iter()
                        .rev()
                        .nth((self.equation)(x, y).abs().round() as usize)
                        .unwrap(),
                );
                x += self.step_by;
            }
            y += self.step_by;
            display.push('\n');
        }

        write!(f, "{}", display)
    }
}
