**Total** is a tool for disk space usage visualization.

![App screenshot](/screenshot.png "App screenshot")

**Total** uses flame graphs to show space usage. Left clicks allow navigating inside selected folder, while right clicks will open folder in default file manager.

# Download

todo: link to releases

# Build

First install rust, node and tauri cli when run:

```
$ npm install
$ cargo tauri build
```

Compiled binary files will be located inside `src-tauri/target/release/` and `src-tauri/target/release/bundle/`.

**note:** code in `src/ipc/` and in `src-tauri/src/ipc/` is generated from `ipc.leap`, which describes types using [leap-lang](https://github.com/rsk700/leap-lang), code generator is not open source yet, but I'm planning to open it later when have time.

# More links

Also I'm making note taking, todo list app [Heaplist](https://heaplist.app/).