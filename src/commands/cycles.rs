use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn handle_cycles(input: Option<String>, directed: bool) -> io::Result<()> {
    // NOTE: setup reader
    let reader: Box<dyn BufRead> = match &input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    // NOTE: setup graph
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.trim().split(',').collect();
        if parts.len() != 2 {
            continue;
        }
        let (src, dst) = (parts[0].trim().to_string(), parts[1].trim().to_string());
        graph.entry(src.clone()).or_default().push(dst.clone());
        if !directed {
            graph.entry(dst).or_default().push(src);
        }
    }

    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    let mut has_cycle = false;

    fn dfs(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
        has_cycle: &mut bool,
        parent: Option<&str>,
        directed: bool,
    ) {
        if *has_cycle {
            return;
        }
        visited.insert(node.to_string());
        stack.insert(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for n in neighbors {
                if !visited.contains(n) {
                    dfs(n, graph, visited, stack, has_cycle, Some(node), directed);
                } else if stack.contains(n) {
                    if directed || parent.map_or(true, |p| p != n) {
                        *has_cycle = true;
                        return;
                    }
                }
            }
        }
        stack.remove(node);
    }

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs(
                node,
                &graph,
                &mut visited,
                &mut stack,
                &mut has_cycle,
                None,
                directed,
            );
            if has_cycle {
                break;
            }
        }
    }

    if has_cycle {
        println!("Cycle detected")
    } else {
        println!("No cycles detected")
    }

    Ok(())
}
