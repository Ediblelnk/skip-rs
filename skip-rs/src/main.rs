use skip_rs::SkipList;

fn main() -> Result<(), String> {
    let mut skip_list = SkipList::new();

    for i in 0..2 {
        let s = format!("as");
        skip_list.insert(i * 2 + 1, s);
    }
    for i in 0..2 {
        let s = format!("as");
        skip_list.insert(i * 2, s);
    }

    println!("Skip List:\n{}", skip_list);

    skip_list.edit(1, |i| *i = i.to_uppercase())?;

    println!("Skip List:\n{}", skip_list);

    Ok(())
}
