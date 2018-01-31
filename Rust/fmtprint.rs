
fn main() {
	// {} will be replaced by the stringified argument
	println!("{} days", 31);

	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	// Having a : before adds a space
	println!("{} out of {:2} people know about binary. The others doesn't", 1, 2);

	// Adding spaces before a number. This adds 5 spaces before 1
	println!("{number:>width$}", number=1, width=6);

	// Adding padding with other numbers. This will print 000001 (five zeroes and a 1)
	println!("{number:>0width$}", number=1, width=6);

	// Creating a structure which consists i32 and naming it Structure
	#[allow(dead_code)]
	struct Structure(i32);

	// Simply trying to print structure using std:fmt doesn't work
	// println!('This does not work: {}', Structure(i32));

}
