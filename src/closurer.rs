pub fn closurer(provider: Box<dyn Provider>) -> Box<dyn Fn() -> i32>{
    Box::new(move || provider.get())
}