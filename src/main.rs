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

// Given a number in the spiral, what coordinate will it be at?
fn coord_for_input(input:i32) -> (i32, i32)
{
	// Ring num is the square root of the number - 1, / 2, rounded up
	let ring = (((input as f32).sqrt() - 1.0) / 2.0).ceil() as i32;
	// This ring has "sides" of this size
	let ring_side = ring * 2 + 1;
	// This ring starts at this value
	let ring_start = (ring_side-2) * (ring_side-2) + 1;
	// This number falls at this index in the ring
	let index_in_ring = input - ring_start;
	
	//println!("Input ring starts at {}, and is {}x{}. Input is at index {}", ring_start, ring_side, ring_side, index_in_ring);

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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn day5()
{
	let input = get_input("day5.txt");

	let mut instructions:Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();

	let mut pc:i32 = 0;
	let mut steps = 0;

	while pc >= 0 && pc < instructions.len() as i32
	{
		let newpc = pc + instructions[pc as usize];
		// Part A logic
		//instructions[pc as usize] += 1;
		// End Part A logic
		// Part B logic
		if instructions[pc as usize] >= 3
		{
			instructions[pc as usize] -= 1;
		}
		else
		{
			instructions[pc as usize] += 1;
		}
		// End Part B logic
		pc = newpc;
		steps += 1;
	}

	println!("Took {} steps", steps);
}

fn redist(banks:&mut Vec<u32>)
{
	let mut index;
	let mut value;

	// Scope to mutably borrow `banks`, since we need to mutably borrow it later too
	{
		// Unfortunately we can't just use iterator::max because it breaks ties by giving you the last tied element
		// So first find the max value
		let maxvalue = banks.iter().max().unwrap();
		// Now enumerate (to include the index), filter only those where the value matches the max, and take the first one
		let tuple = banks.iter().enumerate().filter(|&(_,x)| x == maxvalue).next().unwrap();
		index = tuple.0;
		value = *tuple.1;
	}
	
	// Zero that bank
	banks[index] = 0;

	// Loop the banks dropping 1 into each
	while value > 0
	{
		index = (index + 1) % banks.len();
		banks[index] += 1;
		value -= 1;
	}

	//println!("Vector is now {:?}", banks);
}

#[allow(dead_code)]
fn day6()
{
	let mut input:Vec<u32> = vec![0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11];
	//let mut input:Vec<u32> = vec![0, 2, 7, 0];

	let mut patterns = HashMap::new();

	let mut count = 0;
	let cycles;
	loop
	{
		redist(&mut input);
		count += 1;

		if patterns.contains_key(&input)
		{
			cycles = count - patterns[&input];
			break;
		}

		patterns.insert(input.clone(), count);
	}

	println!("Took {} cycles to reach a repeated pattern with a period of {} cycles", count, cycles);
}

// Wrapper for a usize used to denote a Node's ID
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NodeId
{
	index: usize,
}

// Tree node that takes generic data and has a parent and many children
#[derive(Clone,Debug)]
pub struct Node<T> 
{
	pub parent: Option<NodeId>,
	pub children: Vec<NodeId>,

	pub data: T,
}

// Tree class that represents the nodes as a flat vector
#[derive(Debug)]
pub struct IndexTree<T:Eq+Clone>
{
	nodes : Vec<Node<T>>,
}

// Implementation of IndexTree
impl<T:Eq+Clone> IndexTree<T>
{
	// Constructor
	fn new() -> IndexTree<T>
	{
		IndexTree { nodes:Vec::new(), }
	}

	// Make a new node with the given data and return its ID
	fn new_node(&mut self, data: T) -> NodeId
	{
		let next_index = self.nodes.len();

		self.nodes.push(Node
		{
			parent: None,
			children: Vec::new(),
			data: data,
		});

		NodeId{index: next_index}
	}

	// Get a mutable reference to a given node
	fn get_node(&mut self, id: NodeId) -> &mut Node<T>
	{
		&mut self.nodes[id.index]
	}

	// Find the ID of a node matched by the given predicate
	fn find_node<P>(&self, pred:&P) -> Option<NodeId>
	where P: Fn(&T) -> bool,
	{	
		for i in 0..self.nodes.len()
		{
			if pred(&self.nodes[i].data)
			{
				return Some(NodeId{index:i});
			}
		}

		return None;
	}
}

// Struct to go in my tree
#[derive(Eq,Clone,PartialEq,Debug)]
pub struct Program
{
	name: String,
	weight: u32,
	total_weight: Option<u32>
}

// Given a node, calculate its total weight by recursing into its children if necessary
fn calc_weight(tree:&mut IndexTree<Program>, root_id:NodeId) -> u32
{
	println!("Calculating total weight of {}",tree.get_node(root_id).data.name);
	
	// Clone the list of kids in a quick scope so that we're not borrowing the tree itself later
	let kids:Vec<NodeId> = tree.get_node(root_id).children.clone();

	// Weight of all kids
	let mut kid_weight = 0;
	// Weight of the most recent kid
	let mut last_weight = 0;
	for c in kids
	{
		// If this child doesn't already have a total weight, recurse
		if let None = tree.get_node(c).data.total_weight
		{
			let total_weight = Some(calc_weight(tree, c));
			tree.get_node(c).data.total_weight = total_weight;
		}

		// Get this child's total weight
		let this_weight = tree.get_node(c).data.total_weight.unwrap();
		// Add it to our kid_weight
		kid_weight += this_weight;
		println!("Node {}'s total weight is {}", tree.get_node(c).data.name, this_weight);

		// If this weight doesn't match the last sibling, one of the two is the busted one
		if last_weight != 0 && last_weight != this_weight
		{
			println!("Error! Node {}'s weight ({}) does not match its sibling ({})", tree.get_node(c).data.name, this_weight, last_weight);
		}

		// Remember the most recent weight we saw
		last_weight = this_weight;
	}

	// Set the total weight onto our node and also return it
	let total_weight = tree.get_node(root_id).data.weight + kid_weight;
	tree.get_node(root_id).data.total_weight = Some(total_weight);
	return total_weight;
}

#[allow(dead_code)]
fn day7()
{
	let input = get_input("day7.txt");

	let mut tree:IndexTree<Program> = IndexTree::new();

	for l in input.lines()
	{
		let split:Vec<&str> = l.split(" ").collect();
		let numlen = split[1].len();
		let num:u32 = split[1][1..numlen-1].parse().unwrap();

		let prog = Program 
		{
			name: split[0].to_string(),
			weight: num,
			total_weight: None,
		};

		println!("Processing node {}", prog.name);
		let parent_id = match tree.find_node(&|d| d.name == prog.name)
		{
			None => tree.new_node(prog.clone()),
			Some(parent_id) => parent_id,
		};
		
		// Make sure the weight is set - in case the first occurence of this node was as a child
		tree.get_node(parent_id).data = prog.clone();

		if split.len() > 2
		{
			for child in split[3..].to_vec()
			{
				let child = child.trim_matches(',');
				let prog = Program 
				{
					name: child.to_string(),
					weight: 0,
					total_weight: None
				};

				// Set the child ID by either finding an existing node or creating one
				let child_id = match tree.find_node(&|d| d.name == prog.name)
				{
					Some(child_id) => { println!("  Found existing child {} at {}", prog.name, child_id.index); child_id },
					None => { println!("  Creating new child {}", prog.name); tree.new_node(prog) },
				};

				// Get that child node and set its parent
				tree.get_node(child_id).parent = Some(parent_id);

				// Get the parent node and add this child
				if !tree.get_node(parent_id).children.contains(&child_id)
				{
					tree.get_node(parent_id).children.push(child_id);
				}
			}
		}
	}

	// Just arbitrarily start with the first node in the input file
	let mut curr_node_id = NodeId{index:0};
	let root_node_id:NodeId;
	loop
	{
		let curr_node = tree.get_node(curr_node_id);
		println!("Checking {:?} for a parent...",curr_node);
		if let Some(n) = curr_node.parent
		{
			curr_node_id = n;
		}
		else
		{
			// Hey we found the root!
			root_node_id = curr_node_id;
			break;
		}
	}

	println!("Root node is {}", tree.get_node(root_node_id).data.name);

	// Now calculate the weight and write out which 
	calc_weight(&mut tree, root_node_id);
}

// Operation to perform
enum Day8Operation
{
	Increment(String,i32),
	Decrement(String,i32),
	Invalid,
}

// Condition to check
enum Condition
{
	Greater(String,i32),
	Less(String,i32),
	GreaterEqual(String,i32),
	LessEqual(String,i32),
	Equal(String,i32),
	NotEqual(String,i32),
	Invalid,
}

#[allow(dead_code)]
fn day8()
{
	let input = get_input("day8.txt");

	let mut instructions:Vec<(Condition, Day8Operation)> = Vec::new();
	let mut registers:HashMap<String, i32> = HashMap::new();

	// Parse the input into a list of instructions
	for l in input.lines()
	{
		let split:Vec<&str> = l.split(" ").collect();

		let op = match split[1]
		{
			"inc" => Day8Operation::Increment(split[0].to_string(), split[2].parse().unwrap()),
			"dec" => Day8Operation::Decrement(split[0].to_string(), split[2].parse().unwrap()),
			_ => Day8Operation::Invalid,
		};

		let cond = match split[5]
		{
			">" => Condition::Greater(split[4].to_string(), split[6].parse().unwrap()),
			"<" => Condition::Less(split[4].to_string(), split[6].parse().unwrap()),
			">=" => Condition::GreaterEqual(split[4].to_string(), split[6].parse().unwrap()),
			"<=" => Condition::LessEqual(split[4].to_string(), split[6].parse().unwrap()),
			"==" => Condition::Equal(split[4].to_string(), split[6].parse().unwrap()),
			"!=" => Condition::NotEqual(split[4].to_string(), split[6].parse().unwrap()),
			_ => Condition::Invalid,
		};

		instructions.push( (cond, op) );
	}

	// Track the highwater for part B
	let mut highwater:i32 = 0;

	// Execute the instructions
	for instr in instructions
	{
		// First check the condition
		let result = match instr.0
		{
			Condition::Greater(name, value) => *registers.entry(name).or_insert(0) > value,
			Condition::Less(name, value) => *registers.entry(name).or_insert(0) < value,
			Condition::GreaterEqual(name, value) => *registers.entry(name).or_insert(0) >= value,
			Condition::LessEqual(name, value) => *registers.entry(name).or_insert(0) <= value,
			Condition::Equal(name, value) => *registers.entry(name).or_insert(0) == value,
			Condition::NotEqual(name, value) => *registers.entry(name).or_insert(0) != value,
			_ => false,
		};

		if result
		{
			// Perform the instruction
			match instr.1
			{
				Day8Operation::Increment(name, value) =>
				{
					*registers.entry(name).or_insert(0) += value;
				},
				Day8Operation::Decrement(name, value) => 
				{
					*registers.entry(name).or_insert(0) -= value;
				},
				_ => {},
			};

			// Check the highwater
			let max = registers.values().max().unwrap();
			if *max > highwater
			{
				highwater = *max;
			}

		}

	}

	let max = registers.values().max().unwrap();
	println!("Maximum register value after one run is {}", max);
	println!("Highwater register value was {}", highwater);
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
	//day4();
	//day5();
	//day6();
	//day7();
	day8();

	println!("Elapsed: {} ms", as_msecs(now.elapsed()));
}
