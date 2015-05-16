fn main() {
	let s = "Hello, world!";
	//let r = s+"foo";
	// '+' apparently isn't the concat operator, and the concat macro only works
	// on literals. Will research further.
	println!("{}", s);
}
