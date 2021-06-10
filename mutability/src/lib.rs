pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a,T> LimitTracker<'a,T>
where T:Messenger{
    pub fn new(messenger:&'a T,max : usize)->LimitTracker<'a,T>{
        LimitTracker{
            messenger,
            value:0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize){
        self.value = value;
        let percentile = self.value as f32/self.max as f32;
        if percentile>=1.0{
            self.messenger.send("Error, over quota");
        }else if percentile>0.9{
            self.messenger.send("used 90%");
        }else if percentile>0.8{
            self.messenger.send("used 80%");
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messeges: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{ 
                sent_messeges:RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, messege: &str){
            self.sent_messeges.borrow_mut().push(String::from(messege));
            self.sent_messeges.borrow_mut().push(String::from(messege));

            let mut one = self.sent_messeges.borrow_mut();
            // let mut two = self.sent_messeges.borrow_mut();
            
            one.push(String::from(messege));
            drop(one);
            // two.push(String::from(messege));
            let a = self.sent_messeges.borrow().len();
            println!("{}",a);
        }
    }

    #[test]
    fn test_tracker(){
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger,100);
        tracker.set_value(85);

        assert_eq!(messenger.sent_messeges.borrow().len(),3);        
    }
}