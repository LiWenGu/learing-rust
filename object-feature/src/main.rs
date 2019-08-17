use object_feature::Post;

//
fn main() {
    let mut post = Post::new();

    post.add_text("i ate a salad for lunch today");
    post.request_review();
    post.approve();
    assert_eq!("i ate a salad for lunch today", post.content());
}
