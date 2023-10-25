use crate::ipc::ms::structs::{Entry, PathAggregate};
use std::{
    cmp::Reverse,
    fs,
    path::{Path, PathBuf},
};

pub struct AggData {
    dir_count: u64,
    file_count: u64,
    size: u64,
}

pub struct PathScanResult {
    name: String,
    path: PathBuf,
    // count of directly's directly nested directories
    self_dir_count: u64,
    // count of directly's directly nested files
    self_file_count: u64,
    // total size of directly nested files
    self_size: u64,
    agg_data: Option<AggData>,
    is_file: bool,
}

impl PathScanResult {
    pub fn new(path: &Path, is_file: bool) -> Self {
        Self {
            name: path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "?".to_owned()),
            path: path.to_owned(),
            self_dir_count: 0,
            self_file_count: 0,
            self_size: 0,
            agg_data: None,
            is_file,
        }
    }
}

fn get_file_size(path: &Path) -> u64 {
    let Ok(meta) = fs::metadata(path) else {
        return 0;
    };
    meta.len()
}

fn scan_path(path: &Path) -> (PathScanResult, Vec<PathScanResult>, Vec<PathBuf>) {
    let mut result = PathScanResult::new(path, false);
    let mut file_results = vec![];
    let mut nested = vec![];
    let Ok(entries) = fs::read_dir(path) else {
        return (result, file_results, vec![]);
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
            result.self_dir_count += 1;
        }
        if entry_path.is_file() {
            result.self_file_count += 1;
            let file_size = get_file_size(&entry_path);
            result.self_size += file_size;
            let mut file_scan = PathScanResult::new(&entry_path, true);
            file_scan.self_size = file_size;
            file_scan.agg_data = Some(AggData {
                dir_count: 0,
                file_count: 0,
                size: file_size,
            });
            file_results.push(file_scan);
        } else {
            // only keep if directory
            nested.push(entry_path);
        }
    }
    (result, file_results, nested)
}

