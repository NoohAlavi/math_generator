mod ng;
use std::io;

fn main() {
    loop {
        let mut gen_booklet = String::new();
        let mut num_of_sheets = String::new();
        let mut num_of_questions = String::new();
        let mut min_num = String::new();
        let mut max_num = String::new();

        println!("Welcome to the Noohmon Generator!");
        
        println!("Do you want to generate a booklet? (y/n)");
        io::stdin()
            .read_line(&mut gen_booklet)
            .expect("Could not read line");
        
        let gen_booklet = gen_booklet
            .trim()
            .to_lowercase();

        if gen_booklet != "y" {
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

        ng::generate_sheets(num_of_sheets, num_of_questions, min_num..max_num);
    }
    println!("Thank you for using the Noohmon Generator!");
}
