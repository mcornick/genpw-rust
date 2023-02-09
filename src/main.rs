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

use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = if args.len() > 1 {
        args[1].parse().unwrap_or(16)
    } else {
        16
    };
    if length < 1 {
        panic!("length must be an integer greater than 0")
    };

    let pool = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];
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
