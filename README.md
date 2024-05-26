# Swift Project Utilities

Swift Project Utilities - это набор инструментов для автоматизации рутинных задач в Swift проектах. Эти утилиты призваны облегчить работу разработчиков, предоставляя автоматические решения для часто выполняемых задач, таких как работа с файлами локализации, управление ассетами и многое другое. 

## Оглавление

- [Особенности](#особенности)
- [Установка](#установка)
- [Использование](#использование)
- [Документация](#документация)
- [Планы на будущее](#планы-на-будущее)
- [Вклад](#вклад)
- [Лицензия](#лицензия)

## Особенности

### Работа с файлами локализации

- **Проверка консистентности ключей**: Утилита позволяет проверять, чтобы все файлы локализаций содержали одинаковые ключи.
- **Добавление недостающих ключей**: Автоматическое добавление недостающих ключей в файлы локализации.
- **Парсинг файлов локализаций**: Возможность извлечения ключей и значений из файлов локализаций.

## Установка

Для установки данного набора утилит необходимо клонировать репозиторий и собрать проект с помощью Cargo:

```sh
git clone https://github.com/your-username/swift-project-utilities.git
cd swift-project-utilities
cargo build --release
```

## Использование

### Проверка консистентности ключей

Для проверки консистентности ключей между файлами локализаций используйте функцию `check_keys_consistency`:

```rust
use swift_project_utilities::check_keys_consistency;
use std::path::PathBuf;

let paths = vec![
    PathBuf::from("path/to/en.lproj/Localizable.strings"),
    PathBuf::from("path/to/ru.lproj/Localizable.strings"),
    PathBuf::from("path/to/nl.lproj/Localizable.strings"),
];

let result = check_keys_consistency(&paths);
match result {
    ResultCheckKeys::Equatable() => println!("All keys are consistent across languages."),
    ResultCheckKeys::NonEquatable(discrepancies) => {
        println!("Key discrepancies found: {:?}", discrepancies);
    }
    ResultCheckKeys::Error(err) => println!("Error: {}", err),
}
```

### Добавление недостающих ключей

Для добавления недостающих ключей в файл локализации используйте функцию `add_missing_keys_to_localizable_file`:

```rust
use swift_project_utilities::add_missing_keys_to_localizable_file;
use std::path::PathBuf;

let file_path = PathBuf::from("path/to/ru.lproj/Localizable.strings");
let missing_keys = vec![
    "missingKey1".to_string(),
    "missingKey2".to_string(),
];

match add_missing_keys_to_localizable_file(&file_path, missing_keys) {
    Ok(_) => println!("Missing keys added successfully."),
    Err(err) => println!("Error: {}", err),
}
```

### Парсинг файлов локализаций

Для парсинга файлов локализаций используйте функцию `parse_localizable_file`:

```rust
use swift_project_utilities::parse_localizable_file;
use std::path::PathBuf;

let file_path = PathBuf::from("path/to/en.lproj/Localizable.strings");

match parse_localizable_file(&file_path) {
    Ok(localizables) => {
        for localizable in localizables {
            println!("Key: {}, Value: {}", localizable.key, localizable.value);
        }
    }
    Err(err) => println!("Error: {}", err),
}
```

## Документация

Подробная документация сгенерирована с помощью RustDoc. Для её просмотра выполните команду:

```sh
cargo doc --open
```

## Планы на будущее

- **Управление ассетами**: Автоматическое сканирование и генерация классов для работы с ассетами.
- **Генерация классов**: Автоматическое создание классов и структур на основе различных конфигурационных файлов.
- **Интеграция с CI/CD**: Внедрение возможностей для автоматической проверки и генерации необходимых файлов в процессе непрерывной интеграции и доставки.

## Вклад

Мы приветствуем вклад в проект. Пожалуйста, создавайте issues и pull requests на GitHub. Ваша помощь важна для улучшения данного набора утилит.

## Лицензия

Этот проект лицензируется на условиях лицензии MIT. Подробнее см. [LICENSE](LICENSE).
