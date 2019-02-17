use image::ColorType;
use num::Complex;
use std::{io::Write, str::FromStr};

/*
const USAGE: &'static str = "\
mandelbrot.

Usage:
    mandelbrot <filename> <width> <height> <ulr> <uli> <lrr> <lri>
";

#[derive(Debug, Deserialize)]
struct Args {
    pub arg_filename: String,
    pub arg_width: usize,
    pub arg_height: usize,
    // TODO: figure out how deserialize works with Complex
    pub arg_ulr: f64,
    pub arg_uli: f64,
    pub arg_lrr: f64,
    pub arg_lri: f64,
}
*/

/// this is a documentation comment
fn escape_time(c: Complex<f32>, limit: u32) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f32>,
    lower_right: Complex<f32>,
) -> Complex<f32> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex::new(
        upper_left.re + pixel.0 as f32 * width / bounds.0 as f32,
        upper_left.im - pixel.1 as f32 * height / bounds.1 as f32,
    )
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_complex(s: &str) -> Option<Complex<f32>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex::new(re, im)),
        None => None,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f32>,
    lower_right: Complex<f32>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn main() {
    /*
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let (width, height) = (args.arg_width, args.arg_height);
    let bounds = (width, height);
    let upper_left = Complex::new(args.arg_ulr, args.arg_uli);
    let lower_right = Complex::new(args.arg_lrr, args.arg_lri);
    */
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(std::io::stderr(), "wrong number of args mofuck!").unwrap();
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("image dim fucked");
    let upper_left = parse_complex(&args[3]).expect("upper left fucked");
    let lower_right = parse_complex(&args[4]).expect("lower right fucked");
    let (width, height) = bounds;
    let mut pixels = vec![0; width * height];

    let threads = dbg!(num_cpus::get());
    let rows_per_band = height / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * width).collect();
        // blocks until all spawned threads finish
        crossbeam::scope(|spawner| {
            // into_iter gives each iteration exclusive ownership of one band
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let end = band.len() / width;
                let band_bounds = (width, end);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (width, top + end), upper_left, lower_right);
                // move makes closure takes ownership of all variables it uses
                // only the closure may use mutable slice band
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }
    image::save_buffer(
        &args[1],
        &pixels,
        width as u32,
        height as u32,
        ColorType::Gray(8),
    )
    .expect("saving buffer failed");
}
