fn main() {
    println!("Hello, world!");

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    num = 6;
    unsafe{
        println!("r1::{}",*r1);
        println!("r2:{}",*r2);

        *r2+=1;
        dang();
    }

    println!("num:{}",num);

    let mut v = vec![1,2,8,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);

    assert_eq!(&mut[1,2,8],a);
    assert_eq!(&mut[4,5,6],b);

    let a = my_abs(-3);
    println!("abc(-3) is {}",a);

 
}

use std::slice;

unsafe fn dang(){}
fn split_at_mut(s:&mut[i32],mid:usize)-> (&mut[i32],&mut[i32]){
    let len = s.len();
    let ptr = s.as_mut_ptr();
    assert!(mid<=len);
    unsafe{
        (
            slice::from_raw_parts_mut(ptr,mid),
            slice::from_raw_parts_mut(ptr.add(mid),len-mid)
        )
    }
    
}

extern "C" {
    fn abs(i :i32)->i32;
}

fn my_abs(i:i32)->i32{
    unsafe{
   
        fn_call_from_C();
        abs(i)
    }
}

#[no_mangle]
pub extern "C" fn fn_call_from_C(){
    println!("{}",123 );
}


