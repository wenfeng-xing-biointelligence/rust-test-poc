use crate::provider::Provider;

pub struct Receiver{
    pub provider: Box<dyn Provider>,
}

impl Receiver{
    pub fn new(provider: Box<dyn Provider>) -> Receiver{
        Receiver{
            provider
        }
    }

    pub fn get(&self) -> i32{
        self.provider.get()
    }
}