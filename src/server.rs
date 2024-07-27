use std::net::TcpListener;

pub struct Server {
    addr: String,
}

// self represents the instance of struct, The method is being called on
impl Server {
    pub fn new(addr: String) -> Self {
        //Here Self indicate the Server struct
        Self { addr } //Here Self indicate the Server struct
    }

    // here run is method. the special thing about methods is that they accept a special first parameter called self
    // self just point to the instance of the struct, like self.addr, addr is exist in struct
    // self followsthe ownership rules for normal variables, in the case here the run function takes ownership of the entire structure bcz it takes the ownership of the self variable
    // So the struct will actually be deallocated when the run function exists
    // if we don't want to de allocate our structure at the end of our function, we can make self reference like &self,
    // for &self run does not take ownership of the struct and it can also be mutable referance
    ///// for our use case we can just let run take our ownership of the struct bcz we just want to call run and then one will loop forever

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap(); //here this a recoverable error bcz the bind function will just return a result and we decide how to handle it

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("OK");
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
