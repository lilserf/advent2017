use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use std::time::{Duration, Instant};

#[allow(dead_code)]
fn day1()
{
	let input = get_input("day1.txt");

	// Get an iterator of the chars in this string, which can peek ahead
	let mut iter = input.chars().peekable();
	let mut sum = 0;

	// While that iterator returns something, loop
	while let Some(c) = iter.next()
	{
		// Peek at the NEXT item
		match iter.peek()
		{
			Some(n) => 
			{
				// Convert both chars to digits and ignore any error
				let n = n.to_digit(10).unwrap();
				let c = c.to_digit(10).unwrap();
				// Compare them!
				if n == c 
				{
					sum += n;
				}
			}
			None =>
			{
				// We hit the end of th string, compare to the first item
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
	// Take a different approach for part B - convert the input string to a vector of digits
	let digit_vec: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

	let mut sum = 0;

	let num_digits = digit_vec.len();
	// Loop through the full vector
	for i in 0..num_digits
	{
		// Get the two digits we care about
		let c = digit_vec[i];
		let halfway = (i + num_digits / 2) % num_digits;
		let n = digit_vec[halfway];

		// Compare, voila
		if c == n
		{
			sum += c;
		}
	}

	println!("Sum was {}", sum);
}

// Find the two items in the vector that are common divisors for each other (one way or the other)
#[allow(dead_code)]
fn find_common_divisor(nums: &Vec<u32>) -> u32
{
	// Enumerate indices and items here
	for (i, el1) in nums.iter().enumerate()
	{
		// For each one, loop the rest of the items
		for el2 in &nums[i+1..]
		{
			// Try both directions and return what works
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

	// Should never hit for valid inputs :D
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
		// Split the line on tabs, parse the numbers, assume no error, and collect into a vector
		let nums: Vec<u32> = l.split("\t").map(|w| w.parse().unwrap()).collect();

		// Scope to deal with some borrowing issue I was seeing
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

		// Part B - call the helper function
		sumb += find_common_divisor(&nums);
	}

	println!("Checksum is {}", sum);
	println!("Part b is {}", sumb);
}

// DAY THREE - I divided the spiral into rings, like so:
//
// 3 3 3 3 3 3 3
// 3 2 2 2 2 2 3
// 3 2 1 1 1 2 3
// 3 2 1 0 1 2 3
// 3 2 1 1 1 2 3
// 3 2 2 2 2 2 3
// 3 3 3 3 3 3 3
//
// The size of the rings is predictable:
// * ring 1 is 3x3 which is 8 items
// * ring 2 is 5x5 which is 16 items
// * ring 3 is 7x7 which is 24 items
// Each ring has 8 more numbers than the previous
//
// I also gave each number a coordinate, centered around the 1 in the middle (ring 0)
// growing larger in the right and down directions (UI influence creeping in there)
// So 1 is at (0,0), 2 is at (1,0), 3 is at (1,-1), etc.
//

// Given a ring index (see above), get the biggest number in that ring
fn biggest_in_ring(ring:i32) -> i32
{
	let size = ring * 8;

	if ring > 0
	{
		// Return the previous ring's biggest plus our size
		return biggest_in_ring(ring-1) + size;
	}
	else
	{
		// Ring 0 has a 1 and that's all
		return 1;
	}
}

// Given a number in the spiral, what coordinate will it be at?
fn coord_for_input(input:i32) -> (i32, i32)
{
	let mut ring = 1;
	
	// Find the ring this number falls into
	// This is kinda inefficient since biggest_in_ring is rechecking the same stuff over and over
	while biggest_in_ring(ring) < input
	{
		ring += 1;
	}

	// This ring ends at this value
	let ring_end = biggest_in_ring(ring);
	// This ring starts at this value
	let ring_start = ring_end - (ring * 8) + 1;
	// This ring has "sides" of this size
	let ring_side = ring * 2 + 1;
	// This number falls at this index in the ring
	let index_in_ring = input - ring_start;
	
	//println!("Input is in ring {}, which runs from {} to {}, and is {}x{}. Input is at index {}", ring, ring_start, ring_end, ring_side, ring_side, index_in_ring);

	// Corners of this ring are at (half_side, half_side) in each direction
	let half_side = ring_side / 2;
	// Start one up from the bottom right, which is always the smallest number in the ring
	let mut coord = (half_side, half_side-1);
	// Start traveling upwards
	let mut direction = (0, -1);

	// I couldn't figure out a math way to do this, so just walk it
	for _i in 0..index_in_ring
	{
		// Move in the current direction
		coord.0 += direction.0;
		coord.1 += direction.1;
		// If we're moving up and we hit the top right, start moving left
		if direction == (0, -1) && coord.1 == -1*half_side
		{
			direction = (-1, 0);
		}
		// If we're moving left and we hit the top left, start moving down
		if direction == (-1, 0) && coord.0 == -1*half_side
		{
			direction = (0, 1);
		}
		// If we're moving down and we hit the bottom left, start moving right
		if direction == (0, 1) && coord.1 == half_side
		{
			direction = (1, 0);
		}
	}

	// We will have stopped on the coordinate of our number
	return coord;
}

// Given a coordinate in the spiral and a HashMap of the value at each coordinate, add up
// the neighbors of this coord to find the value that should go here
fn add_up_value(h:&HashMap<(i32,i32), i32>, coord:(i32,i32)) -> i32
{
	// Offsets for all our neighbors
	let neighbors: [(i32, i32); 8] = 
	[
		(-1,-1), (0,-1), (1, -1),
		(-1, 0),         (1, 0),
		(-1, 1), (0, 1), (1, 1)
	];

	let mut value = 0;
	
	// Loop through the neighbors
	for n in neighbors.iter()
	{
		let c = ( coord.0+n.0, coord.1+n.1 );
		// Check the hashtable
		value += match h.get(&c)
		{
			// Found! Add the value
			Some(v) => *v,
			// Not found, add nuthin'
			None => 0
		}
	}

	return value;
}


#[allow(dead_code)]
fn day3()
{
	let input = 312051;
	//let input = 23;

	// Part A ends up just being a helper function for part B :D
	let coord = coord_for_input(input);

	println!("Input is at coord {:?} which is {} steps", coord, coord.0.abs() + coord.1.abs());

	let mut h = HashMap::new();

	// Populate the middle
	h.insert( (0,0), 1);
	// And then start at number 2 in the spiral
	let mut i = 2;
	loop
	{
		// Get the coordinate for this number
		let coord = coord_for_input(i);
		// Add up its neighbors
		let value = add_up_value(&h, coord);
		// Add it to the hashmap
		h.insert(coord, value);

		// Check if we exceeded the input and break;
		if value > input
		{
			println!("First value greater than {} was {} at index {}", input, value, i);
			break;
		}

		// Otherwise, go on to the next number in the spiral
		i += 1;
	}
}

fn day4()
{
	let input = get_input("day4.txt");

	// Let's just count up part A and B at the same time
	let mut count = (0,0);
	// For each line in the input
	for l in input.lines()
	{
		// Track whether this line is valid in our two rulesets
		let mut valid = (true, true);
		let mut h = HashSet::new();
		let mut anagrams = HashSet::new();
		// Split the line into words and iterate them
		let words : Vec<&str> = l.split(' ').collect();
		for w in words
		{
			// If this word is in the hashset, it's not a valid passphrase
			if h.contains(w)
			{
				valid.0 = false;
			}
			// Add it to the hashset
			h.insert(w);

			// Create a sorted vector of the letters in this word
			let mut letters:Vec<char> = w.chars().collect();
			letters.sort();

			// Check the anagrams hashset for a matching vector
			if anagrams.contains(&letters)
			{
				valid.1 = false;
			}

			// Insert a copy of this vector
			anagrams.insert(letters.clone());
		}

		// Count whether this was valid in each ruleset
		if valid.0
		{
			count.0 += 1;
		}
		if valid.1
		{
			count.1 += 1;
		}
	}

	println!("File contains {} valid passphrases.", count.0);
	println!("File contains {} valid non-anagrammed passphrases.", count.1);
}

// Helper function to read a string from an input file
fn get_input(name:&str) -> String
{
	let prefix = String::from("input/");
	let filename = prefix+name;
	let mut file = File::open(filename).unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();
	return input;
}

// Helper function to display a less stupid unit of time
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
	//day3();
	day4();

	println!("Elapsed: {} ms", as_msecs(now.elapsed()));
}
