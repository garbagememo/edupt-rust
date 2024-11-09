extern crate rayon;
#[macro_use] extern crate lazy_static;

mod edupt;

fn main() {
   let samps = if std::env::args().len() == 2 { std::env::args().skip(1).next().unwrap().parse().unwrap() } else { 16 };

    println!("Path tracing renderer: edupt");
    edupt::render(640, 480, samps, 2);
}
