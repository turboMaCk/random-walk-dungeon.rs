extern crate random_walk;

use clap::{Arg, App, ArgMatches};
use rand::{thread_rng};

fn res_to_opt<T,E>(res: Result<T,E>) -> Option<T> {
    res.map_or_else(|_| None, |x| Some(x))
}

fn main() {
    let mut dimensions: usize = 20;
    let mut tunnels_count: usize = 50;
    let mut max_tunnel_length: usize = 8;
    let mut rng = thread_rng();

    // String values of args
    let dimensions_str = dimensions.to_string();
    let tunnels_count_str = tunnels_count.to_string();
    let max_tunnel_length_str = max_tunnel_length.to_string();

    let matches: ArgMatches = App::new("Random Walk Dungeon")
        .version("0.1.0")
        .author("Marek Fajkus <marek.faj@gmail.com>")
        .about("Random ASCII dungeon generator")
        .arg(Arg::with_name("dimensions")
             .long("dimensions")
             .short("d")
             .default_value(&dimensions_str)
             .value_name("natural number")
             .help("Number of tails per side.")
        )
        .arg(Arg::with_name("tunnels")
             .long("tunnels")
             .short("t")
             .default_value(&tunnels_count_str)
             .value_name("natural number")
             .help("Number of tunnels to digg.")
        )
        .arg(Arg::with_name("tunnel_len")
             .long("tunnel-length")
             .short("l")
             .default_value(&max_tunnel_length_str)
             .value_name("natural number")
             .help("Maximal length of tunnel in tails.")
        )
        .get_matches();

    matches.value_of("dimensions")
        .and_then(|v| res_to_opt(v.parse::<usize>()))
        .map(|v| dimensions = v);

    matches.value_of("tunnels")
        .and_then(|v| res_to_opt(v.parse::<usize>()))
        .map(|v| tunnels_count = v);

    matches.value_of("tunnels_len")
        .and_then(|v| res_to_opt(v.parse::<usize>()))
        .map(|v| max_tunnel_length = v);

    random_walk::ascii_render_map(&random_walk::generate_map(&mut rng, dimensions, tunnels_count, max_tunnel_length));
}
