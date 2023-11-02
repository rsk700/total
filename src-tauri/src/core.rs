use murmur3::murmur3_32;

use crate::ipc::ms::structs::AggregateEntry;
use std::{
    cmp::Reverse,
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct AggData {
    // count of directly's directly nested directories
    self_dir_count: u64,
    // count of directly's directly nested files
    self_file_count: u64,
    // total size of directly nested files
    self_size: u64,
    dir_count: u64,
    file_count: u64,
    size: u64,
}

impl std::ops::Add for AggData {
    type Output = AggData;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            self_dir_count: self.self_dir_count + rhs.self_dir_count,
            self_file_count: self.self_file_count + rhs.self_file_count,
            self_size: self.self_size + rhs.self_size,
            dir_count: self.dir_count + rhs.dir_count,
            file_count: self.file_count + rhs.file_count,
            size: self.size + rhs.size,
        }
    }
}

impl std::ops::AddAssign<&Self> for AggData {
    fn add_assign(&mut self, rhs: &Self) {
        self.self_dir_count += rhs.self_dir_count;
        self.self_file_count += rhs.self_file_count;
        self.self_size += rhs.self_size;
        self.dir_count += rhs.dir_count;
        self.file_count += rhs.file_count;
        self.size += rhs.size;
    }
}

pub struct PathScanResult {
    name: String,
    path: PathBuf,
    agg_data: Option<AggData>,
    is_file: bool,
    parent: Option<usize>,
    nested: Vec<usize>,
}

impl PathScanResult {
    fn get_safe_name(path: &Path) -> String {
        path.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "?".to_owned())
    }

    pub fn new_dir(path: &Path) -> Self {
        Self {
            name: Self::get_safe_name(path),
            path: path.to_owned(),
            agg_data: None,
            is_file: false,
            parent: None,
            nested: vec![],
        }
    }

    pub fn new_file(path: &Path, file_size: u64) -> Self {
        Self {
            name: Self::get_safe_name(path),
            path: path.to_owned(),
            agg_data: Some(AggData {
                self_dir_count: 0,
                self_file_count: 1,
                self_size: file_size,
                dir_count: 0,
                file_count: 1,
                size: file_size,
            }),
            is_file: true,
            parent: None,
            nested: vec![],
        }
    }
}

fn get_file_size(path: &Path) -> u64 {
    let Ok(meta) = fs::metadata(path) else {
        return 0;
    };
    meta.len()
}

fn scan_path(path: &Path) -> (PathScanResult, Vec<PathBuf>) {
    if path.is_file() {
        (PathScanResult::new_file(path, get_file_size(path)), vec![])
    } else {
        let mut nested = vec![];
        let Ok(entries) = fs::read_dir(path) else {
            return (PathScanResult::new_dir(path), vec![]);
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
            nested.push(entry_path);
        }
        (PathScanResult::new_dir(path), nested)
    }
}

fn get_string_hash(s: &str) -> i64 {
    murmur3_32(&mut Cursor::new(s), 0).unwrap() as i64
}

