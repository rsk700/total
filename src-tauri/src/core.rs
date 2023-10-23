use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct PathScanResult {
    name: String,
    path: PathBuf,
    // count of directly nested directories
    dir_count: u64,
    // count of directly nested files
    file_count: u64,
    // total size of directly nested files
    size: u64,
}

fn get_file_size(path: &Path) -> u64 {
    let Ok(meta) = fs::metadata(path) else {
        return 0;
    };
    meta.len()
}

fn scan_path(path: &Path) -> (PathScanResult, Vec<PathBuf>) {
    let mut result = PathScanResult {
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "?".to_owned()),
        path: path.to_owned(),
        dir_count: 0,
        file_count: 0,
        size: 0,
    };
    let mut nested = vec![];
    let Ok(entries) = fs::read_dir(path) else {
        return (result, vec![]);
    };
    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let entry_path = entry.path();
        if entry_path.is_symlink() {
            // ignoring symlinks
            continue;
        }
        if entry_path.is_dir() {
            result.dir_count += 1;
        }
        if entry_path.is_file() {
            result.file_count += 1;
            result.size += get_file_size(&entry_path);
        }
        nested.push(entry_path);
    }
    (result, nested)
}

pub struct Scanning {
    root: PathBuf,
    entries: Vec<PathScanResult>,
    // entry_index -> indexes of nested entries
    tree: Vec<Vec<usize>>,
    queue: Vec<(usize, PathBuf)>,
}

impl Scanning {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path: PathBuf = path.into();
        if !path.is_dir() && !path.is_absolute() {
            panic!("expecting absolute path to directory")
        }
        let mut entries = vec![];
        let mut queue = vec![];
        // performing first scan step in order to prefill queue
        let (root_scan, root_queue) = scan_path(&path);
        entries.push(root_scan);
        for next_path in root_queue.into_iter() {
            // 0 - is index of root
            queue.push((0, next_path));
        }
        Self {
            root: path,
            entries,
            // adding nested for root (at index `0`), it empty before scan
            tree: vec![vec![]],
            queue,
        }
    }

    pub fn scan_step(&mut self) {
        let Some((parent, path)) = self.queue.pop() else {
            return;
        };
        let (path_scan, path_queue) = scan_path(&path);
        let path_index = self.entries.len();
        self.entries.push(path_scan);
        self.tree.push(vec![]);
        self.tree[parent].push(path_index);
        for next_path in path_queue.into_iter() {
            self.queue.push((path_index, next_path));
        }
    }

    pub fn scan_is_finished(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn done_count(&self) -> u64 {
        self.entries.len() as u64
    }
}
