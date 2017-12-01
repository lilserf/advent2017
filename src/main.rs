use std::fs::File;
use std::io::prelude::*;

use std::time::{Duration, Instant};

#[allow(dead_code)]
fn day1()
{
	let input = get_input("day1.txt");

	let mut iter = input.chars().peekable();
	let mut sum = 0;

	while let Some(c) = iter.next()
	{
		match iter.peek()
		{
			Some(n) => 
			{
				let n = n.to_digit(10).unwrap();
				let c = c.to_digit(10).unwrap();
				if n == c 
				{
					sum += n;
				}
			}
			None =>
			{
				// Compare to first
				let n = input.chars().nth(0).unwrap().to_digit(10).unwrap();
				let c = c.to_digit(10).unwrap();
				if n == c
				{
					sum += n;
				}
			}
		}
	}

	println!("Sum was {}", sum);
}

#[allow(dead_code)]
fn day1b()
{
	let input = get_input("day1.txt");
	let digit_vec: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

	let mut sum = 0;

	let num_digits = digit_vec.len();
	for i in 0..num_digits
	{
		let c = digit_vec[i];
		let halfway = (i + num_digits / 2) % num_digits;
		let n = digit_vec[halfway];

		if c == n
		{
			sum += c;
		}
	}

	println!("Sum was {}", sum);
}

fn get_input(name:&str) -> String
{
	let prefix = String::from("input/");
	let filename = prefix+name;
	let mut file = File::open(filename).unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();
	return input;
}

fn as_msecs(d:Duration) -> f32
{
	let mut msecs:f32 = (d.as_secs() * 1000) as f32;
	msecs = msecs + (d.subsec_nanos() as f32 / 1000000.0) as f32;
	return msecs;
}

fn main() 
{
	let now = Instant::now();
    //day1();
	day1b();

	println!("Elapsed: {} ms", as_msecs(now.elapsed()));
}
