use std::process::{exit, Command};
use std::io;



fn uploadrepo(){

    println!("wanna add a README.md file ?? type yes or no");
    
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("failed to read");

    let trimmedchoice = choice.trim();


    let init = Command::new("git")
        .arg("init")
        .output()
        .expect("couldn't initialize the repo");

    if !init.status.success(){
        println!("failed to initialize git");
        exit(0);
    }

    match trimmedchoice {
        "yes" => {
            let addreadme = Command::new("git")
                            .arg("add")
                            .arg("README.md")
                            .output().expect("failed to add readme");
            if !addreadme.status.success(){
                println!("failed to add readme");
                exit(0);
            }

            println!("added README.md");
        }
        "no" => println!("did not add a README file"),
        _ => { 
            println!("invalid choice");
            exit(1);
        }
    }


    let commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("first commit")
        .output()
        .expect("failed to commit changes");

    if !commit.status.success(){
        println!("commiting changs failed");
        exit(0)
    }

    let create_branch = Command::new("git")
        .arg("branch")
        .arg("-M")
        .arg("main")
        .output()
        .expect("failed to create branch");

    if !create_branch.status.success(){
        println!("failed to create branch");
        exit(0)
    }

    
    println!("enter remote origin link :");
    let mut remoteorigin = String::new();
    io::stdin().read_line(&mut remoteorigin).expect("error at remote origin");

    let trimmedremote = remoteorigin.trim();



    match check_link(trimmedremote) {
        true => println!("valied git repo repo {}", trimmedremote),
        false => {
            println!("invalid repo");
            exit(1);
        }


    }

    let remote = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(trimmedremote)
        .output()
        .expect("failed to remote");

    if !remote.status.success(){
        println!("failed to remote");
        exit(1)
    }

    let push = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .output()
        .expect("failed t o push");

    if !push.status.success(){
        println!("failed to push to repo");
        exit(1);
    }

}


fn check_link(link: &str) -> bool{
    link.starts_with("https://github.com/")
}



fn updaterepo(){
    let command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to run git");
    

    if !command.status.success(){
        println!("adding failed");
        exit(0)
    }

    let commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("some changes")
        .output()
        .expect("failed to commit changes");

    if !commit.status.success(){
        println!("commiting changs failed");
        exit(0)
    }

    let push = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("failed to push");

    if !push.status.success(){
        println!("pushing failed");
        exit(0)
    }
    
}


fn main(){
    println!("choose operation : commitchanges or newrepo");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("failed to input");
    let trimmedoperation = operation.trim();

    match trimmedoperation {
        "commitchanges" => updaterepo(),
        "newrepo" => uploadrepo(),
        _ => {
            println!("invalied choice");
            exit(1);
        }
    }
}
