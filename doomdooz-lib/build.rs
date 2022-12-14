use globwalk;

use std::fs;
use tera::Tera;

const HEADER: &str = "// THIS CODE IS GENERATED BY BUILD SCRIPT\n";

fn get_submodules(module: &str) -> Vec<String> {
    let mut submodules: Vec<String> = vec![];

    for submodule in globwalk::glob("src/cop/".to_owned() + module + "/*.rs").unwrap() {
        if let Ok(submodule) = submodule {
            let filename = submodule.path().file_name().unwrap().to_str().unwrap();
            let (submodule_name, _) = filename.split_once(".").unwrap();

            submodules.push(submodule_name.to_owned());
        }
    }

    submodules
}

fn main() {
    for template in globwalk::glob("src/cop/*.template").unwrap() {
        if let Ok(template) = template {
            let filename = template.path().file_name().unwrap().to_str().unwrap();
            let (module_name, _) = filename.split_once(".").unwrap();

            println!("module name : {}", module_name);
            let submodules = get_submodules(module_name);

            println!("submodules : {:?}", submodules);

            let tera = Tera::new("src/cop/*.template").unwrap();
            let mut context = tera::Context::new();
            context.insert("submodules", &submodules);

            let content = HEADER.to_string() + &tera.render(filename, &context).unwrap();

            fs::write("src/cop/".to_owned() + module_name + ".rs", content).unwrap();
        }
    }
}
