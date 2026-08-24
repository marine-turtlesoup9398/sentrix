use sentrix_ir::Language;
use tree_sitter::{Language as TsLanguage, Parser};

pub struct TreeSitterParser;

impl TreeSitterParser {
    pub fn get_language(lang: &Language) -> Option<TsLanguage> {
        match lang {
            Language::Python => Some(tree_sitter_python::LANGUAGE.into()),
            Language::TypeScript => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            Language::JavaScript => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            Language::Go => Some(tree_sitter_go::LANGUAGE.into()),
            Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            _ => None,
        }
    }

    pub fn create_parser(lang: &Language) -> Option<Parser> {
        let ts_lang = Self::get_language(lang)?;
        let mut parser = Parser::new();
        if parser.set_language(&ts_lang).is_ok() {
            Some(parser)
        } else {
            None
        }
    }
}
