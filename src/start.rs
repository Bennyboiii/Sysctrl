

pub fn run(service: String) {
    let servicepath = parser(service.clone(), false);
    let inputpath = parser("input".to_string(), true);
    let outpath = format!("{}{}/",inputpath ,service);
    println!("sending contents of {} to {}",outpath, servicepath);
    for file in std::fs::read_dir(outpath).unwrap(){
        let filepath = file.unwrap().path();
        let path = filepath.to_string_lossy();
        copy_file(path.to_string(), servicepath.to_string());
    }
}

pub fn stop(service: String) {
    let servicepath = parser(service.clone(), false);
    let inputpath = parser("input".to_string(), true);
    let outpath = format!("{}/{}/",inputpath ,service);
    println!("deleting contents of {} in {}",outpath, servicepath);
    for file in std::fs::read_dir(outpath).unwrap(){
        let filepath = file.unwrap().path();
        let destpath = filepath.to_string_lossy();
        delete_file(destpath.to_string(), servicepath.to_string())
    }
}


pub fn parser(service: String, permission: bool) -> String {
    let contents = std::fs::read_to_string("assoc.db").unwrap();
    let serv = contents.find(&service).unwrap();
    let spl = contents.split_at(serv);
    let servpath = spl.1.split_once("\t").unwrap();
    let path = servpath.1.split_once("\n").unwrap();
    if service == "input" && !permission {
        panic!("Attempt to start 'input' service. This is not allowed: try sysctl path <path> to modify the sysctl service path.");
    } else {
        return path.0.to_string();
    }
}

fn copy_file(file: String, dest: String) {
    let filepath = std::path::Path::new(&file);
    let destpath = std::path::Path::new(&dest);
    let filename = filepath.file_name().unwrap();
    std::fs::copy(filepath, destpath.join(filename)).expect("could not copy");
}

fn delete_file(file: String, dest: String) {
    let filepath = std::path::Path::new(&file);
    let destpath = std::path::Path::new(&dest);
    let filename = filepath.file_name().unwrap();
    std::fs::remove_file(destpath.join(filename)).expect("could not stop service");
}
