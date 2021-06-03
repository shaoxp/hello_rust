pub mod guess{
    pub struct Guess{
        value : i32,
    }

    impl Guess{
        pub fn new(v : i32) -> Guess{
            if v< 1 || v>100{
                panic!("Guess value must be 1,100", );
            }

            Guess{value:v}
        }

        pub fn value(&self) ->i32{
            self.value
        }
    }
}