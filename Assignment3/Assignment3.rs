use std::io;

fn main()
{
    let mut n = 1;
    let mut choice = String::new();
    println!("Give all even numbers until 100");
    while n <= 100
    {
        if n%2 == 0
        {
            println!("{}",n);
        }
        n+=1;
    }
    println!("Quest No2: Please choose answer to the following question");
    println!("How old am I? \n Am I: \n 22 \n 28 \n 40");
    
    loop
    {
        io::stdin().read_line(&mut choice).expect("Failed");  
        choice = choice.trim().parse().expect("Failed");  
        if choice == "28"
        {
            println!("Congrats");
            break;
        }
        else
        {
            println!("Failed");
        }
        choice = "".to_string();

    }
    println!("Quest No3: Please give number to count digits");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed");  
    let mut number:i32 = number.trim().parse().expect("FAiled");

    let mut digit = 1;

    while number >= 10 
    {
            number = number/10;
            digit +=1;
    }
    println!("Number of digits: {}",digit);
    
}