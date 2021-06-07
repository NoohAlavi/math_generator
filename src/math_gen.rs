mod sheet;

use eval::eval;
use rand::{Rng, prelude::SliceRandom};
use std::ops::Range;
use std::time::SystemTime;

fn generate_unique_id() -> u64 {
    // ? Generates Random ID based on system time
    // TODO make it use a better system, as the numbers can eventually get too big, and sometimes duplicate ids can be sent, causing less sheets to be generated
    let mut secs_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();
    secs_since_epoch /= 200;
    secs_since_epoch as u64
}

pub fn generate_sheets(num_of_sheets: u16, num_of_questions: u16, question_range: Range<u16>, operators: String) {
    for i in 0..num_of_sheets {
        // Create a new sheet with a randomly generated unique id
        let mut new_sheet = sheet::Sheet::new(generate_unique_id());

        for j in 1..=num_of_questions {
            // Generate two random numbers for the question
            let num1 = rand::thread_rng().gen_range(question_range.clone());
            let num2 = rand::thread_rng().gen_range(question_range.clone());

            // Pick a random operator, if multiple are present
            let operator: &str;
            let operators_ls: Vec<&str> = operators.split(' ').collect();

            operator = operators_ls.choose(&mut rand::thread_rng()).unwrap();

            // Format the numbers in the form of an equation and get the answer
            let mut new_question = format!("{} {} {}", num1, operator, num2);
            let mut answer = eval(&new_question)
                .unwrap()
                .to_string();
            answer = format!("{}) {}", j, answer);

            // Format the equation by adding the question number and an equal sign
            new_question = format!("{}) {} = ", j, new_question);
            
            new_sheet.questions.push(new_question);
            new_sheet.answers.push(answer);
        }
        // Once all the questions have been generated, save the questions and answers to a text file
        match new_sheet.save_to_file() {
            Err(_) => {
                println!("[ERROR] Could not make directory or save file");
            },
            Ok(_) => {
                println!("[SUCCESS] Sheet ID {} questions and answers saved succesfully, {} sheets remaining.", new_sheet.id, num_of_sheets - i - 1);
            }
        }
    }
}
