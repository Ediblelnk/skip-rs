use skip_rs::SkipList;

fn main() {
    let mut skip_list = SkipList::<usize>::new(5);
    for i in 0..50 {
        skip_list.insert(i);
    }

    println!("Skip List:\n{}", skip_list);
}
