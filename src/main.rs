extern crate random_walk;

use clap::{Arg, App, ArgMatches};

fn main() {
    let mut dimmensions: usize = 20;
    let mut tunnels_count: usize = 50;
    let mut max_tunnel_length: usize = 8;

    let matches: ArgMatches = App::new("Random Walk Dungeon")
        .version("0.1.0")
        .author("Marek Fajkus <marek.faj@gmail.com>")
        .about("Random ASCII dungeon generator")
        .arg(Arg::with_name("dimmensions")
             .long("dimmensions")
             .short("d")
             .default_value("20")
             .value_name("natural number")
             .help("Number of tails per side.")
        )
        .arg(Arg::with_name("tunnels")
             .long("tunnels")
             .short("t")
             .default_value("50")
             .value_name("natural number")
             .help("Number of tunnels to digg.")
        )
        .arg(Arg::with_name("tunnel_len")
             .long("tunnel-length")
             .short("l")
             .default_value("8")
             .value_name("natural number")
             .help("Maximal length of tunnel in tails.")
        )
        .get_matches();

    matches.value_of("dimmensions")
        .and_then(|v| v.parse::<usize>().map_or_else(|_| None, |x| Some(x)))
        .map(|v| dimmensions = v);

    matches.value_of("tunnels")
        .and_then(|v| v.parse::<usize>().map_or_else(|_| None, |x| Some(x)))
        .map(|v| tunnels_count = v);

    matches.value_of("tunnels_len")
        .and_then(|v| v.parse::<usize>().map_or_else(|_| None, |x| Some(x)))
        .map(|v| max_tunnel_length = v);

    random_walk::ascii_render_map(&random_walk::generate_map(dimmensions, tunnels_count, max_tunnel_length));
}
