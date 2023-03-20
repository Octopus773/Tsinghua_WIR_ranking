use csv;
use scraper;
use std::{env, fs, path::Path};
use unidecode;

fn get_html_text(html_filepath: &Path) -> Option<Vec<String>> {
    let html_content = match fs::read_to_string(html_filepath) {
        Ok(html_content) => html_content,
        Err(e) => {
            println!("Error reading html file: {}", e);
            return None;
        }
    };
    let document = scraper::Html::parse_document(&html_content);
    let body = document
        .select(&scraper::Selector::parse("body").unwrap())
        .next()
        .unwrap();
    let text = body
        .text()
        .filter(|s| !s.trim().is_empty())
        .map(|s| -> String {
            let s = s.trim();
            unidecode::unidecode(s)
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect()
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    Some(text)
}

fn get_bm25_score(query_elements: &[&str], results_elements: &[&str]) -> f64 {
    let mut bm25_score = 0.0;
    for query_element in query_elements {
        for results_element in results_elements {
            if query_element == results_element {
                bm25_score += 1.0;
            }
        }
    }
    bm25_score
}

fn process_query(query: &str, html_filepath: &Path, idx: usize) {
    let query_elements = query.split_whitespace().collect::<Vec<_>>();
    let results_elements = get_html_text(html_filepath);
    if results_elements.is_none() {
        return;
    }
    let results_elements = results_elements.unwrap();
    let results_elements = results_elements
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    let bm25_score = get_bm25_score(&query_elements, &results_elements);
    println!(
        "Query num: {}, file {}",
        idx,
        html_filepath.file_name().unwrap().to_str().unwrap()
    );
    println!("BM25 score: {}", bm25_score);
}

fn handle_query_dir(dir_path: &Path) {
    println!("Handling query dir: {}", dir_path.display());
    let mut rdr = csv::Reader::from_path(&dir_path.join("rank.csv")).unwrap();
    // read query and id columns
    for (idx, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                println!("Skipping record: {}", e);
                continue;
            }
        };
        let query = record.get(1).unwrap();
        let id = record.get(6).unwrap();
        let parse_id = match id.parse::<i32>() {
            Ok(id) => id,
            Err(e) => {
                println!("Error parsing id: {}", e);
                println!("Skipping record: {:?}", record);
                continue;
            }
        };
        if parse_id < 0 {
            println!("Invalid id: {}", id);
            println!("Skipping record: {:?}", record);
            continue;
        }
        let html_filepath = dir_path.join(id).with_extension("html");
        process_query(query, &html_filepath, idx + 1);
    }
}

fn main() {
    let wd = env::args().nth(1).expect("No working directory given");
    let directories = vec!["query1", "query2", "query3", "query4", "query5"];

    for dir in directories {
        handle_query_dir(&Path::new(&wd).join(dir));
    }
}
