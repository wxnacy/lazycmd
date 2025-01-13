# lazycmd

## 使用

### output

快速执行命令

```rust
use lazycmd::output;
let out = output(["which", "curl"]).unwrap();
assert_eq!(out, "/usr/bin/curl");
```

### spawn

执行命令并实时输出

```rust
use lazycmd::spawn;
spawn(["ls", "-l"]).unwrap();
```

```bash
total 32
-rw-r--r--@ 1 wxnacy  staff   372  1 13 20:29 Cargo.lock
-rw-r--r--@ 1 wxnacy  staff    96  1 13 20:58 Cargo.toml
-rw-r--r--@ 1 wxnacy  staff  1063  1 13 19:06 LICENSE
-rw-r--r--@ 1 wxnacy  staff   270  1 13 20:59 README.md
drwxr-xr-x@ 4 wxnacy  staff   128  1 13 20:41 src
drwxr-xr-x@ 5 wxnacy  staff   160  1 13 19:06 target

```

