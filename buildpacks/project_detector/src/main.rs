use std::fs;
use std::env;

fn file_match(filename: String) -> bool {
    let file_matches: Vec<String> = vec![
        String::from("pom.xml"), String::from("build.gradle"),
        String::from("package.json"), String::from("packages.config"),
        String::from("build.gradle.kts"), String::from("Gemfile"),
        String::from(".gitignore"), String::from("Cargo.toml"),
        String::from("go.mod"), String::from("Makefile"),
        String::from("project.toml"), String::from("pyproject.toml")
    ];
    return file_matches.contains(&filename.clone());
}

fn check_dir(dir_path: String) {
    let dir_content = fs::read_dir(dir_path.clone()).unwrap();
    for file in dir_content {
        let file_obj = file.unwrap();
        let filename = file_obj.file_name().into_string().unwrap();
        let filepath = String::from(file_obj.path().to_str().unwrap());

        if file_match(filename.clone()) {
            println!("Project found: {}", dir_path.clone());
            break;
        }

        if file_obj.file_type().unwrap().is_dir() {
            check_dir(String::from(filepath.clone()));
            continue;
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 {
        println!("Usage: {} <path>", arguments[0].clone());
        return;
    }

    let path = arguments[1].clone();
    check_dir(path);

}
