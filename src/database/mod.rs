use crate::{ request::{ Request, Status } };

#[derive(Debug)]
pub struct Database {
    entries: Vec<Request>
}
impl Database {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }
    pub fn from_path() -> Self {
        todo!();
    }

    pub fn new_entry(&mut self) -> usize {
        let id = (*self).entries.len();
        (*self).entries.push( 
            Request { 
                id,
                status: Status::Submitting
            }
        );
        id
    }
    pub fn get(&mut self, index: usize) -> Option<&mut Request> {
        (*self).entries
            .get_mut(index)
    }
}