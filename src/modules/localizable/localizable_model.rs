
/// Структура для модели локализации
#[derive(Debug)]
pub struct LocalizableModel {
    pub key: String,
    pub value: String,
}

impl LocalizableModel {
    pub fn to_swift_property(&self) -> String {
        format!("public static let {}: NSLocalizedString = .init(\"{}\", comment: .empty)", self.key, self.key)
    }
}