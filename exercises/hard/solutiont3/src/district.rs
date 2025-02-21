use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json;

pub fn count_provinces() -> String {
    // 读取 JSON 文件
    let content = fs::read_to_string("district.json").expect("Failed to read file");

    // 解析 JSON 数据
    let data: HashMap<String, HashMap<String, Vec<String>>> =
        serde_json::from_str(&content).expect("Failed to parse JSON");

    let mut batches: Vec<_> = data.keys().cloned().collect();
    batches.sort();

    // 存储每个批次的省份数
    let mut results = Vec::new();

    // 处理每个批次
    for batch in batches {
        let batch_data = &data[&batch];

        // 构建邻接表
        let mut adj: HashMap<String, HashSet<String>> = HashMap::new();

        // 为所有城市（键和邻居）创建条目，并跳过自环
        for (city, neighbors) in batch_data {
            // 过滤掉自环
            let filtered_neighbors: Vec<_> = neighbors.iter().filter(|&n| n != city).cloned().collect();
            // 仅当城市有其他邻居或被其他城市列为邻居时，加入邻接表
            if !filtered_neighbors.is_empty() || batch_data.keys().any(|k| batch_data[k].contains(city) && k != city) {
                adj.entry(city.clone()).or_insert_with(HashSet::new);
                for neighbor in &filtered_neighbors {
                    adj.entry(neighbor.clone()).or_insert_with(HashSet::new);
                }
            }
        }

        // 添加双向边
        for (city, neighbors) in batch_data {
            let filtered_neighbors: Vec<_> = neighbors.iter().filter(|&n| n != city).cloned().collect();
            for neighbor in filtered_neighbors {
                if adj.contains_key(city) && adj.contains_key(&neighbor) {
                    adj.get_mut(city).unwrap().insert(neighbor.clone());
                    adj.get_mut(&neighbor).unwrap().insert(city.clone());
                }
            }
        }

        // 计算连通分量（省份数）
        let mut visited = HashSet::new();
        let mut components = 0;

        for city in adj.keys() {
            if !visited.contains(city) {
                components += 1;
                let mut queue = VecDeque::new();
                queue.push_back(city.clone());
                visited.insert(city.clone());

                while let Some(current) = queue.pop_front() {
                    for neighbor in adj[&current].iter() {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // 记录该批次的省份数
        results.push(components.to_string());
    }

    results.join(",")
}