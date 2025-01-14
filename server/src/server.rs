use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;



pub struct Server{
        addr: String,
    } 
   
     impl Server { //implementing Server struct
       pub fn new(addr: String) -> Self {
            Server {
                addr
            }
        }
        pub fn run(self){
         print!("Our server is listening on {:?}", self.addr);

         let listener = TcpListener::bind(&self.addr).unwrap(); //binding to the provided address
        

          loop {
            match  listener.accept(){
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; //reading the data using buffer
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received message: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]){
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                            

                        }
                        Err(_) => println!("Failed to read message"),
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
 
            }
          }
        }
     }