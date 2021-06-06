use std::fs::{File, create_dir_all};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Sheet {
    pub id: u32,
    pub questions: Vec<String>,
    pub answers: Vec<String>
}

impl Sheet {
    pub fn new(id: u32) -> Sheet {
        Sheet {
            id,
            questions : Vec::new(),
            answers: Vec::new()
        }
    }

    pub fn get_header(&self, is_question_sheet: bool) -> String {
        let mut header = format!("-NOOH'S MATH GENERATOR-\nSHEET ID {}\n", self.id);
        
        if is_question_sheet {
            header.push_str("NAME: __________\nDATE: __________\n\n");
        } else {
            header.push('\n');
        }
        
        header
    }

    fn get_footer(&self) -> String {
        format!("\n\nScore: ___/{}", self.questions.len())
    }
    
    pub fn save_to_file(&self) -> std::io::Result<()>{
        //Create directory
        create_dir_all(format!("sheets/{}", self.id))?;
        
        //Define file paths
        let file_path_questions = format!("sheets/{}/questions.txt", self.id);
        let file_path_answers = format!("sheets/{}/answers.txt", self.id);

        //Format arrays to strings
        let mut questions_str = String::new();
        questions_str.push_str(&self.get_header(true));
        questions_str.push_str(&self.questions.join("\n"));
        questions_str.push_str(&self.get_footer());

        let mut answers_str = String::new();
        answers_str.push_str(&self.get_header(false));
        answers_str.push_str(&self.answers.join("\n"));

        // generate file with questions

        let mut questions_file = File::create(file_path_questions)?;
        questions_file.write_all(&questions_str.as_bytes())?;

        // generate file with answers

        let mut answers_file = File::create(file_path_answers)?;
        answers_file.write_all(&answers_str.as_bytes())?;

        Ok(())
    }
}
