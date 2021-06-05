mod sheet;

use eval::eval;
use rand::Rng;
use std::ops::Range;

fn generate_unique_id() -> u32 {
    // TODO keep a list of all currents ids in some kind of file, and then generate unique ids. 
    rand::thread_rng().gen_range(0..100000)
}

pub fn generate_sheets(num_of_sheets: u16, num_of_questions: u16, question_range: Range<u16>) {
    for i in 0..num_of_sheets {
        // Create a new sheet with a randomly generated unique id
        let mut new_sheet = sheet::Sheet::new(generate_unique_id());

        println!("Generating questions and answers for sheet ID {}, {} sheet(s) remaining", new_sheet.id, num_of_sheets - i - 1);
        for j in 1..=num_of_questions {
            // Generate two random numbers for the question
            let num1 = rand::thread_rng().gen_range(question_range.clone());
            let num2 = rand::thread_rng().gen_range(question_range.clone());
            
            // Format the numbers in the form of an equation and get the answer
            let mut new_question = format!("{} + {}", num1, num2);
            let answer = eval(&new_question)
                .unwrap()
                .to_string();
            
            // Format the equation by adding the question number and an equal sign
            new_question = format!("{}) {} = ", j, new_question);
            println!("{}", new_question);
            
            new_sheet.questions.push(new_question);
            new_sheet.answers.push(answer);
        }
        // Once all the questions have been generated, save the questions and answers to a text file
        new_sheet.save_to_file();
    }
}
