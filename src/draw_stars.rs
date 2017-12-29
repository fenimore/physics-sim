extern crate find_folder;
extern crate piston_window;
extern crate csv;

use std::error::Error;
use std::fs::File;

const DIMENSION: u32 = 1000;

fn draw_stars(c: Context, g: &mut G2d) {
    let mut stars = Vec::new();
    match read_csv() {
        Ok(s) => stars.extend(s),
        Err(b) => println!("FaileD"),
    }
    for star in stars.iter() {
        Ellipse::new([1.0, 1.0, 1.0, star.2*0.01])
            .draw(
                [star.0*20.0, star.1*20.0, 1.0, 1.0],
                &c.draw_state, c.transform, g
            );
    }
}

type Star = (f64, f64, f32);

fn read_csv() -> Result<Vec<Star>, Box<Error>> {
    let file = File::open("celestial.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut stars = Vec::new();
    for result in rdr.deserialize() {
        let star: Star = result?;
        stars.push(star);
    }
    Ok(stars)
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "piston: draw_state", Size{width: DIMENSION, height: DIMENSION+80},
        ).exit_on_esc(true).samples(4).build().unwrap();
    // piston window lazy means that only events will tricker a step
    window.set_lazy(false);



    let mut solar_system = big_bang();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.129, 0.1468, 0.168, 1.0], g);
            g.clear_stencil(0);

            draw_stars(c, g);
        });
    }
}
