# Tsinghua Website ranking

## Project made as Assignment 2 of Web Information Retrieval course 2023

### Description

Implementation of VSM and BM25 ranking, to rank prefetched HTML pages from popular search engines

### Usage

```bash
cargo run --release path/to/queries/folder
```
Compiled with
`rustc 1.68.0 (2c8cc3432 2023-03-06)`
older versions should work too

### Folder structure

```
[cbihan@fedora-hp Assignment3]$ ls -R
.:
ass3-data-niko.zip  Assignment3-Ranking.pptx  df.csv  query1  query2  query3  query4  query5  query_clement.zip  read_csv.py

./query1:
10.html  11.html  12.html  13.html  14.html  15.html  17.html  18.html  1.html  20.html  2.html  3.html  4.html  5.html  6.html  7.html  8.html  9.html  rank.csv  rank_result.csv

./query2:
10.html  11.html  12.html  13.html  14.html  15.html  16.html  17.html  18.html  19.html  1.html  20.html  2.html  3.html  4.html  5.html  6.html  7.html  8.html  9.html  rank.csv  rank_result.csv

./query3:
10.html  11.html  12.html  13.html  14.html  15.html  16.html  17.html  18.html  19.html  1.html  20.html  2.html  3.html  4.html  5.html  6.html  7.html  8.html  9.html  rank.csv  rank_result.csv

./query4:
10.html  11.html  12.html  13.html  14.html  15.html  16.html  17.html  18.html  19.html  1.html  20.html  2.html  3.html  4.html  5.html  6.html  7.html  8.html  9.html  rank.csv  rank_result.csv

./query5:
10.html  11.html  12.html  13.html  15.html  16.html  17.html  18.html  19.html  1.html  20.html  2.html  3.html  5.html  6.html  7.html  8.html  9.html  rank.csv  rank_result.csv
```


### Stdout

```
Handling query dir: /Tsinghua/Courses/wir/Assignment3/query1
Query num: 1, file 11.html
vsm score: 1612.3492710179569
BM25 score: 42.74202374200901
Query num: 2, file 12.html
vsm score: 831.4328854539145
BM25 score: 40.37183742598913
Query num: 3, file 13.html
vsm score: 363.29477746634404
BM25 score: 47.14411655093671
Query num: 4, file 14.html
vsm score: 89.10827050057694
BM25 score: 28.996909519241214
Query num: 5, file 15.html
vsm score: 453.70060249484754
BM25 score: 34.969642123975184
Invalid id: -1, skipping record 6
Query num: 7, file 17.html
vsm score: 829.8500169996972
BM25 score: 42.20430428624832
Query num: 8, file 18.html
vsm score: 2894.0131889568147
BM25 score: 30.757499480306763
Skipping record 9: CSV parse error: record 9 (line 9, field: 4, byte: 2458): invalid utf-8: invalid UTF-8 in field 4 near byte index 8
Query num: 10, file 20.html
vsm score: 1619.8221413615627
BM25 score: 15.9311906389579
Skipping record 11: CSV parse error: record 11 (line 11, field: 4, byte: 3071): invalid utf-8: invalid UTF-8 in field 4 near byte index 53
Skipping record 12: CSV parse error: record 12 (line 12, field: 4, byte: 3443): invalid utf-8: invalid UTF-8 in field 4 near byte index 56
Query num: 13, file 3.html
vsm score: 296.87302912761726
BM25 score: 38.42382730573747
Query num: 14, file 4.html
vsm score: 831.4328854539145
BM25 score: 40.37183742598913
Query num: 15, file 5.html
vsm score: 296.87302912761726
BM25 score: 38.42382730573747
Query num: 16, file 6.html
vsm score: 1097.8074581835658
BM25 score: 28.59517083489698
Skipping record 17: CSV parse error: record 17 (line 17, field: 4, byte: 4880): invalid utf-8: invalid UTF-8 in field 4 near byte index 51
Query num: 18, file 8.html
vsm score: 1994.8592904976115
BM25 score: 35.16786729134432
Query num: 19, file 9.html
vsm score: 776.9700145268117
BM25 score: 40.6696856898119
Query num: 20, file 10.html
vsm score: 1370.3844420220053
BM25 score: 35.26965727282964
...cropped for brevity...
```

Files rank_result.csv are generated in each query folder.