pub struct Scanning {
    root_index: usize,
    // entries, tree and parents have same number of elements, and index is
    // equal to entity handle
    // todo: wrap `entries` and `tree` into one struct? (both should change at
    // same time, both should change at same time)
    entries: Vec<PathScanResult>,
    // entry_index -> indexes of nested entries
    tree: Vec<Vec<usize>>,
    // entity_handle -> parent_handle
    parents: Vec<usize>,
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
        let mut tree = vec![];
        // performing first scan step in order to prefill queue
        let (root_scan, files_scan, root_queue) = scan_path(&path);
        entries.push(root_scan);
        tree.push(vec![]);
        for file_scan in files_scan.into_iter() {
            let handle = entries.len();
            entries.push(file_scan);
            // files has no nested, but need to fill index, indexes of `entries`
            // match indexes of `tree`
            tree.push(vec![]);
            tree[0].push(handle);
        }
        for next_path in root_queue.into_iter() {
            // 0 - is index of root
            queue.push((0, next_path));
        }
        Self {
            root_index: 0,
            entries,
            tree,
            // empty because root has no known parent
            // todo: fix bug - parent indexes is same as in entries/tree
            parents: vec![],
            queue,
        }
    }

    pub fn scan_step(&mut self) {
        let Some((parent, path)) = self.queue.pop() else {
            return;
        };
        let (path_scan, files_scan, path_queue) = scan_path(&path);
        let path_index = self.entries.len();
        self.entries.push(path_scan);
        self.tree.push(vec![]);
        self.tree[parent].push(path_index);
        self.parents.push(parent);
        for file_scan in files_scan.into_iter() {
            let handle = self.entries.len();
            self.entries.push(file_scan);
            // taking index for file_scan entry
            self.tree.push(vec![]);
            self.tree[path_index].push(handle);
        }
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

    fn aggregate(&mut self) {
        struct Cursor {
            handle: usize,
            nested_i: usize,
            agg: AggData,
        }
        struct Stack(Vec<Cursor>);
        impl Stack {
            fn nested_index(&self) -> usize {
                self.0.last().unwrap().nested_i
            }

            fn handle(&self) -> usize {
                self.0.last().unwrap().handle
            }

            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            fn update_agg(&mut self, agg: &AggData) {
                let last_agg = self.0.last_mut().unwrap();
                last_agg.agg.dir_count += agg.dir_count;
                last_agg.agg.file_count += agg.file_count;
                last_agg.agg.size += agg.size;
            }

            fn inc_nested_index(&mut self) {
                self.0.last_mut().unwrap().nested_i += 1;
            }

            fn push(&mut self, cursor: Cursor) {
                self.0.push(cursor);
            }

            fn pop(&mut self) -> (usize, AggData) {
                let cursor = self.0.pop().unwrap();
                (cursor.handle, cursor.agg)
            }
        }
        let mut stack = Stack(vec![]);
        if self.entries[self.root_index].agg_data.is_none() {
            stack.push(Cursor {
                handle: self.root_index,
                nested_i: 0,
                agg: AggData {
                    dir_count: 0,
                    file_count: 0,
                    size: 0,
                },
            });
        }
        while !stack.is_empty() {
            while stack.nested_index() < self.tree[stack.handle()].len() {
                let next_nested = self.tree[stack.handle()][stack.nested_index()];
                if let Some(agg) = &self.entries[next_nested].agg_data {
                    stack.update_agg(agg);
                    stack.inc_nested_index();
                } else {
                    stack.push(Cursor {
                        handle: next_nested,
                        nested_i: 0,
                        agg: AggData {
                            dir_count: 0,
                            file_count: 0,
                            size: 0,
                        },
                    })
                }
            }
            let (handle, agg) = stack.pop();
            self.entries[handle].agg_data = Some(agg);
        }
    }

    pub fn get_aggregate_data(&mut self, up_to_fraction: f32) -> PathAggregate {
        struct IndexMapper(Vec<Option<usize>>);
        impl IndexMapper {
            fn new(size: usize) -> Self {
                Self(vec![None; size])
            }

            fn map(&mut self, index: usize, new_index: usize) {
                self.0[index] = Some(new_index);
            }

            fn index(&self, i: usize) -> usize {
                self.0[i].unwrap()
            }
        }
        self.aggregate();
        let mut index_map = IndexMapper::new(self.entries.len());
        let mut queue = vec![self.root_index];
        let mut entries = vec![];
        // tree will be storing original indexes, which will be remapped to new
        // indexes of entries
        let mut tree = vec![];
        let total = self.entries[self.root_index]
            .agg_data
            .as_ref()
            .unwrap()
            .size as f32;
        // todo: sort by size descending,
        while let Some(next_entry_index) = queue.pop() {
            let next_entry = &self.entries[next_entry_index];
            let next_agg = next_entry.agg_data.as_ref().unwrap();
            let mut tail_size = 0;
            let agg_index = entries.len();
            index_map.map(next_entry_index, agg_index);
            tree.push(vec![]);
            for next_nested_index in &self.tree[next_entry_index] {
                let next_nested_index = *next_nested_index;
                let next_nested = &self.entries[next_nested_index];
                let next_nested_agg = next_nested.agg_data.as_ref().unwrap();
                if (next_nested_agg.size as f32) / total < up_to_fraction {
                    tail_size += next_nested_agg.size;
                } else {
                    tree[next_entry_index].push(next_nested_index);
                    queue.push(next_nested_index);
                }
            }
            entries.push(Entry::new(
                next_entry_index as i64,
                next_entry.name.clone(),
                next_entry.path.to_string_lossy().as_ref().to_owned(),
                next_entry.self_size as i64,
                next_agg.size as i64,
                next_entry.self_file_count as i64,
                tail_size as i64,
                next_agg.file_count as i64,
                next_entry.self_dir_count as i64,
                next_agg.dir_count as i64,
                next_entry.is_file,
                self.parents.get(next_entry_index).map(|i| *i as i64),
            ));
        }
        // remap tree to new indexes
        let tree = tree
            .into_iter()
            .map(|ii| {
                let mut ii = ii
                    .into_iter()
                    .map(|i| index_map.index(i) as i64)
                    .collect::<Vec<_>>();
                ii.sort_by_key(|i| Reverse(entries[*i as usize].size));
                ii
            })
            .collect();
        PathAggregate::new(entries, tree)
    }
}
