use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    French,
    Japanese,
    Spanish,
    Korean,
    Chinese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Japanese => "ja",
            Language::Spanish => "es",
            Language::Korean => "ko",
            Language::Chinese => "zh",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Japanese => "日本語",
            Language::Spanish => "Español",
            Language::Korean => "한국어",
            Language::Chinese => "中文",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::French,
            Language::Japanese,
            Language::Spanish,
            Language::Korean,
            Language::Chinese,
        ]
    }

    #[allow(dead_code)]
    pub fn from_code(code: &str) -> Option<Language> {
        match code {
            "en" => Some(Language::English),
            "fr" => Some(Language::French),
            "ja" => Some(Language::Japanese),
            "es" => Some(Language::Spanish),
            "ko" => Some(Language::Korean),
            "zh" => Some(Language::Chinese),
            _ => None,
        }
    }
}

pub struct I18n {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    pub current_language: Language,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = I18n {
            bundles: HashMap::new(),
            current_language: Language::English,
        };

        // Load all language bundles
        for lang in Language::all() {
            if let Some(bundle) = Self::load_bundle(lang) {
                i18n.bundles.insert(lang.code().to_string(), bundle);
            }
        }

        i18n
    }

    fn load_bundle(lang: Language) -> Option<FluentBundle<FluentResource>> {
        let ftl_string = match lang {
            Language::English => include_str!("../../locales/en.ftl"),
            Language::French => include_str!("../../locales/fr.ftl"),
            Language::Japanese => include_str!("../../locales/ja.ftl"),
            Language::Spanish => include_str!("../../locales/es.ftl"),
            Language::Korean => include_str!("../../locales/ko.ftl"),
            Language::Chinese => include_str!("../../locales/zh.ftl"),
        };

        let resource = FluentResource::try_new(ftl_string.to_string()).ok()?;
        let lang_id: LanguageIdentifier = lang.code().parse().ok()?;
        let mut bundle = FluentBundle::new(vec![lang_id]);

        if bundle.add_resource(resource).is_ok() {
            Some(bundle)
        } else {
            None
        }
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language
    }

    pub fn set_language(&mut self, lang: Language) {
        self.current_language = lang;
    }

    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, None)
    }

    pub fn t_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let lang_code = self.current_language.code();

        if let Some(bundle) = self.bundles.get(lang_code) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(pattern) = message.value() {
                    let mut errors = vec![];
                    let value = bundle.format_pattern(pattern, args, &mut errors);
                    return value.to_string();
                }
            }
        }

        // Fallback to English if translation not found
        if self.current_language != Language::English {
            if let Some(bundle) = self.bundles.get("en") {
                if let Some(message) = bundle.get_message(key) {
                    if let Some(pattern) = message.value() {
                        let mut errors = vec![];
                        let value = bundle.format_pattern(pattern, None, &mut errors);
                        return value.to_string();
                    }
                }
            }
        }

        // Ultimate fallback
        key.to_string()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
