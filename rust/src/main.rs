use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "iterations")]
    iterations: usize,

    /// compute using all threads
    #[structopt(short, long)]
    multi: bool,
}

fn main() {
    let args = Opt::from_args();

    if args.multi {
        println!("{}", calc_multi_threaded(args.iterations));
    } else {
        println!("{}", calc_single_threaded(args.iterations));
    }
}

fn calc_round(i: usize) -> f64 {
    if i % 2 == 0 {
        1.0 / ((i as f64) * 2.0 + 1.0)
    } else {
        -1.0 / ((i as f64) * 2.0 + 1.0)
    }
}

fn calc_single_threaded(iterations: usize) -> f64 {
    (0..iterations).map(calc_round).sum::<f64>() * 4.0
}

fn calc_multi_threaded(iterations: usize) -> f64 {
    (0..iterations).into_par_iter().map(calc_round).sum::<f64>() * 4.0
}
