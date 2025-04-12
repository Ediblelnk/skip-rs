use skip_rs::SkipList;

fn main() {
    let mut skip_list = SkipList::new();

    for i in 0..10 {
        skip_list.insert(i, i);
    }

    println!("Skip List:\n{}", skip_list);
}
