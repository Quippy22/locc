use tokei::LanguageType;

pub const SKIP_TYPES: &[LanguageType] = &[
    LanguageType::Json,
    LanguageType::Yaml,
    LanguageType::Toml,
    LanguageType::Ini,
    LanguageType::Xml,
    LanguageType::RON,
    LanguageType::Sql,
    LanguageType::Jsonnet,
    LanguageType::Thrift,
    LanguageType::LinkerScript,
    LanguageType::ModuleDef,
    LanguageType::SRecode,
    LanguageType::Markdown,
    LanguageType::ReStructuredText,
    LanguageType::AsciiDoc,
    LanguageType::Org,
];

pub fn lang_color(lang: &LanguageType) -> (u8, u8, u8) {
    match lang {
        LanguageType::Rust => (222, 165, 132),
        LanguageType::Python => (53, 114, 165),
        LanguageType::JavaScript => (241, 224, 90),
        LanguageType::TypeScript => (49, 120, 198),
        LanguageType::Go => (0, 173, 216),
        LanguageType::Ruby => (112, 21, 22),
        LanguageType::C => (85, 85, 85),
        LanguageType::Cpp => (243, 75, 125),
        LanguageType::CSharp => (23, 134, 0),
        LanguageType::Java => (176, 114, 25),
        LanguageType::Bash | LanguageType::Sh | LanguageType::Zsh | LanguageType::Fish
        | LanguageType::Nushell | LanguageType::Ksh => (137, 224, 81),
        LanguageType::Html => (227, 76, 38),
        LanguageType::Css => (86, 61, 124),
        LanguageType::Dart => (0, 180, 171),
        LanguageType::Kotlin => (169, 123, 255),
        LanguageType::Swift => (240, 81, 56),
        LanguageType::Zig => (236, 130, 21),
        LanguageType::Lua => (0, 0, 128),
        LanguageType::Haskell => (94, 80, 134),
        LanguageType::Scala => (194, 45, 64),
        LanguageType::Elixir => (78, 47, 99),
        LanguageType::Clojure => (219, 88, 55),
        LanguageType::Perl => (2, 152, 130),
        LanguageType::R => (25, 145, 208),
        LanguageType::ObjectiveC => (67, 142, 255),
        LanguageType::VimScript => (19, 191, 123),
        _ => (136, 136, 136),
    }
}
