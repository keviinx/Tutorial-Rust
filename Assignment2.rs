std::io

fn main()
{
	let mut letter = String::new();
	let mut operator  = String::new();
	let mut number1;
	let mut number2;
	let mut result;
	let mut percentage;
	let mut grade = String::new();

	println!("Question 1: Proof if Alphabet is vowel or not);
	println!("Please enter the letter now!");
	io::stdin().read_line(&mut letter).expect("Failed");
	letter = letter.trim().parse().expect("Failed trimming");
	
	if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
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
	number1= number1= .trim().parse().expect("Failed trimming");
	number2= = number2= .trim().parse().expect("Failed trimming");
	println!("Please insert the required calculation: +,-,*,/");
	io::stdin().read_line(&mut operator).expect("Failed");
	operator = operator.trim().parse().expect("Failed trimming");

	if(operator == '+')
	{
		result = number1 + number 2;
	}
	else if(operator == '-') 
	{
		result = number1 - number 2;
	}
	else if(operator == '*') 
	{
		result = number1 * number 2;
	}
	else if (operator == '/') 
	{
		result = number1 / number 2;
	}
	else
	{
		println!("wrong operator");
	}
	println!("Result: {}", result);
	println!("Question 3: Quote of student");
	println!("Question 3: Please give Percentage of Student to Return the Grade");
	io::stdin().read_line(&mut percentage).expect("Failed");
	percentage = percentage.trim().parse().expect("Failed trimming");
	
	if(percentage > 90 )
	{
		grade = 'A';
	}
	else if(percentage > 80 )
	{
		grade = 'B';
	}
	else if(percentage > 70 )
	{
		grade = 'C';
	}
	else
	{
		grade = "FAILED";
	}
	println!("The student:{}",grade);
}