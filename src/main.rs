/*
    find 命令：
        先实现当前目录查找文件的匹配
        再拓展到子目录
        然后怎加复杂的搜索 
        最后增加并发功能 这样子提高性能
*/

use std::{env, io::Error};
use walkdir::WalkDir;
use colored::*;


fn main() -> Result<(),Error>
{
    let test = WalkDir::new(".");

    // for entry in test{
    //     println!("{}",entry?.path().display());
    // }

    // 获取参数值 当前目录下完全遍历
    let arg:Vec<String> = env::args().collect();

    if arg.len() == 1 {
        println!("参数为0")
    }else {
        let found = while_arg(test, &arg);
        if !found{
            println!("找不到目标文件");
        }
    }

    Ok(())

}

// 循环打印函数
fn iter_print(dir: WalkDir){
    for entry in dir.into_iter().flat_map(|e| e.ok()){
        println!("{}",entry.path().display());
    }
}

fn while_arg(test:WalkDir,arg:&Vec<String>) -> bool{
    let file_name = arg[1].clone(); 
    let mut found = false;        
    match file_name.as_str() {
        "-a" => {iter_print(test);
                found = true},

        _ => {
                for entry in test.into_iter().flat_map(|e| e.ok()){
                    //println!("{}",entry.path().display());
                    let temp = entry.path().to_str().unwrap().to_string();
                    //println!("{}",temp);
                    if temp.contains(&file_name) {
                        println!("{}",temp.blue().bold());
                        found = true;
                    }
                }
            }
        }    
    found      
}
