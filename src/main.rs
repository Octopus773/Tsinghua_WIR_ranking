use std::fs;


fn handle_query_dir(dir_path: &str) {
    println!("Handling query dir: {}", dir_path);
    let rank_file = fs::read(format!("{}/rank.csv", dir_path)).unwrap();
    let files = fs::read_dir(dir_path).unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        println!("File path: {}", file_path.display());
    }


}

fn main() {
    let directories = vec!["query1", "query2", "query3", "query4", "query5"];
    println!("Hello, world!");
}
