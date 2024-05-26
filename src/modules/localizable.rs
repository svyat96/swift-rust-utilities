use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub mod localizable_model;

use localizable_model::LocalizableModel;

/// Функция для добавления недостающих ключей в файл локализации
/// 
/// # Аргументы
/// 
/// * `file_path` - Путь к файлу локализации
/// * `missing_keys` - Список ключей, которые нужно добавить
/// 
/// # Возвращает
/// 
/// * `Result<(), String>` - Результат операции
pub fn add_missing_keys_to_localizable_file(
    file_path: &PathBuf,
    missing_keys: Vec<String>,
) -> Result<(), String> {
    // Открываем файл для добавления в конец
    let file = OpenOptions::new().append(true).open(file_path)
        .map_err(|e| format!("Error opening file {:?}: {}", file_path, e))?;
    
    let mut writer = io::BufWriter::new(file);

    // Добавляем недостающие ключи в конец файла
    for key in missing_keys {
        writeln!(writer, r#""{}" = "";"#, key)
            .map_err(|e| format!("Error writing to file {:?}: {}", file_path, e))?;
    }

    Ok(())
}

/// Функция для парсинга файла локализации
/// 
/// # Аргументы
/// 
/// * `path` - Путь к файлу локализации
/// 
/// # Возвращает
/// 
/// * `Result<Vec<LocalizableModel>, Box<dyn std::error::Error>>` - Результат парсинга файла локализации
pub fn parse_localizable_file(
    path: &PathBuf,
) -> Result<Vec<LocalizableModel>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    // Создаем буферизированный читатель для файла
    let reader = io::BufReader::new(file);

    // Регулярное выражение для извлечения ключа и значения
    let re = Regex::new(r#"^\s*"([^"]+)"\s*=\s*"([^"]+)"\s*;"#)?;

    let mut result_vec: Vec<LocalizableModel> = vec![];

    // Перебираем строки файла
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let key = String::from(&caps[1]);
            let value = String::from(&caps[2]);
            result_vec.push(LocalizableModel { key, value })
        }
    }

    Ok(result_vec)
}

/// Функция для группировки файлов по логическим группам и языкам
/// 
/// # Аргументы
/// 
/// * `files` - Вектор путей к файлам
/// 
/// # Возвращает
/// 
/// * `HashMap<String, HashMap<String, PathBuf>>` - Сгруппированные файлы по логическим группам и языкам
pub fn group_files_by_logical_group_and_language(
    files: Vec<PathBuf>,
) -> HashMap<String, HashMap<String, PathBuf>> {
    let mut grouped_files: HashMap<String, HashMap<String, PathBuf>> = HashMap::new();

    for path in files {
        if let Some((key, language)) = parse_key_and_language(&path) {
            let language_files = grouped_files.entry(key).or_insert_with(HashMap::new);
            let paths = language_files.entry(language).or_default();
            paths.push(path);
        }
    }

    grouped_files
}

/// Функция для извлечения логической группы и языка из пути
/// 
/// # Аргументы
/// 
/// * `path` - Путь к файлу локализации
/// 
/// # Возвращает
/// 
/// * `Option<(String, String)>` - Логическая группа и язык, если удается извлечь
pub fn parse_key_and_language(path: &PathBuf) -> Option<(String, String)> {
    let re = Regex::new(r"^(.*)/([^/]+)\.lproj/([^/]+)\.strings$").unwrap();
    let path_str = path.to_str()?;

    if let Some(caps) = re.captures(path_str) {
        let base_path = &caps[1];
        let language = &caps[2];
        let file_name = &caps[3];
        let key = format!("{}/{}", base_path, file_name);
        Some((key, language.to_string()))
    } else {
        None
    }
}

/// Функция для извлечения ключей из файла локализации
/// 
/// # Аргументы
/// 
/// * `file_path` - Путь к файлу локализации
/// 
/// # Возвращает
/// 
/// * `Result<HashSet<String>, std::io::Error>` - Набор ключей, извлеченных из файла локализации
fn extract_keys(file_path: &PathBuf) -> Result<HashSet<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut keys = HashSet::new();

    // Регулярное выражение для извлечения ключей
    let re = Regex::new(r#"^\s*"([^"]+)"\s*=\s*"[^"]*"\s*;"#).unwrap();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            keys.insert(caps[1].to_string());
        }
    }
    Ok(keys)
}

