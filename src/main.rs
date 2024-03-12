use rb_tree::rbtree_mod::*;

fn main() {
    let mut t = RedBlackTree::new();
    t.add(1);
    println!("{:#?}", t);
}
