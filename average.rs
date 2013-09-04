//Josh Lisko, jpl4ke
//CS 4414 
//PS1
//With help from the rust irc

use std::{os, uint, float};

fn main() {

	let args = os::args();
	let mut n = 0;
	let mut sum = 0.0;

	for uint::range(1, args.len()) |x| {
		match float::from_str(args[x]) {
			Some(num) => { 
				sum += num; 
				n+=1; 
			}
, 
			None => { 
				println(fmt!("Bad input: %s",args[x])); 
			} 
		}
	}

	println(fmt!("Average: %f", sum/(n as float) ) );

	}
