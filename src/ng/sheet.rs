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
    
    pub fn save_to_file(&self) {
        let _file_path = "/sheets/{}.txt";
        
        // TODO generate file with questions

        // TODO generate file with answers
    }
}
