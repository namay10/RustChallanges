use state::Post;

fn main() {
    let mut post = Post::new();
    println!("{:#?}", post.content());
    post.request_review();
    println!("{:#?}", post.content());
    post.approve();
    println!("{:#?}", post.content());
}
