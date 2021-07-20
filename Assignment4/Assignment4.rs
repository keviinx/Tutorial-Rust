fn main()
{	
	facturial1();
	facturial2(5);
	println!("{}",facturial3(4));
	println!("{:?}",facturial4(5)); //:? for different values
	println!("{:?}",facturial5(4)); //:? for different values

	fn facturial5(a:i32)->(String,i32){
	let mut fact = 1;

	for n in 1 ..a+1	
	{
		fact = fact*n;
	}
	("Factorial of 5:".to_string(),fact)
}
}

fn facturial1()
{
  	let a = 4;
	let mut fact = 1;

	for n in 1 ..a+1	
	{
		fact = fact*n;
	}
	println!("{}",fact);
}


fn facturial2(a:i32){
	let mut fact = 1;

	for n in 1 ..a+1	
	{
		fact = fact*n;
	}
	println!("{}",fact);
}


fn facturial3(a:i32)->i32{
	let mut fact = 1;

	for n in 1 ..a+1	
	{
		fact = fact*n;
	}
	fact
}

fn facturial4(a:i32)->(String,i32){
	let mut fact = 1;

	for n in 1 ..a+1	
	{
		fact = fact*n;
	}
	("Factorial of 5:".to_string(),fact)
}