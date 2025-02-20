mod provider;
mod receiver;
mod closurer;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    use crate::closurer::closurer;
    use crate::provider::{Provider, Provider_1, Provider_2};
    use crate::receiver::Receiver;

    #[test]
    fn test(){
        let provider_1 = Provider_1;
        let provider_2 = Provider_2;

        let receiver_1 = Receiver::new(Box::new(provider_1));
        let receiver_2 = Receiver::new(Box::new(provider_2));

        assert_eq!(receiver_1.get(), 100);
        assert_eq!(receiver_2.get(), 200);
    }

    fn test_closurer(){
        let provider_1 = Provider_1;
        let provider_2 = Provider_2;

        let closurer_1 = closurer(Box::new(provider_1));
        let closurer_2 = closurer(Box::new(provider_2));

        assert_eq!(closurer_1(), 100);
        assert_eq!(closurer_2(), 200);
    }

    fn test_adder(){
        let provider_1 = Provider_1;
        let provider_2 = Provider_2;

        let adder_1 = adder(Box::new(provider_1));
        let adder_2 = adder(Box::new(provider_2));

        assert_eq!(adder_1(10), 110);
        assert_eq!(adder_2(10), 210);
    }
}
