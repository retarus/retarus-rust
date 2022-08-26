use serde::{Serialize, Deserialize};
use base64;

/// A Document represents an attachment in a fax(pdf, txt usw.)
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct Document{
    /// The name of the document
    pub name: String,
    /// The content of the scanned document
    pub data: String,
    pub charset: String,
}
impl Document {
    /// Create a new instance of the document struct, default encoding is UTF-8
    /// 
    /// 
    /// # Examples
    /// ```rust
    /// use retarus::fax::document::Document;
    /// use retarus::fax::job::Job;
    /// 
    /// let filename = "assets/testPdf.pdf";
    /// let content = std::fs::read(filename).unwrap();
    /// 
    /// let document = Document::new(filename.to_string(), content, None);
    /// ```
    pub fn new(name: String, data: Vec<u8>, charset: Option<String>) -> Document {
        let processed_data = base64::encode(data);
        let mut chars = "utf-8".to_string();
        if charset.is_some(){
            chars = charset.unwrap();
        }
        return Document{ name, data: processed_data, charset: chars };
    }


}


