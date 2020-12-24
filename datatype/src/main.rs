fn main() {
// Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.
	let emoji = '😎';
	println!("emoji char test {}", emoji);

	//tuple
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (_x, y, _z) = tup;
	println!("y = {}", y);
	println!("y = {}", tup.1);

	let a: [u8; 3] = [1,2,3];
	let b = [3; 5];
	println!("a = {}", a[1]);
	println!("a = {}", b[1]);

}
