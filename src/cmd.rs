use std::{ffi::OsStr, io::{self, BufRead}, process::{Child, Command, Stdio}};

use anyhow::{anyhow, Result};

/// 执行命令
///
/// Examples
///
/// ```
/// use lazycmd::output;
///
/// let out = output(["which", "curl"]).unwrap();
///
/// assert_eq!(out, "/usr/bin/curl");
/// ```
pub fn output<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<S> = args.into_iter().collect();
    let output = Command::new(&args[0])
        .args(&args[1..])
        .output()?;

    let outerr = String::from_utf8_lossy(&output.stderr);
    if !outerr.is_empty() {
        if let Some(e) = outerr.strip_suffix("\n") {
            return Err(anyhow!("{e}"));
        } else {
            return Err(anyhow!("{outerr}"));
        }
    }
    let out = String::from_utf8_lossy(&output.stdout);
    if let Some(o) = out.strip_suffix("\n") {
        return Ok(o.to_string());
    }
    Ok(out.to_string())
}


/// 运行命令并实时输出
///
/// Examples
///
/// ```
/// use lazycmd::spawn;
///
/// spawn(["ls", "-l"]).unwrap();
/// ```
pub fn spawn<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<S> = args.into_iter().collect();
    let mut child: Child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped()) // 将标准输出设置为管道
        .spawn()?; // 启动命令

    // 获取标准输出的句柄
    let stdout = child.stdout.take().ok_or_else(|| anyhow!("Could not capture standard output"))?;

    // 创建一个 BufReader 来逐行读取输出
    let reader = io::BufReader::new(stdout);

    // 逐行读取输出并打印
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    // 等待命令完成
    let _ = child.wait()?;
    Ok(())
}

