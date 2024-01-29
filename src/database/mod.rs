use firebase_rs::*;
use crate::{ 
    request::{ Request, StatusCode },
};
use serde::{ Serialize, Deserialize};
use std::fs::{ OpenOptions, File };
use std::io::{ Seek, Write, Read };

#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    email: String,
    age: i8,
    ethnicity: String,
    gender: char,
    height: String,
    status: Status
}
#[derive( Serialize, Deserialize, Debug )]
pub struct Status {
    code: StatusCode,
    value: String,
}

#[derive( Debug )]
pub struct Database {
    _state: Firebase
}
impl Database {
    pub fn init () -> Self {
        Self {
            _state: Firebase::auth("https://network-technology-project-default-rtdb.firebaseio.com/", "AIzaSyDN-cTPmRowdB1sERiTDVqRzA3-sM4_T2g").unwrap()
        }
    }
    pub fn new_entry (&self, user: User) {
        todo!();
    }
    pub fn update_status (&self, status: StatusCode) {
        todo!();
    }
}