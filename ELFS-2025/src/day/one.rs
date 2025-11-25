pub fn solve() {
    
}

fn import() -> Vec<Vec<char>> {
    let doc = File::open("inputs/six").expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| l.expect("where char").chars().collect::<Vec<_>>())
        .collect()
}
