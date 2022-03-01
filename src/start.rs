use std::fs;
use std::path::Path;

// START/STOP

pub fn run(service: String) {
    let servicepath = parser(service.clone(), false, true);
    let inputpath = parser("input\t".to_string(), true, false);
    let outpath = format!("{}{}/",inputpath ,service);
    println!("copying contents of {} to {}",outpath, servicepath);
    for file in fs::read_dir(outpath).unwrap(){
        let filepath = file.unwrap().path();
        let path = filepath.to_string_lossy();
        copy_file(path.to_string(), servicepath.to_string());
    }
}

pub fn stop(service: String) {
    let servicepath = parser(service.clone(), false, true);
    let inputpath = parser("input\t".to_string(), true, false);
    let outpath = format!("{}/{}/",inputpath ,service);
    println!("deleting contents of {} in {}",outpath, servicepath);
    for file in fs::read_dir(outpath).unwrap(){
        let filepath = file.unwrap().path();
        let destpath = filepath.to_string_lossy();
        delete_file(destpath.to_string(), servicepath.to_string())
    }
}


pub fn parser(mut service: String, permission: bool, includetab: bool) -> String {
    let contents = fs::read_to_string("assoc.db").unwrap();
    let serv = contents.find(&service).unwrap();
    if includetab {
        service.push('\t');
    }
    let spl = contents.split_at(serv);
    let servpath = spl.1.split_once("\t").unwrap();
    let path = servpath.1.split_once("\n").unwrap();
    if service == "input" && !permission {
        panic!("Attempt to start 'input' service. This is not allowed: try sysctl path <path> to modify the sysctl service path.");
    } else {
        return path.0.to_string();
    }
}

pub fn list() {
    let database = fs::read_to_string("assoc.db").unwrap();
    println!("{}", database);
}


fn copy_file(file: String, dest: String) {
    let filepath = Path::new(&file);
    let destpath = Path::new(&dest);
    let filename = filepath.file_name().unwrap();
    fs::copy(filepath, destpath.join(filename)).expect("could not copy");
}

fn delete_file(file: String, dest: String) {
    let filepath = Path::new(&file);
    let destpath = Path::new(&dest);
    let filename = filepath.file_name().unwrap();
    fs::remove_file(destpath.join(filename)).expect("could not stop service");
}

// RULES

// pub fn ruleparser(service: String) {
//     let contents = fs::read_to_string("assoc.db").unwrap();
//     let parsed_contents = contents.split("\t");
//     println!("{:?}", parsed_contents);


// }
