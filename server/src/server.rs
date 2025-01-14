use std::net::TcpListener;


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
        
        }
     }