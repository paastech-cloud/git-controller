
use std::{process::Command};


pub fn build_project(projectname: String, workingdir_path: String, dockerfile: String) {

    let dockerfile_path = format!("{}/{}", workingdir_path, dockerfile);

    
    let result = Command::new("/usr/bin/docker")
        .arg("build")
        .arg("-t")
        .arg(projectname.clone())
        .arg("-q")
        .arg("-f")
        .arg(dockerfile_path.clone())
        .arg(workingdir_path.clone())
        .output()
        .expect("failed to execute docker build");

    let stdout = String::from_utf8(result.stdout).unwrap();
    let stderr = String::from_utf8(result.stderr).unwrap().to_string();

    let result_split = stdout.split(":");
    let id = result_split.skip(1).next().unwrap();
    println!("Built Image: {}", id);

    if !stderr.is_empty() {
        println!("Error while building docker image of project: {}", projectname.clone());
        println!("{}", stderr);
        return;
    }
}

