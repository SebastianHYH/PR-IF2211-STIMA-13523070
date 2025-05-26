use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};

const INF: i32 = 1_000_000_000;

fn read_graph_from_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut graph = Vec::with_capacity(n);

    for _ in 0..n {
        let row = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        graph.push(row);
    }

    Ok(graph)
}

fn tsp(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let size = 1 << n;
    let mut dp = vec![vec![INF; n]; size];
    dp[1][0] = 0;

    for mask in 1..size {
        for u in 0..n {
            if mask & (1 << u) == 0 {
                continue;
            }
            for v in 0..n {
                if mask & (1 << v) != 0 || u == v {
                    continue;
                }
                let next_mask = mask | (1 << v);
                dp[next_mask][v] = min(dp[next_mask][v], dp[mask][u] + graph[u][v]);
            }
        }
    }

    let mut result = INF;
    let all_visited = (1 << n) - 1;
    for u in 1..n {
        if dp[all_visited][u] < INF && graph[u][0] < INF {
            result = min(result, dp[all_visited][u] + graph[u][0]);
        }
    }
    
    if result == INF {
        return -1;
    }
    
    result
}

fn main() {
    println!("Masukkan file input (e.g. test/test1.txt):");

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    let filename = filename.trim();
    match read_graph_from_file(&filename) {
        Ok(graph) => {
            println!("Adjacency Matrix:");
            for row in &graph {
                println!("{:?}", row);
            }

            let cost = tsp(graph);
            if cost == -1 {
                println!("Gagal mencari rute yang valid");
            } else {
                println!("Minimum tour cost: {}", cost);
            }
        }
        Err(e) => eprintln!("Failed to read graph: {}", e),
    }
}