/// Функция для проверки консистентности ключей между языками
/// 
/// # Аргументы
/// 
/// * `paths` - Срез путей к файлам локализаций
/// 
/// # Возвращает
/// 
/// * `ResultCheckKeys` - Результат проверки консистентности ключей
pub fn check_keys_consistency(paths: &[PathBuf]) -> ResultCheckKeys {
    let mut reference_keys: Option<HashSet<String>> = None;
    let mut all_keys: HashSet<String> = HashSet::new();
    let mut discrepancies: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for path in paths {
        let keys = match extract_keys(path) {
            Ok(keys) => keys,
            Err(err) => {
                return ResultCheckKeys::Error(format!("Error reading file {:?}: {}", path, err))
            }
        };

        // Обновляем полный список всех ключей
        all_keys = all_keys.union(&keys).cloned().collect();

        // Инициализация пустого списка для каждого языка
        discrepancies.insert(path.clone(), vec![]);

        if reference_keys.is_none() {
            reference_keys = Some(keys.clone());
        }
    }

    // Проверяем, чего не хватает в каждом языке
    for (lang_dir, keys) in discrepancies.iter_mut() {
        let path = paths
            .iter()
            .find(|p| p == &lang_dir)
            .unwrap();
        let file_keys = match extract_keys(path) {
            Ok(file_keys) => file_keys,
            Err(err) => {
                return ResultCheckKeys::Error(format!("Error reading file {:?}: {}", path, err))
            }
        };
        let missing_keys: HashSet<_> = all_keys.difference(&file_keys).cloned().collect();
        *keys = missing_keys.into_iter().collect();
    }

    for (_, value) in &discrepancies {
        if !value.is_empty() {
            return ResultCheckKeys::NonEquatable(discrepancies);
        }
    }
    ResultCheckKeys::Equatable()
}

/// Перечисление для результатов проверки консистентности ключей
pub enum ResultCheckKeys {
    Equatable(),
    NonEquatable(HashMap<PathBuf, Vec<String>>),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_keys() {
        let en_path = PathBuf::from("/en.lproj/Localizable.strings");
        let ru_path = PathBuf::from("/ru.lproj/Localizable.strings");
        let nl_path = PathBuf::from("/nl.lproj/Localizable.strings");
        let paths = vec![en_path, ru_path, nl_path];
        let result = check_keys_consistency(&paths);
        match result {
            ResultCheckKeys::Equatable() => assert!(true),
            _ => assert!(false, "Expected Equatable"),
        }
    }

    #[test]
    fn test_empty_path() {
        let path = PathBuf::from("");
        assert_eq!(parse_key_and_language(&path), None);
    }

    #[test]
    fn test_no_strings() {
        let path = PathBuf::from("/path/to/project/Supporting Files/en.lproj/");
        assert_eq!(parse_key_and_language(&path), None);
    }

    #[test]
    fn test_no_lproj() {
        let path = PathBuf::from("/path/to/project/Supporting Files/Localizable.strings");
        assert_eq!(parse_key_and_language(&path), None);
    }

    #[test]
    fn test_multiple_lproj() {
        let path = PathBuf::from(
            "/path/to/project/Supporting Files/en.lproj/subdir/ru.lproj/Localizable.strings",
        );
        assert_eq!(
            parse_key_and_language(&path),
            Some((
                "/path/to/project/Supporting Files/en.lproj/subdir/Localizable".to_string(),
                "ru".to_string()
            ))
        );
    }

    #[test]
    fn test_valid_path() {
        let path = PathBuf::from("/path/to/project/Supporting Files/en.lproj/Localizable.strings");
        assert_eq!(
            parse_key_and_language(&path),
            Some((
                "/path/to/project/Supporting Files/Localizable".to_string(),
                "en".to_string()
            ))
        );
    }

    #[test]
    fn test_valid_with_empty_tail() {
        let path = PathBuf::from("/en.lproj/Localizable.strings");
        assert_eq!(
            parse_key_and_language(&path),
            Some(("/Localizable".to_string(), "en".to_string()))
        );
    }
}
