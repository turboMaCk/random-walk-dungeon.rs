extern crate getrandom;
extern crate rand;
extern crate random_walk;

use clap::{App, Arg, ArgMatches};
use rand::{rngs::StdRng, SeedableRng};

fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}

fn from_hex_string(str: &str) -> Option<[u8; 32]> {
    if str.len() < 64 {
        return None;
    }

    let mut res = [0; 32];
    for i in 0..32 {
        match u8::from_str_radix(&str[i..i + 2], 16) {
            Ok(v) => res[i] = v,
            _ => return None,
        }
    }

    Some(res)
}

fn main() -> () {
    let mut dimensions: usize = 20;
    let mut tunnels_count: usize = 50;
    let mut max_tunnel_length: usize = 8;

    // String values of args
    let dimensions_str = dimensions.to_string();
    let tunnels_count_str = tunnels_count.to_string();
    let max_tunnel_length_str = max_tunnel_length.to_string();
    let mut seed: [u8; 32] = [0; 32];
    let _ = getrandom::getrandom(&mut seed);

    let matches: ArgMatches = App::new("Random Walk Dungeon")
        .version("0.1.0")
        .author("Marek Fajkus <marek.faj@gmail.com>")
        .about("Random ASCII dungeon generator")
        .arg(
            Arg::with_name("dimensions")
                .long("dimensions")
                .short("d")
                .default_value(&dimensions_str)
                .value_name("natural number")
                .help("Number of tails per side."),
        )
        .arg(
            Arg::with_name("tunnels")
                .long("tunnels")
                .short("t")
                .default_value(&tunnels_count_str)
                .value_name("natural number")
                .help("Number of tunnels to digg."),
        )
        .arg(
            Arg::with_name("tunnel_len")
                .long("tunnel-length")
                .short("l")
                .default_value(&max_tunnel_length_str)
                .value_name("natural number")
                .help("Maximal length of tunnel in tails."),
        )
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .short("s")
                .default_value(&max_tunnel_length_str)
                .value_name("string")
                .help("32 bytes hex string"),
        )
        .get_matches();

    matches
        .value_of("dimensions")
        .and_then(|v| v.parse::<usize>().ok())
        .map(|v| dimensions = v);

    matches
        .value_of("tunnels")
        .and_then(|v| v.parse::<usize>().ok())
        .map(|v| tunnels_count = v);

    matches
        .value_of("tunnels_len")
        .and_then(|v| v.parse::<usize>().ok())
        .map(|v| max_tunnel_length = v);

    matches
        .value_of("seed")
        .and_then(|v| from_hex_string(v))
        .map(|v| seed = v);

    use random_walk::map::Map;

    let mut rng: StdRng = StdRng::from_seed(seed);
    println!("seed: {}", to_hex_string(seed.to_vec()));

    let map: Map =
        random_walk::generate_map(&mut rng, dimensions, tunnels_count, max_tunnel_length);

    println!("{}", map.ascii_render());
}
