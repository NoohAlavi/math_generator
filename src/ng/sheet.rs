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
    
    pub fn save_to_file(&self) -> std::io::Result<()>{
        //Create directory
        create_dir_all(format!("sheets/{}", self.id))?;

        
        //Define file paths
        let file_path_questions = format!("sheets/{}/questions.txt", self.id);
        let file_path_answers = format!("sheets/{}/answers.txt", self.id);

        //Format arrays to strings
        let questions_str = self.questions.join("\n");
        let answers_str = self.answers.join("\n");

        // TODO generate file with questions

        let mut questions_file = File::create(file_path_questions)?;
        questions_file.write_all(&questions_str.as_bytes())?;

        // TODO generate file with answers

        let mut answers_file = File::create(file_path_answers)?;
        answers_file.write_all(&answers_str.as_bytes())?;

        Ok(())
    }
}
