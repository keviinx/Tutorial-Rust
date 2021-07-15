use std::io;

fn main()
{
	let mut letter = String::new();
	let mut operator  = String::new();
	let mut number1  = String::new();
	let mut number2  = String::new();
	let result:i32;
	let mut percentage  = String::new();
	let gradeuate;

	println!("Question 1: Proof if Alphabet is vowel or not");
	println!("Please enter the letter now!");
	io::stdin().read_line(&mut letter).expect("Failed");
	letter = letter.trim().parse().expect("Failed trimming");
	
	if letter == "a" || letter == "e" || letter == "i" || letter == "o" || letter == "u"
	{
		println!("Letter is vowel");
	}
	else
	{
		println!("Letter is not vowel");
	}
	

	println!("Question2: Taschenrechner");
	println!("Please insert the numbers you want to use for calculation");	
	io::stdin().read_line(&mut number1).expect("Failed");
	io::stdin().read_line(&mut number2).expect("Failed");
	println!("Please insert the required calculation: +,-,*,/");
	io::stdin().read_line(&mut operator).expect("Failed");
	operator = operator.trim().parse().expect("Failed trimming");
	
	let number1:i32 = number1.trim().parse().expect("Failed trimming");
	let number2:i32 = number2.trim().parse().expect("Failed trimming");

	if operator == "+"
	{
		result = number1 + number2;
	}
	else if operator == "-"
	{
		result = number1 - number2;
	}
	else if operator == "*"
	{
		result = number1 * number2;
	}
	else if operator == "/"
	{
		result = number1 / number2;
	}
	else
	{
		result = 0;
		println!("wrong operator");
	}
	println!("Result: {}", result);
	println!("Question 3: Quote of student");
	println!("Question 3: Please give Percentage of Student to Return the Grade");
	io::stdin().read_line(&mut percentage).expect("Failed");
	let percentage:i32 = percentage.trim().parse().expect("Failed ");
	
	if percentage > 90 
	{
		gradeuate = "A".to_string();
	}
	else if percentage > 80 
	{
		gradeuate = "B".to_string();
	}
	else if percentage > 70 
	{
		gradeuate = "C".to_string();
	}
	else
	{
		gradeuate = "Faild".to_string();
	}
	println!("Students grade: {}", gradeuate);
}
