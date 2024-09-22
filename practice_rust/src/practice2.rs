use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn error_handling(){
    let f= File::open("file.txt");

    let f = match f{
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("file.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Problem creating the file: {:?}", e)
                }
            }
        }
        Err(error) =>{
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

pub fn unwrap_error_handling()
{
    // let f= File::open("file.txt").unwrap();
    let f= File::open("file.txt").expect("test");
}

pub fn question_mark_error_handling() -> Result<String, io::Error>{
    let mut f= File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}