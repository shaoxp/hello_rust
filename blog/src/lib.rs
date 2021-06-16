pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post{
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(k) = self.state.take(){
            self.state = Some(k.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(k) = self.state.take(){
            self.state = Some(k.approve());
        }
    }

    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }
}

pub trait State{
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a> (&self,_post: &'a Post)->&'a str{
        ""
    }

    type A;
    type B;
}

pub struct Draft {
    
}

pub struct PendingReview {
}

pub struct Published {

}

impl State for Draft{
    type A = i32;
    type B = u32;
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
} 

impl State for PendingReview{
    type A = i32;
    type B = u32;

    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

impl State for Published{
    type A = i32;
    type B = u32;
    
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn content<'a> (&self,post: &'a Post)->&'a str{
        &post.content
    }
}