// #![windows_subsystem = "windows"]

use foldeye::*;
use directories::UserDirs;
use std::fs;

mod folderkind;

fn main() -> Result<(), std::io::Error> {
    if let Some(path) = get_downloads() {
        let directory = Directory::new(&path.clone())?;
        let chron_timing = "1/1 * * * * *".to_string();

        let chron = Chron::new(path.clone(), chron_timing, directory);
        chron.watch_folder(&path, &|comparison| {
            for action in comparison.action {
                match action {
                    ComparisonActionEnum::Inserted(files) => { 
                        files_inserted_event(files, &path) 
                    }
                    ComparisonActionEnum::Removed(files) => {
                        files_removed_event(files, &path)
                    }
                    ComparisonActionEnum::Replaced(_) => {}
                }
            }
        })?;
    } else {
        println!("Could not find downloads folder");
    }

    Ok(())
}

fn files_removed_event(files: Vec<String>, path: &String) {
    clear_empty_folders(&path);

    let is_part = files.iter().any(|file| { file.contains("part") });
    if is_part {
        move_remaining_files(path);
    }
}

fn move_remaining_files(path: &String){
    //get all files in path
    let files = fs::read_dir(path).unwrap();
    //filter files that are not in a folder
    let files = files.filter(|file| {
        let file = file.as_ref().unwrap();
        let file = file.path();
        let file = file.to_str().unwrap();
        !file.contains("\\")
    });
    //move files to folder
    for file in files {
        let file = file.unwrap();
        let file = file.path();
        let file = file.to_str().unwrap();
        let file = file.to_string();
        let file_name = file.split("\\").last().unwrap().to_string();
        let folder_name = file_name.split(".").next().unwrap().to_string();
        let folder_path = format!("{}\\{}", path, folder_name);
        let file_path = format!("{}\\{}", path, file_name);
        fs::create_dir(&folder_path).unwrap();
        fs::rename(&file_path, &format!("{}\\{}", folder_path, file_name)).unwrap();
    }
}

fn files_inserted_event(files: Vec<String>, path: &String) {
    clear_empty_folders(&path);

    if files.len() > 0 {
        files.iter().for_each(|file| {
            let extension = file.split(".").last().unwrap();
            let file_name = file.split("/").last().unwrap();
            let dir = create_dir(&path, extension);

            let new_file_path = format!("{}/{}", &dir, &file_name);

            let result = fs::rename(&file, &new_file_path);
            match result {
                Ok(_) => println!("Moved file {} to {}", &file, &new_file_path),
                Err(e) => { println!("Error moving file {} to {}: {}", &file, &new_file_path, &e); }
            }
        });
    }
}

fn clear_empty_folders(path: &String) {
    let dir = fs::read_dir(path).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir = fs::read_dir(&path).unwrap();
            if dir.count() == 0 {
                fs::remove_dir(&path).unwrap();
            }
        }
    }
}

fn create_dir(path: &str, name: &str) -> String {
    let mut new_path = "".to_string();
    if let Some(kind) = folderkind::FolderKind::new().get_kind(name) {
        new_path = format!("{}/{}", path, kind.to_string())
    }

    println!("{}", new_path);

    match fs::create_dir_all(&new_path) {
        Ok(_) => {}
        Err(e) => println!("Error creating directory: {}", e)
    }

    new_path
}

fn get_downloads () -> Option<String> {
    match UserDirs::new().unwrap().download_dir() {
        Some(path) => Some(path.to_str().unwrap().to_string()),
        None => Some("/home/stiwie/Downloads".to_string()), //linux version
    }
}
