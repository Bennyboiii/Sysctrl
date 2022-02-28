use std::collections::HashMap;
mod start;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let command = arguments.next().expect("command was not passed");

    //println!("The key is: {} and the value is: {}", key, value);

    if command == "create" {
        let service = arguments.next().expect("service was not passed");
        let path = arguments.next().expect("path was not passed");
        create_db_entry(service, path);
    } else if command == "delete" {
        let service = arguments.next().expect("service was not passed");
        delete_db_entry(service);

    } else  if command == "path" {
        let path = arguments.next().expect("path was not passed");
        create_input(path);

    } else if command == "start" {
        let service = arguments.next().expect("service was not passed");
        start::run(service);
    } else if command == "stop" {
        let service = arguments.next().expect("service was not passed");
        start::stop(service);
    } else if command == "help" {
        println!("HELP: Possible commands below");
        println!("create <service> <path>: <service> creates a service where service files can be placed. <path> registers the installation directory of the service.");
        println!("delete <service>: deletes a service.");
        println!("path <path>: registers a path where service folders go.");
        println!("start <service>: starts a service by placing the contents of the service folder into the installation directory.");
        println!("stop <service>:  deletes the service files from the installation directory, stopping the service.");
        println!("help: displays this message.");
        println!(" ");
        println!("ADDITIONAL HELP: To create a service, create a service folder and path, and install the service files to the service folder.");
        println!("For example, if I want to install multiple versions of CUDA, I would create service folders with my CUDA versions as names.");
        println!("the path for these services would be the NVIDIA drivers path.");
        println!("I would then install each CUDA version into their respective service folders by using the mv command, or doing it manually.");
        println!("Then, I can simply start different CUDA versions by typing 'sysctl stop CUDA10.2', 'systemctl start CUDA11.5'.");
        println!("This makes it easier for developers to use multiple versions of drivers or libraries if needed. This avoids dependency hell.");
    }
    else {
        panic!("Invalid Command");
    }
}

struct Database {
    map: HashMap< String, String >,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("assoc.db")?;
        for line in contents.lines() {
            let (service, path) = line.split_once('\t').expect("Corrupt database");
            map.insert(service.to_owned(), path.to_owned());
        }
        Ok(Database { map, flush: false})
    }

    fn insert(&mut self, service: String, path: String) {
        self.map.insert(service, path);
    }

    fn delete(&mut self, service: String) {
        self.map.remove(&service);
        self.map.remove(&service.to_uppercase());

    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
           let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
        for (service, path) in &database.map {
            contents.push_str(service);
            contents.push('\t');
            contents.push_str(path);
            contents.push('\n');
        }   
        std::fs::write("assoc.db", contents)
 
}

fn create_db_entry(service: String, path: String) {
    let mut database = Database::new().expect("Creating db crashed");
    let pathexists = std::path::Path::new(&path).exists();
    if pathexists {
        database.insert(service.to_uppercase(), path.clone());
        database.insert(service.clone(), path);
        database.flush().unwrap();
        let inputpath = start::parser("input".to_string(), true);
        let fullpath = format!("{}/{}/",inputpath ,service);
        std::fs::create_dir(fullpath).expect("Couldn't create directory");
    } else {
        panic!("Path does not exist.")
    }
}
fn create_input(path: String) {
    let mut database = Database::new().expect("Creating db crashed");
    let pathexists = std::path::Path::new(&path).exists();
    if pathexists {
        database.insert("input".to_string(), path);
        database.flush().unwrap(); 
    } else {
        panic!("Path does not exist.");
    }
}
fn delete_db_entry(service: String) {
    let mut database = Database::new().expect("Creating db crashed");
    database.delete(service);
    database.flush().unwrap();
}