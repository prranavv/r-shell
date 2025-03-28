use std::{env, fs, io::{stdin, stdout, Write}, path::Path, process::Command};

use pathsearch::find_executable_in_path;

fn read_event_loop(){
    loop{
        
        print!("$ ");
        let mut input = String::new();
        let _ =stdout().flush().unwrap();
        let _ = stdin().read_line(&mut input);
        let mut iter =input.split_whitespace();
        let head =iter.next();
        let tail = iter.collect::<Vec<&str>>().join(" ");
        match head{
            Some("exit")=>{
                std::process::exit(0);
            }
            Some("echo")=>{
                println!("{}",tail);
            }
            Some("pwd")=>{
                let path = env::current_dir();
                match path{
                    Ok(val)=>println!("{}",val.display()),
                    _=>()
                }
            }

            Some("type")=>{
                match tail.as_str(){
                    "echo" | "pwd" | "exit" | "type" => println!("{} is a shell builtin",tail),
                    command =>{
                        let x = pathsearch::find_executable_in_path(command);
                        match x{
                            Some(val)=>println!("{} is {}",command,val.display()),
                            None=>println!("{}: not found",tail)
                        }
                    }
                }
            }

            Some("cd")=>{
                match tail.as_str(){
                    "~"=>{
                        let path = Path::new("/home/pranav");
                        let result = env::set_current_dir(path);
                        match result{
                            Err(_)=>{
                                println!("Error here")
                            }
                            _=>()
                        }
                    }

                    x=>{
                        let path = Path::new(x);
                        let result = env::set_current_dir(path);
                        match result{
                            Err(_)=>{
                                println!("cd {}: No such working directory",x);
                            }
                            _=>()
                        }
                    }
                }
            }

            Some("ls")=>{
                let path = env::current_dir().unwrap();
                let files =fs::read_dir(path).unwrap();
                for file in files{
                    if let Ok(file)=file{
                        print!("{} ",file.file_name().into_string().unwrap())
                    }
                }
                println!("")
            }

            Some(val)=>{
                match find_executable_in_path(val){
                    Some(_)=>{
                        run_binary(val,&tail);
                    }

                    None=>println!("{}: command not found",input.trim())
                }
            }

            _=>{
                println!("{}: command not found.",input.trim())
            }
        }
    }
}


fn run_binary(binary: &str,arg: &str){
    let child= Command::new(binary).arg(arg).output().unwrap();
    if child.status.success(){
    println!("{}",String::from_utf8_lossy(&child.stdout))
    }else {
        eprintln!("{}",String::from_utf8_lossy(&child.stderr))
    }
}

fn main(){
    read_event_loop();    
}