use std::io;
use std::fs::File;

pub type FileReader = io::Result<io::Lines<io::BufReader<File>>>;
// This function will take in as input the file that was read and print out the solution to stdin
pub fn solve(contents : FileReader){
    let mut sum = 0;
    if let Ok(lines) = contents {
	for line in lines{
	    if let Ok(s) = line {
		println!("{}",s);
		let mut digits = s.chars()
			.filter(|s| s.is_digit(10))
		        .map(|s| s.to_digit(10).unwrap())
		        .peekable();
		sum += 10*digits.peek().unwrap();
		sum += digits.last().unwrap();
	    } else {
		println!("Error in reading line");
	    }
	}
    } else {
	println!("Error in file");
    }
    println!("{}",sum);
}
