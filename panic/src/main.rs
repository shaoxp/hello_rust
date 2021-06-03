use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let v = vec![1,2,3,4];

  let a = v[2];

  let f = File::open("hello.txt");

  let f = match f{
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound=> match File::create("hello.txt"){
            Ok(fc) => fc,
            Err(e) =>  panic!("Problem of opening file with error : {:?}",e)

        }
        other => panic!("Problem of opening file with error : {:?}",other)   
      }
  };

  let f = File::open("helloworld.txt").unwrap_or_else(|error|{
      if error.kind() == ErrorKind::NotFound{
          File::create("helloworld.txt").unwrap_or_else(|e|{
            panic!("Creating faile {:?}",e );
          })
      }else{
          panic!("Open fail {:?}", error );
      }
  });

  //let f = File::open("hello3.txt").unwrap();
  let f = File::open("hello3.txt").expect("abc");
}
