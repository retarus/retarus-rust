
use std::{path::PathBuf, io::{Read}, fs::File};
use reqwest::{multipart};


pub struct WebexpressJob {
    pub j_username: Option<String>,
    pub j_password: Option<String>,
    dlu_listname: String,
    dlu_listcomment: String,
    dlu_type: String,
    dlu_file: String,
    dlu_charset: String,
    dlu_visibility: Option<String>,
    dlu_defaultcountrycode: String,
    dlu_firstrowcolumnnames: String
}
impl WebexpressJob {
    pub fn builder() -> WebexpressJobBuilder {
        WebexpressJobBuilder::default()
    }
    
    /// Create a minimal job with all the needed information to send to the server.
    pub fn minimal(listname: String, listcomment: String, list_type: String, file_path: String, charset: String, default_country_code: String) -> WebexpressJob {
         WebexpressJob {
            j_username: None,
            j_password: None,
            dlu_listname: listname,
            dlu_listcomment: listcomment,
            dlu_type: list_type,
            dlu_file: file_path,
            dlu_charset: charset,
            dlu_visibility: None,
            dlu_defaultcountrycode: default_country_code,
            dlu_firstrowcolumnnames: "on".to_string()
        }
    }
}
impl From<WebexpressJob> for multipart::Form {
    fn from(item: WebexpressJob) -> multipart::Form {
        let file_path = PathBuf::from(&item.dlu_file);
        let mut file = File::open(&file_path).unwrap();


        let mut buffer = vec![0; file_path.metadata().unwrap().len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        println!("File length: {:?}", buffer.len());
        let filename = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let file_part = multipart::Part::bytes(buffer).file_name(filename).mime_str("text/csv").unwrap();
        
        reqwest::multipart::Form::new()
        .text("j_username", item.j_username.unwrap())
        .text("j_password", item.j_password.unwrap())
        .text("dlu_listcomment", item.dlu_listcomment)
        .part("dlu_file", file_part)
        .text("dlu_listname", item.dlu_listname)
        .text("dlu_type", item.dlu_type)
        .text("dlu_charset", item.dlu_charset)
        .text("dlu_visibility", "company".to_string())
        .text("dlu_defaultcountrycode", item.dlu_defaultcountrycode)
        .text("dlu_firstrowcolumnnames", item.dlu_firstrowcolumnnames)
    }
} 

pub struct WebexpressJobBuilder {
    j_username: Option<String>,
    j_password: Option<String>,
    dlu_listname: String,
    dlu_listcomment: String,
    dlu_type: String,
    dlu_file: String,
    dlu_charset: String,
    dlu_visibility: Option<String>,
    dlu_defaultcountrycode: String,
    dlu_firstrowcolumnnames: String
}
impl WebexpressJobBuilder {
    pub fn default() -> WebexpressJobBuilder {
        WebexpressJobBuilder{
            j_username: None,
            j_password: None,
            dlu_listname: "".to_string(),
            dlu_listcomment: "".to_string(),
            dlu_type: "".to_string(),
            dlu_file: "".to_string(),
            dlu_charset: "".to_string(),
            dlu_visibility: None,
            dlu_defaultcountrycode: "".to_string(),
            dlu_firstrowcolumnnames: "".to_string(),
        }
    }
}