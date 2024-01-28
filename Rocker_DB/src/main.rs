use rocksdb::{DB, Options};
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 打开一个 RocksDB 数据库，如果数据库不存在则创建它
    let path = Path::new("file_index_db");
    let mut db = DB::open_default(path)?;

    // 假设我们有一些文件名和对应的路径
    let files = vec![
        ("main.rs", "./src/main.rs"),
        ("file2.txt", "/path/to/file2.txt"),
        ("file3.txt", "/path/to/file3.txt"),
    ];

    // 将文件名和路径存储到数据库
    for (file_name, file_path) in files {
        db.put(file_name.as_bytes(), file_path.as_bytes())?;
    }

    // 检索并打印一个文件路径
    let file_name = "main.rs";
    match db.get(file_name.as_bytes())? {
        Some(value) => println!("Found path for {}: {}", file_name, String::from_utf8(value)?),
        None => println!("File not found: {}", file_name),
    }

    Ok(())
}
