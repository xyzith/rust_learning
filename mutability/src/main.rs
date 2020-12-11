fn main() {
	let mut x = 5;
	println!("x = {}", x);
	x = 6;
	println!("x = {}", x);
/* error
	x = '9';
*/
// shadow
	let x = '8';
	println!("x = {}", x);
}
