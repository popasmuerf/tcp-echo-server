use std::net::{TcpListener,TcpStream};
use std::thread ;
use std::io::{Read,Write,Error};

/*
1. Accept incoming TCP stream
2. Create a 4KB buffer
3. for(;;) read read from the TCP stream into the buffer
   and then write the contents of the buffer to a variable of bytes
   3.1 If the buffer is empty then return Result::Ok(())
   3.2 write the retrieved bytes back into a stream. and return 
       as Result<(),Error>



4. A server might need some time to process the input before
   serving back the response.  Let's simulate that by sleeping
   for a random amount of time in the handle_client function.

*/
fn handle_client(mut stream:TcpStream)-> Result<(),Error>{
    println!("Incoming connection from : {}",
             stream.peer_addr()?);
             let mut buf :[u8;512] = [0; 512];
             loop{
                 let bytes_read : usize = stream.read(&mut buf)? ;

                 if bytes_read == 0{
                     return Ok(());
                 }
                 stream.write(&buf[..bytes_read])? ;
             }
}


fn main() {
    let listener:TcpListener = TcpListener::bind("0.0.0.0:9999")
    .expect("Could not bind");

    for stream in listener.incoming(){
        match stream{
            Result::Err(e) => {eprintln!("Failed: {}",e)}
            Result::Ok(stream) =>{
                thread::spawn(move || {
                    handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}",error));

                });
            }
        }
    }

}
