pub fn immutable_variables() {
	println!("variables are immutable by default");
	let x = 5;
	println!("The value of x is: {x}");
	// x = 6; // This will not compile
	println!("The value of x is: {x}");
}

pub fn mutable_variables() {
	println!("variables can be mutated with mut keyword in the declaration");
	let mut x = 5;
	println!("The value of x is: {x}");
	x = 6; // This will compile
	println!("The value of x is: {x}");
	// through mutation the type of the variable cannot be changed
	// x = "hello"; // This will not compile
	println!("The value of x is: {x}");
}

pub fn constants() {
	println!("constants are always immutable");
	// const values must be annotated with a type
	// const values must be set to a constant expression
	// constant expressions are evaluated at compile time at best
	// TODO: Need to know more about constant expressions
	const HOURS_IN_A_YEAR: u32 = 24 * 365;
	println!("No of hours in a year is {HOURS_IN_A_YEAR}");
}

pub fn shadowing() {
	println!("Shadowing is different from mutability");
	let x = 5;
	println!("The value of x is: {x}");
	// here x is a completely new variable
	// the life of previously declared x ends here and
	// the life of new x starts here
	let x = x + 1;
	{
		// in nested scope, shadowing works like other languages
		let x = x * 2;
		println!("The value of x in the inner scope is: {x}");
	}
	println!("The value of x is: {x}");
}