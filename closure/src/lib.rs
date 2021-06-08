
pub mod counter;

pub struct Cacher<T>
where T: Fn(u32)->u32{
    calc : T,
    value: Option<u32>
}

impl<T> Cacher<T>
where T: Fn(u32)->u32
{
    pub fn new(calc:T)-> Cacher<T>{
        Cacher{
            calc,
            value:None
        }
    }

    pub fn value(&mut self, arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn call_with_diffreent(){
        let mut c = Cacher::new(|c|c);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(2,v2);
    }

    #[test]
    fn iterate_test(){
        let v1 = vec![2,3,5,7];
        let mut v1_it = v1.iter();
        assert_eq!(v1_it.next(),Some(&2));
        assert_eq!(v1_it.next(),Some(&3));
        assert_eq!(v1_it.next(),Some(&5));
        assert_eq!(v1_it.next(),Some(&7));
        assert_eq!(v1_it.next(),None);
    }
}