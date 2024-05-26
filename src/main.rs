use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::io::{self, Write};

mod modules;

use modules::localizable::{ResultCheckKeys, check_keys_consistency, parse_localizable_file, group_files_by_logical_group_and_language, add_missing_keys_to_localizable_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Абсолютный путь к текущей директории
    let absolute_path_buf = env::current_exe()?.parent().unwrap().to_path_buf();

    println!(
        "Абсолютный путь к текущей директории: {:#?}",
        absolute_path_buf
    );

    // Путь до файла с локализацией
    let localizable_path = absolute_path_buf.join("Localizable.strings");

    print!("Путь до файла с локализацией: {:#?}", localizable_path);

    // let vec_loc = parse_localizable_file(&localizable_path)?;

    // println!("{:#?}", &vec_loc);

    let files = find_files(
        &absolute_path_buf,
        &vec!["strings"],
        &vec!["SBPWidget.framework"],
    )?;

    let v: Vec<String> = files
        .iter()
        .map(|path| {
            let localizables = parse_localizable_file(path).unwrap_or(vec![]);
            let vec_string: Vec<String> =
                localizables.iter().map(|v| v.to_swift_property()).collect();
            vec_string.join("\n")
        })
        .collect();

    let path_swift_file = Path::new("sw.txt");
    fs::write(path_swift_file, &v[0])?;

    println!("{:#?}", files);

    let grouped_files = group_files_by_logical_group_and_language(files);

    println!("{:#?}", grouped_files);

    let mut vec_hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (key, value) in grouped_files {
        if value.len() > 1 {
            let mut vec_path_buf: Vec<PathBuf> = vec![];
            for (_, path) in value {
                vec_path_buf.push(path);
            }
            vec_hash_map.insert(key, vec_path_buf);
        }
    }

    println!("Словарь: {:#?}", vec_hash_map);

    for (key, value) in vec_hash_map {
        match check_keys_consistency(&value) {
            ResultCheckKeys::Equatable() => println!("Не требуется обновление! {}", key),
            ResultCheckKeys::NonEquatable(hash) => { 
                println!("{:#?}: {:#?}", key, hash);

                for (file_path, missing_keys) in hash {
                    match add_missing_keys_to_localizable_file(&file_path, missing_keys) {
                        Ok(_) => println!("{:#?} Добавили!", file_path),
                        Err(err) => println!("{:#?}: {}", file_path, err),
                    }
                }
            },
            ResultCheckKeys::Error(err) => println!("{}", err),
        }
    }

    Ok(())
}

fn find_files(
    dir: &PathBuf,
    extensions: &Vec<&str>,
    exclude: &Vec<&str>,
) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = &path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if path.is_dir() && !exclude.contains(name) {
            files.extend(find_files(&path, extensions, exclude)?);
        } else if let Some(ext) = path.extension() {
            let extension = &ext.to_str().unwrap_or_default();
            if extensions.contains(extension) {
                files.push(path);
            }
        }
    }
    Ok(files)
}
