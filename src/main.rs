use std::process::{Command , exit};
use names::Generator;

fn updaterepo(){
    let command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute");

    if !command.status.success(){
        eprintln!("failed to run git");
        exit(1)
    }else{
        println!("git run success");
    }



}


fn main(){
    updaterepo();
}
