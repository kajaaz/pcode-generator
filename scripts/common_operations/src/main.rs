use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let paths = vec![
        "/home/kgorna/Documents/tools/pcode-generator/scripts/callother_count/outputs/detailed_output_calculus.txt",
        "/home/kgorna/Documents/tools/pcode-generator/scripts/callother_count/outputs/detailed_output_geth.txt",
        "/home/kgorna/Documents/tools/pcode-generator/scripts/callother_count/outputs/detailed_output_ptr.txt",
    ];

    let mut common_operations: Option<HashSet<String>> = None;

    for path in paths {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let operations = reader.lines()
            .filter_map(Result::ok)
            .filter(|line| line.starts_with("Index:"))
            .collect::<HashSet<_>>();

        common_operations = match common_operations {
            Some(common) => {
                let intersection = common.intersection(&operations).cloned().collect();
                Some(intersection)
            },
            None => Some(operations),
        };
    }

    let mut operations_indexed: HashMap<u32, String> = HashMap::new();

    if let Some(operations) = common_operations {
        for op in operations {
            if let Some((index, name)) = parse_line(&op) {
                operations_indexed.insert(index, name);
            }
        }
    }

    let mut output_file = File::create("output_common.txt")?;

    if !operations_indexed.is_empty() {
        writeln!(output_file, "Common CALLOTHER operations ordered by index:")?;
        let mut keys: Vec<&u32> = operations_indexed.keys().collect();
        keys.sort_unstable();
        for key in keys {
            if let Some(name) = operations_indexed.get(key) {
                writeln!(output_file, "Index: 0x{:x}, Name: {}", key, name)?;
            }
        }
    } else {
        writeln!(output_file, "No common operations found.")?;
    }

    Ok(())
}

fn parse_line(line: &str) -> Option<(u32, String)> {
    line.split_once(',')
        .and_then(|(index_part, name_part)| {
            let index_str = index_part.trim().strip_prefix("Index: 0x").unwrap_or("");
            let index = u32::from_str_radix(index_str, 16).ok()?;
            let name = name_part.trim().strip_prefix("Name: ").map(|n| n.to_string());
            name.map(|n| (index, n))
        })
}
