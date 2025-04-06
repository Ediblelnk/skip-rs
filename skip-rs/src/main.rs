use skip_rs::SkipList;

fn main() {
    let skip_list = SkipList::<usize>::new(4);
    // skip_list.insert(10);
    // skip_list.insert(20);
    // skip_list.insert(30);

    println!("Skip List: {}", skip_list);
}
