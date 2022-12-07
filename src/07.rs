mod utils;

use std::collections::HashMap;

fn main() {
    let input = utils::read_input(7);
    let files = files(&input);
    let folders = folders(&files);

    println!("{}", part1(&folders));
    println!("{}", part2(&folders));
}

type Path<'a> = Vec<&'a str>;

fn files(output: &str) -> Vec<(Path, u32)> {
    let mut files = Vec::new();
    let mut current_dir = Vec::new();
    for line in output.lines().skip(1) {
        let parts: Vec<_> = line.split(' ').collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", dir] => current_dir.push(dir),
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, file] => {
                let mut path = current_dir.clone();
                path.push(file);
                files.push((path, size.parse().unwrap()))
            }
            _ => panic!(),
        }
    }
    files
}

fn folders<'a>(files: &Vec<(Path<'a>, u32)>) -> HashMap<Path<'a>, u32> {
    let mut folders = HashMap::new();
    for (file, size) in files {
        for end in 0..file.len() {
            *folders.entry(file[..end].to_vec()).or_default() += size;
        }
    }
    folders
}

fn part1(folders: &HashMap<Path, u32>) -> u32 {
    folders.values().filter(|&&size| size <= 100000).sum()
}

fn part2(folders: &HashMap<Path, u32>) -> u32 {
    let needed = 30000000 - (70000000 - folders[&vec![]]);
    *folders
        .values()
        .filter(|&&size| size >= needed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUTPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn example() {
        let files = files(OUTPUT);
        let folders = folders(&files);

        assert_eq!(folders.len(), 4);
        assert_eq!(folders[&vec!["a", "e"]], 584);
        assert_eq!(folders[&vec!["a"]], 94853);
        assert_eq!(folders[&vec!["d"]], 24933642);
        assert_eq!(folders[&vec![]], 48381165);

        assert_eq!(part1(&folders), 95437);
        assert_eq!(part2(&folders), 24933642);
    }
}
