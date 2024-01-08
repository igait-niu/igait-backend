use crate::{ 
    request::{ Request, Status },
};
use serde::{ Serialize, Deserialize};
use std::fs::{ OpenOptions, File };
use std::io::{ Seek, Write, Read };

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    entries: Vec<Request>
}
impl Database {
    pub fn from_path(path: &str) -> Self {
        // Open DB file handle
        let mut db_file_handle: File = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect(&format!("Unable to get handle on '{path}'!"));
        
        // Read contents
        let mut db_json_string = String::new();
        db_file_handle.read_to_string(&mut db_json_string)
            .expect(&format!("Unable to read '{path}'!"));

        serde_json::from_str::<Self>(&db_json_string)
            .expect("Unable to parse!")
    }

    pub fn save(&self) {
        if let Ok(mut file_handle) = OpenOptions::new()
            .read(true)
            .write(true)
            .open("data/db.json")
        {
            let self_as_string: String = serde_json::to_string_pretty(&self).unwrap();
    
            file_handle.set_len(0).unwrap();
            file_handle.rewind().unwrap();
            file_handle.write_all(self_as_string.as_bytes()).expect("Failed to write!");
        } else {
            println!("File does not exist or is busy!");
        }
    }
    pub fn new_entry(&mut self) -> usize {
        let id = (*self).entries.len();
        (*self).entries.push( 
            Request { 
                id,
                status: Status::Submitting
            }
        );

        println!("[New Entry: ID {id}]");
        
        self.save();

        id
    }
    pub fn get(&mut self, index: usize) -> Option<&mut Request> {
        (*self).entries
            .get_mut(index)
    }
}