pub struct Scanning {
    root_index: usize,
    // entries, tree and parents have same number of elements, and index is
    // equal to entity handle
    entries: Vec<PathScanResult>,
    // parent index -> nested path
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
            root_index: 0,
            entries,
            queue,
        }
    }

    pub fn scan_step(&mut self) {
        let Some((parent, path)) = self.queue.pop() else {
            return;
        };
        let (mut path_scan, path_queue) = scan_path(&path);
        path_scan.parent = Some(parent);
        let path_index = self.entries.len();
        self.entries.push(path_scan);
        self.entries[parent].nested.push(path_index);
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
                last_agg.agg += agg;
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
                agg: Default::default(),
            });
        }
        while !stack.is_empty() {
            while stack.nested_index() < self.entries[stack.handle()].nested.len() {
                let next_nested_i = self.entries[stack.handle()].nested[stack.nested_index()];
                let next_nested = &self.entries[next_nested_i];
                if let Some(agg) = &next_nested.agg_data {
                    if next_nested.is_file {
                        stack.update_agg(agg);
                    } else {
                        stack.update_agg(&AggData {
                            self_dir_count: 1,
                            self_file_count: 0,
                            self_size: 0,
                            dir_count: agg.dir_count + 1,
                            file_count: agg.file_count,
                            size: agg.size,
                        });
                    }
                    stack.inc_nested_index();
                } else {
                    stack.push(Cursor {
                        handle: next_nested_i,
                        nested_i: 0,
                        agg: Default::default(),
                    })
                }
            }
            let (handle, agg) = stack.pop();
            self.entries[handle].agg_data = Some(agg);
        }
    }

    pub fn get_aggregate_data(&mut self, up_to_fraction: f32) -> Vec<AggregateEntry> {
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

            fn try_index(&self, i: usize) -> Option<usize> {
                self.0.get(i).copied().flatten()
            }
        }
        self.aggregate();
        let mut index_map = IndexMapper::new(self.entries.len());
        let mut queue = vec![self.root_index];
        let mut entries = vec![];
        // tree will be storing original indexes, which will be remapped to new
        // indexes of entries
        let mut tree = vec![];
        let mut local_ids = vec![];
        let mut local_parents = vec![];
        let total = self.entries[self.root_index]
            .agg_data
            .as_ref()
            .unwrap()
            .size as f32;
        while let Some(next_entry_index) = queue.pop() {
            let next_entry = &self.entries[next_entry_index];
            let next_agg = next_entry.agg_data.as_ref().unwrap();
            let mut tail_size = 0;
            let mut tail_file_count = 0;
            let mut tail_dir_count = 0;
            let agg_index = entries.len();
            index_map.map(next_entry_index, agg_index);
            tree.push(vec![]);
            local_ids.push(next_entry_index);
            local_parents.push(self.entries[next_entry_index].parent);
            for next_nested_index in &self.entries[next_entry_index].nested {
                let next_nested_index = *next_nested_index;
                let next_nested = &self.entries[next_nested_index];
                let next_nested_agg = next_nested.agg_data.as_ref().unwrap();
                if (next_nested_agg.size as f32) / total < up_to_fraction {
                    tail_size += next_nested_agg.size;
                    tail_file_count += next_nested_agg.file_count;
                    tail_dir_count += next_nested_agg.dir_count;
                } else {
                    tree[agg_index].push(next_nested_index);
                    queue.push(next_nested_index);
                }
            }
            entries.push(AggregateEntry::new(
                next_entry_index as i64,
                next_entry_index as i64,
                next_entry.name.clone(),
                get_string_hash(&next_entry.name),
                next_entry.path.to_string_lossy().as_ref().to_owned(),
                next_agg.self_size as i64,
                next_agg.size as i64,
                tail_size as i64,
                next_agg.self_file_count as i64,
                next_agg.file_count as i64,
                tail_file_count as i64,
                next_agg.self_dir_count as i64,
                next_agg.dir_count as i64,
                tail_dir_count as i64,
                next_entry.is_file,
                self.entries[next_entry_index].parent.map(|i| i as i64),
                self.entries[next_entry_index].parent.map(|i| i as i64),
                vec![],
            ));
        }
        // remap global ids to local ids
        let tree: Vec<Vec<i64>> = tree
            .into_iter()
            .map(|ii| {
                let mut ii = ii
                    .into_iter()
                    .map(|i| index_map.index(i))
                    .collect::<Vec<_>>();
                ii.sort_by_key(|i| Reverse(entries[*i].size));
                ii.into_iter().map(|i| i as i64).collect()
            })
            .collect();
        let local_ids: Vec<i64> = local_ids
            .into_iter()
            .map(|i| index_map.index(i) as i64)
            .collect();
        let local_parents: Vec<Option<i64>> = local_parents
            .into_iter()
            .map(|ii| ii.and_then(|i| index_map.try_index(i)))
            .map(|ii| ii.map(|i| i as i64))
            .collect();
        for (i, e) in entries.iter_mut().enumerate() {
            e.nested = tree[i].clone();
            e.local_id = local_ids[i];
            e.local_parent = local_parents[i];
        }
        entries
    }

    pub fn rescan(&mut self) {
        *self = Self::new(&self.entries[self.root_index].path);
    }

    pub fn jump(&mut self, handle: usize) {
        self.root_index = handle;
    }

    pub fn navigate(&mut self, handle: usize, path: &str) {
        let path: PathBuf = path.into();
        if self
            .entries
            .get(handle)
            .map(|e| e.path == path)
            .unwrap_or_default()
        {
            self.root_index = handle;
        } else {
            *self = Self::new(&path);
        }
    }

    pub fn level_up(&mut self) -> Option<PathBuf> {
        if let Some(parent) = self.entries.get(self.root_index).and_then(|e| e.parent) {
            // already scanned have parent's index
            self.root_index = parent;
            Some(self.entries[self.root_index].path.clone())
        } else {
            // root is on top, need rescan
            let root_path = self.entries.get(self.root_index)?.path.parent()?.to_owned();
            *self = Self::new(&root_path);
            Some(root_path)
        }
    }
}
