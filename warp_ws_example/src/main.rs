
pub struct User_Instance {
    pub uiid: usize,
    pub topics: Vec<string>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>
}

pub struct Register_Request {
    uiid: usize,
}


pub struct Register_Reponse {
    url: String,
}

pub struct Event {
    topic: String,
    uiid: Option<usize>,
    message: String,
}

pub struct Topics_Request {
    topics: Vec<String>,
}




fn main() {
    println!("Hello, world!");
}
