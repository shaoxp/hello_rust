use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;
use std::error::Error;

fn main()->Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_name_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_name_from_file2() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_name_from_file3() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_name_from_file4() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt")
}