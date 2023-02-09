// Copyright 2021-2023 Mark Cornick <mcornick@mcornick.com>
//
// Permission to use, copy, modify, and/or distribute this software for
// any purpose with or without fee is hereby granted, provided that the
// above copyright notice and this permission notice appear in all
// copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
// WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
// DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA
// OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

extern crate getopts;
use getopts::Options;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("U", "upper", "include uppercase", "true");
    opts.optopt("u", "lower", "include lowercase", "true");
    opts.optopt("d", "digit", "include digits", "true");
    opts.optopt("l", "length", "length to generate", "LENGTH");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let mut length: usize = 16;
    if matches.opt_present("l") {
        length = matches.opt_str("l").unwrap().parse().unwrap();
    }

    if length < 1 {
        panic!("length must be an integer greater than 0")
    };

    let uppercase = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let lowercase = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut pool: Vec<&str> = Vec::new();

    // FIXME: these really should be booleans if getopts can handle the default being true
    if matches.opt_present("U") {
        if matches.opt_str("U").unwrap().to_lowercase() != "false" {
            pool.extend(&uppercase);
        }
    } else {
        pool.extend(&uppercase);
    }

    if matches.opt_present("u") {
        if matches.opt_str("u").unwrap().to_lowercase() != "false" {
            pool.extend(&lowercase);
        }
    } else {
        pool.extend(&lowercase);
    }

    if matches.opt_present("d") {
        if matches.opt_str("d").unwrap().to_lowercase() != "false" {
            pool.extend(&digits);
        }
    } else {
        pool.extend(&digits);
    }
    
    if pool.is_empty() {
        panic!("pool is empty");
    }

    let mut alphabet = Vec::new();
    let repeats = (length / pool.len()) + 1;
    (0..repeats).for_each(|_| {
        alphabet.extend(&pool);
    });

    let mut password = vec![""; length];
    let mut rng = rand::thread_rng();

    (0..length).for_each(|i| {
        let r = rng.gen_range(0..alphabet.len());
        password[i] = alphabet[r];
        alphabet.remove(r);
    });
    println!("{}", password.join(""));
}
