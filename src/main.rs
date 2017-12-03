use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

#[allow(dead_code)]
fn find_common_divisor(nums: &Vec<u32>) -> u32
{
	for (i, el1) in nums.iter().enumerate()
	{
		for el2 in &nums[i+1..]
		{
			if el1 % el2 == 0
			{
				return el1 / el2;
			}
			else if el2 % el1 == 0
			{
				return el2 / el1;
			}
		}
	}

	return 0;
}

#[allow(dead_code)]
fn day2()
{
	let input = get_input("day2.txt");

	let mut sum = 0;
	let mut sumb = 0;
	for l in input.lines()
	{
		let nums: Vec<u32> = l.split("\t").map(|w| w.parse().unwrap()).collect();

		{
			// part a
			let mut min = nums[0];
			let mut max = nums[0];
			for &n in &nums
			{
				if n > max
				{
					max = n;
				}
				if n < min
				{
					min = n;
				}
			}
			sum += max-min;
		}

		sumb += find_common_divisor(&nums);
	}

	println!("Checksum is {}", sum);
	println!("Part b is {}", sumb);
}

fn biggest_in_ring(ring:i32) -> i32
{
	let size = ring * 8;

	if ring > 0
	{
		return biggest_in_ring(ring-1) + size;
	}
	else
	{
		return 1;
	}
}

fn coord_for_input(input:i32) -> (i32, i32)
{
	let mut ring = 1;
	
	while biggest_in_ring(ring) < input
	{
		ring += 1;
	}

	let ring_end = biggest_in_ring(ring);
	let ring_start = ring_end - (ring * 8) + 1;
	let ring_side = ring * 2 + 1;

	let index_in_ring = input - ring_start;
	
	//println!("Input is in ring {}, which runs from {} to {}, and is {}x{}. Input is at index {}", ring, ring_start, ring_end, ring_side, ring_side, index_in_ring);

	let half_side = ring_side / 2;
	let mut coord = (half_side, half_side-1);

	let mut direction = (0, -1);

	for _i in 0..index_in_ring
	{
		coord.0 += direction.0;
		coord.1 += direction.1;
		if direction == (0, -1) && coord.1 == -1*half_side
		{
			direction = (-1, 0);
		}
		if direction == (-1, 0) && coord.0 == -1*half_side
		{
			direction = (0, 1);
		}
		if direction == (0, 1) && coord.1 == half_side
		{
			direction = (1, 0);
		}
	}

	return coord;
}

fn add_up_value(h:&HashMap<(i32,i32), i32>, coord:(i32,i32)) -> i32
{
	let neighbors: [(i32, i32); 8] = 
	[
		(-1,-1), (0,-1), (1, -1),
		(-1, 0),         (1, 0),
		(-1, 1), (0, 1), (1, 1)
	];

	let mut value = 0;
	
	for n in neighbors.iter()
	{
		let c = ( coord.0+n.0, coord.1+n.1 );
		value += match h.get(&c)
		{
			Some(v) => *v,
			None => 0
		}
	}

	return value;
}

fn day3()
{
	let input = 312051;
	//let input = 23;
	let coord = coord_for_input(input);

	println!("Input is at coord {:?} which is {} steps", coord, coord.0.abs() + coord.1.abs());

	let mut h = HashMap::new();

	// Populate the middle
	h.insert( (0,0), 1);
	let mut i = 2;
	loop
	{
		let coord = coord_for_input(i);

		let value = add_up_value(&h, coord);
		
		h.insert(coord, value);

		if value > input
		{
			println!("First value greater than {} was {} at index {}", input, value, i);
			break;
		}

		i += 1;
	}
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
	//day1b();
	//day2();
	day3();

	println!("Elapsed: {} ms", as_msecs(now.elapsed()));
}
