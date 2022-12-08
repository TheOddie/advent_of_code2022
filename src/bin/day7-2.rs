use std::collections::HashMap;

fn main()
{
    let input: Vec<Vec<&str>> = include_str!("../input/day7.txt").lines().map(|x| x.split_whitespace().collect::<Vec<&str>>()).collect();

    println!("{:?}", calculate_file_system(input));
}

fn calculate_file_system(commands: Vec<Vec<&str>>) -> (String, u64)
{
    let mut file_system: HashMap<String, u64> = HashMap::new(); // maps absolute path of file to its size
    let mut directory_map: HashMap<String, u64> = HashMap::new();
    let mut current_working_directory_path: Vec<String> = Vec::new();

    for line in commands
    {
        if line[0] == "$"
        {
            if line[1] == "cd"
            {
                match line[2] {
                    "/" => {
                        // println!("Going to root!");
                        current_working_directory_path.clear();
                    }
                    ".." => {
                        // println!("Going back!");
                        current_working_directory_path.pop();
                    }
                    dir => {
                        // println!("Going into {}", dir);
                        current_working_directory_path.push(dir.to_string());
                        file_system.insert(vec_path_to_string(&current_working_directory_path), 0);
                    }
                }
            }
        }
        else
        {
            if line[0] == "dir"
            {
                // println!("Dir");
                let mut new_dir = current_working_directory_path.clone();
                new_dir.push(line[1].to_string());
                file_system.insert(vec_path_to_string(&new_dir), 0);
            }
            else
            {
                // println!("File with size {}", line[0]);
                let mut new_file = current_working_directory_path.clone();
                new_file.push(line[1].to_string());
                let a = file_system.insert(vec_path_to_string(&new_file), line[0].parse::<u64>().unwrap());

                match a {
                    None => {
                        let absolute_path = vec_path_to_string(&current_working_directory_path);
                        let current_working_directory_size = *directory_map.get(absolute_path.as_str()).unwrap_or(&0);
                        directory_map.insert(absolute_path, current_working_directory_size + line[0].parse::<u64>().unwrap());
                    }
                    Some(x) => {
                        println!("File with size {} is already recorded!", x);
                    }
                }
            }
        }
        // println!("{}", vec_path_to_string(&current_working_directory_path));
    }

    // println!("{:?}", file_system);

    let mut propagated_directory_size_map: HashMap<String, u64> = HashMap::new();
    propagated_directory_size_map.insert("/".to_string(), *directory_map.get("/").unwrap());

    for entry in &directory_map {
        if entry.0 == "/" { continue; } // don't double count the size of the root directory
        let mut absolute_path_as_vec = string_path_to_vec(&entry.0);
        loop {
            let current_path = vec_path_to_string(&absolute_path_as_vec);
            let current_size = *propagated_directory_size_map.get(current_path.as_str()).unwrap_or(&0);
            propagated_directory_size_map.insert(current_path, current_size + entry.1);
            if absolute_path_as_vec.pop().is_none() { break; }
        }
    }

    let mut sorted_directory_list = propagated_directory_size_map.iter()
        .map(|entry| (entry.0.clone(), *entry.1)).collect::<Vec<(String, u64)>>();
    sorted_directory_list.sort_by(|(_, size_a), (_, size_b)| size_a.cmp(size_b));

    // for directory in &directory_map {
    //     println!("{:?}", directory);
    // }

    let size_of_root = sorted_directory_list.last().unwrap().1;
    let update_size = 30_000_000;
    let filesystem_size = 70_000_000;
    let target_to_remove = size_of_root - (filesystem_size - update_size);
    // println!("{} {}", size_of_root, target_to_remove);

    for directory in &sorted_directory_list {
        if directory.1 >= target_to_remove {
            return directory.clone();
        }
    }

    unreachable!();
}

fn vec_path_to_string(path: &Vec<String>) -> String
{
    let mut result = String::from("/");
    for item in path {
        result.push_str(item.as_str());
        result.push_str("/");
    }
    if result.len() > 1 {result.pop();}
    result
}

fn string_path_to_vec(path: &String) -> Vec<String>
{
    path.split("/").map(|item| item.to_string()).skip(1).collect()
}
