pub trait CommandAction{
    fn action(&mut self);
}

pub trait CommandInit{
    fn init(&mut self);
}