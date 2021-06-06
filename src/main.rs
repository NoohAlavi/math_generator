mod math_gen;
use std::io;

fn main() {
    loop {
        let mut gen_sheets = String::new();
        let mut num_of_sheets = String::new();
        let mut num_of_questions = String::new();
        let mut min_num = String::new();
        let mut max_num = String::new();
        let mut operators = String::new();

        println!("Welcome to Nooh's Math Generator!");
        
        println!("Do you want to generate sheets? (y/n)");
        io::stdin()
            .read_line(&mut gen_sheets)
            .expect("Could not read line");
        
        let gen_sheets = gen_sheets
            .trim()
            .to_lowercase();

        if gen_sheets != "y" {
            break
        }

        println!("Enter number of sheets to generate: ");
        io::stdin()
            .read_line(&mut num_of_sheets)
            .expect("Could not read line");

        println!("Enter number of questions per sheet: ");
        io::stdin()
            .read_line(&mut num_of_questions)
            .expect("Could not read line");

        println!("Enter minimum number: ");
        io::stdin()
            .read_line(&mut min_num)
            .expect("Could not read line");
        
        println!("Enter maximum number: ");
        io::stdin()
            .read_line(&mut max_num)
            .expect("Could not read line");

        println!("Enter operator (+, -, *, /) - If multiple seperate with spaces");
        io::stdin()
            .read_line(&mut operators)
            .expect("Could not read line");
        
        let num_of_sheets: u16 = num_of_sheets.trim()
            .parse()
            .expect("Please type a number!");

        let num_of_questions: u16 = num_of_questions
            .trim()
            .parse()
            .expect("Please type a number!");

        let min_num: u16 = min_num
            .trim()
            .parse()
            .expect("Please type a number!");

        let max_num: u16 = max_num
            .trim()
            .parse()
            .expect("Please type a number!");

        operators = String::from(operators.trim());

        math_gen::generate_sheets(num_of_sheets, num_of_questions, min_num..max_num, operators);
    }
    println!("Thank you for using Nooh's Math Generator!");
}
