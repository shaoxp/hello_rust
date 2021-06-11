use blog2::Post;
fn main() {
    let mut post = Post::new();

    let content = "i am try learn rust programming to startup my company!";

    post.add_text(content);


    let post = post.request_review();
  

    let post = post.approve();

    assert_eq!(content,post.content());

    println!("Hello, world!");
}

