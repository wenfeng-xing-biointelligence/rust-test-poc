pub trait Provider{
    fn get(&self) -> i32;
}

pub struct Provider_1;

impl Provider for Provider_1{
    fn get(&self) -> i32{
        100
    }
}

pub struct Provider_2;

impl Provider for Provider_2{
    fn get(&self) -> i32{
        200
    }
}