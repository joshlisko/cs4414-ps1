//Josh Lisko, jpl4ke
//CS 4414 
//PS1

use std::{os, uint};

fn main() {

	let args: ~[~str] = os::args();
	let mut count = 1;
	
	loop{
		print(args[count]+" ");
		if (count > args.len()-2){
			println(" ");
			break;	
		}	
		count +=1;
	}

	
}
