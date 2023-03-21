use csv;
use scraper;
use std::{env, fs, path::Path, collections::HashMap, };
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
        .flat_map(|s| s.split_whitespace())
        .map(|s| -> String {
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

fn get_vsm_score(query_elements: &[&str], results_elements: &[&str], df_ref: &HashMap<String, i64>) -> f64 {
    let N: f64 = 100_000_000_000.0;
    let avdl = 500;

    query_elements.iter().fold(0.0, |acc, &query| {
        let tf = results_elements.iter().filter(|&&x| x == query).count() as f64;
        let df = *df_ref.get(query).unwrap() as f64;
        let idf = ((N - df + 0.5) / (df + 0.5)).log10();
        let vsm_score = idf * tf;
        acc + vsm_score
    })
}

fn get_bm25_score(query_elements: &[&str], results_elements: &[&str]) -> f64 {
    // implement BM25 algorithm used for document ranking
    // query_elements: are the query elements
    // results_elements: are the elements of the html file aka the document

    let k1 = 1.2;
    let b = 0.75;
    let mut score = 0.0;
    let avdl = results_elements.len() as f64 / results_elements.len() as f64;

    for query in query_elements {
        let mut query_score = 0.0;
        let idf = (results_elements.len() as f64
            - results_elements
                .iter()
                .filter(|&&x| x.contains(query))
                .count() as f64
            + 0.5)
            / (results_elements
                .iter()
                .filter(|&&x| x.contains(query))
                .count() as f64
                + 0.5);

        for result in results_elements {
            let tf = result.split(' ').filter(|&x| x == *query).count() as f64;
            let dl = result.split(' ').count() as f64;

            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (dl / avdl));
            let bm25_score = idf * numerator / denominator;

            query_score += bm25_score;
        }

        score += query_score;
    }

    score
}

fn process_query(query_elements: &[&str], html_filepath: &Path, idx: usize, df_ref: &HashMap<String, i64>) {
    let results_elements = get_html_text(html_filepath);
    if results_elements.is_none() {
        return;
    }
    let results_elements = results_elements.unwrap();
    let results_elements = results_elements
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    let vsm_score = get_vsm_score(query_elements, &results_elements, df_ref);
    //let bm25_score = get_bm25_score(&query_elements, &results_elements);
    println!(
        "Query num: {}, file {}",
        idx,
        html_filepath.file_name().unwrap().to_str().unwrap()
    );
    println!("vsm score: {}", vsm_score);
}

fn handle_query_dir(dir_path: &Path, df_ref: &HashMap<String, i64>) {
    println!("Handling query dir: {}", dir_path.display());
    let mut rdr = csv::Reader::from_path(&dir_path.join("rank.csv")).unwrap();
    // read query and id columns
    for (idx, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                println!("Skipping record {}: {}", idx + 1, e);
                continue;
            }
        };
        let query = record.get(1).unwrap();
        let id = record.get(6).unwrap();
        let parse_id = match id.parse::<i32>() {
            Ok(id) => id,
            Err(e) => {
                println!("Error parsing id: {}, skipping record {}", e, idx + 1);
                continue;
            }
        };
        if parse_id < 0 {
            println!("Invalid id: {}, skipping record {}", id, idx + 1);
            continue;
        }
        let html_filepath = dir_path.join(id).with_extension("html");
        let query = query.to_lowercase();
        let query_elements = query.split_whitespace().collect::<Vec<_>>();
        process_query(&query_elements, &html_filepath, idx + 1, df_ref);
    }
}

fn main() {
    let wd = env::args().nth(1).expect("No working directory given");
    let directories = vec!["query1", "query2", "query3", "query4", "query5"];

    let mut df_file = csv::Reader::from_path(&Path::new(&wd).join("df.csv")).unwrap();
    let mut df_map: HashMap<String, i64> = HashMap::new();
    
    for record in df_file.records() {
        let r = match record {
            Ok(r) => r,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        let word = r.get(1).unwrap();
        let frequency = r.get(2).unwrap();
        let frequency = frequency.parse::<i64>().unwrap();

        df_map.insert(word.to_owned(), frequency);
    }

    for dir in &directories {
        handle_query_dir(&Path::new(&wd).join(dir), &df_map);
    }
}
