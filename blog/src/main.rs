use blog::Post;
fn main() {
    let mut post = Post::new();

    let content = "i am try learn rust programming to startup my company!";

    post.add_text(content);
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!(content,post.content());

    println!("Hello, world!");
}
