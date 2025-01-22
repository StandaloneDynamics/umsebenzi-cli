use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env::var;


static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];

fn get_editor() -> Option<String> {
    for env in ENV_VARS{
        let value = match var(env){
            Ok(e) => e,
            Err(_) => continue
        };
        return Some(value)
    }
    None
}

pub fn text_editor(data: Option<String>) -> Option<String>{
    if let Some(editor) = get_editor(){
        let mut temp_path = std::env::temp_dir();
        temp_path.push("project.txt");

        let mut file = match  File::create(&temp_path){
            Ok(f) => f,
            Err(err) => {
                eprintln!("Unable to create temp file: {:?}", err);
                std::process::exit(1); 
            }
        };
        if data.is_some(){
            file.write_all(data.expect("fail getting").as_bytes()).expect("uable to write description to file");
        }
        

        let status = match Command::new(editor).arg(&temp_path).status(){
            Ok(r) => r,
            Err(err) => {
                eprintln!("Unable to edit file {:?}", err);
                std::process::exit(1); 
            }
        };
        if status.success(){
            let editable = match fs::read_to_string(&temp_path){
                Ok(contents) => contents,
                Err(err) => {
                    eprintln!("Unable to read file {:?}", err);
                    std::process::exit(1); 
                }
            };
            fs::remove_file(&temp_path).expect("Unable to delete temp file");
            return Some(editable);

        }else{
            eprintln!("Unable to save description");
        std::process::exit(1);    
        }
    }else{
        eprintln!("Unable to open editor");
        std::process::exit(1);
    }

}