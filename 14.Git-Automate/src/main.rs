/*
 * @Author: yk
 * @Date: 2024-07-19 10:58:52
 * @Description: This is a simple git-auto script with add,commit,push 
 */
use std::fs::OpenOptions;
use std::io::{Write,Read};
use std::process::{Command,exit};

fn add_commit_push() {

    // Set the dynamic commit_num

    let mut commit_num = 0;

    /*  
    1. Search current commit_num
     
     Firstly, we try to read the `commit_count.txt`,
     we set read(true)----It means if this file exist, 
     we will get it by parse.
    */

    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .open("commit_count.txt"){
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            commit_num = content.trim().parse().unwrap();
        }
    
    commit_num += 1;

    // 2.Set Command
    
    // (1)Basic add command: `git add -A`
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Filed to excute git add command");

    if !add_command.status.success(){
        eprintln!("Error: Filed to add files to git repo");
        exit(1);
    }

    // (1)Basic add command: `git commit -m "xxxx"`   
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("{commit_num}"))
        .output()
        .expect("Filed to excute git commit command");

    if !commit_command.status.success(){
        eprintln!("Error: Filed to commit changes");
        exit(1);
    }

    /*
    3. Update the commit_num

    `set create(true)`-----Means file will be create if not exists
    `set write(true)`----Means file exists can be excute

    Then we open the file and write the commit_num into it.

     */
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .open("commit_count.txt"){
            file.write_all(commit_num.to_string().as_bytes()).unwrap();
        }
    
    // (3) Basic push command: 'git push origin master'
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Filed to excute push command");

    if !push_command.status.success(){
        eprintln!("Error: Filed to push changes");
        exit(1);
    }

    println!("Successfully! add commit push have been fished!");

}

fn main(){
    add_commit_push();

}