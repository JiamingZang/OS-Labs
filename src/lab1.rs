use std::sync::{Arc, Mutex};
use random::default;

pub struct Product{
    p: i32,
    is_empty:bool,
}

pub struct Producer {
    shared: Arc<Mutex<Product>>,

}

pub struct Consumer{
    shared: Arc<Mutex<Product>>,
}

impl Producer{
    pub fn produce(&self) -> Result<(), String>{
        let mut product = self.shared.try_lock().unwrap();
        if product.is_empty {
            product.p = 1;
            product.is_empty = false;
            println!("{}{}",format!("生产者已经生产："),product.p);
            Ok(())
        }else{
            println!("{}",format!("缓冲区不为空"));
            Err(format!("缓冲区不为空"))
        }

    }
}

impl Consumer{
    pub fn comsume(&self) -> Result<i32, String>{
        let mut product = self.shared.try_lock().unwrap();
        if !product.is_empty {
            let res = product.p;
            product.is_empty=true;
            println!("{}{}",format!("消费者已经消费:"),product.p);
            Ok(res)
        }else{
            Err(format!("缓冲区为空"))
        }
    }
}

impl Product {
    pub fn default() -> Self {
        Product {p:0, is_empty:true}
    }
}

pub fn initialize() ->(Producer,Consumer){
    let product = Arc::new(Mutex::new(Product::default()));
    (
        Producer {shared:product.clone()},
        Consumer {shared:product}
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread};
    #[test]
    pub fn test_it_works() {
        let (producer,consumer) = initialize();
        thread::spawn(move ||{
            producer.produce();
            producer.produce();
        }).join().unwrap();
        thread::spawn(move || {
            consumer.comsume();
        }).join().unwrap();
    }
}