use std::path::Path;

pub fn detect(path: &Path) -> Option<(&'static str, (u8, u8, u8))> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" => Some(("Rust", (222, 165, 132))),
        "py" => Some(("Python", (53, 114, 165))),
        "js" => Some(("JavaScript", (241, 224, 90))),
        "ts" => Some(("TypeScript", (49, 120, 198))),
        "go" => Some(("Go", (0, 173, 216))),
        "rb" => Some(("Ruby", (112, 21, 22))),
        "c" | "h" => Some(("C", (85, 85, 85))),
        "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" => Some(("C++", (243, 75, 125))),
        "cs" => Some(("C#", (23, 134, 0))),
        "java" => Some(("Java", (176, 114, 25))),
        "sh" | "bash" | "zsh" | "fish" | "ksh" => Some(("Shell", (137, 224, 81))),
        "html" | "htm" => Some(("HTML", (227, 76, 38))),
        "css" => Some(("CSS", (86, 61, 124))),
        "dart" => Some(("Dart", (0, 180, 171))),
        "kt" | "kts" => Some(("Kotlin", (169, 123, 255))),
        "swift" => Some(("Swift", (240, 81, 56))),
        "zig" => Some(("Zig", (236, 130, 21))),
        "lua" => Some(("Lua", (0, 0, 128))),
        "hs" => Some(("Haskell", (94, 80, 134))),
        "scala" => Some(("Scala", (194, 45, 64))),
        "ex" | "exs" => Some(("Elixir", (78, 47, 99))),
        "clj" | "cljs" | "cljc" => Some(("Clojure", (219, 88, 55))),
        "pl" | "pm" => Some(("Perl", (2, 152, 130))),
        "r" | "R" => Some(("R", (25, 145, 208))),
        "m" | "mm" => Some(("Objective-C", (67, 142, 255))),
        "vim" => Some(("Vim script", (19, 191, 123))),
        _ => None,
    }
}
