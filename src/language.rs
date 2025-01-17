#[derive(PartialEq)]
pub enum Language {
    Rust,
    Python,
    Other(String),
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Other(language) => language.as_str(),
        }
    }

    #[inline]
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => Self::Rust,
            "py" => Self::Python,
            other_extension => Self::Other(other_extension.to_string()),
        }
    }

    /// language specific tree-sitter node types
    pub fn top_level_node_type(&self) -> &str {
        match self {
            Language::Rust => "source_file",
            Language::Python => "module",
            _ => "",
        }
    }

    pub fn decorator_node_type(&self) -> &str {
        match self {
            Language::Rust => "attribute_item",
            Language::Python => "null",
            _ => "",
        }
    }

    pub fn comment_node_type(&self) -> &str {
        match self {
            Language::Rust => "line_comment",
            Language::Python => "comment",
            _ => "",
        }
    }

    pub fn annotation_whitelist(&self) -> Vec<&str> {
        match self {
            Language::Rust => vec![
                "attribute_item",
                "mod_item",
                "enum_item",
                "type_item",
                "impl_item",
                "function_item",
                "struct_item",
                "trait_item",
                "macro_definition",
            ],
            Language::Python => vec![
                "class_definition",
                "function_definition",
                "decorated_definition",
            ],
            _ => vec![],
        }
    }

    pub fn nested_traversable_symbols(&self) -> Vec<&str> {
        match self {
            Language::Rust => vec!["mod_item", "impl_item"],
            Language::Python => vec!["class_definition"],
            _ => vec![],
        }
    }
}

impl From<&str> for Language {
    fn from(language_name: &str) -> Self {
        match language_name {
            "rust" => Self::Rust,
            "python" => Self::Python,
            other_language => Self::Other(other_language.to_string()),
        }
    }
}
