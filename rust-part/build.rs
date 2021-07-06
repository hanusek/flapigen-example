use flapigen::{CppConfig, CppOptional, CppStrView, CppVariant, LanguageConfig};
use std::{env, path::Path};

fn main()
{
    let out_dir = env::var("OUT_DIR").expect("no OUT_DIR, but cargo should provide it");
    let cpp_cfg = CppConfig::new(Path::new("..").join("cpp-part").join("rust-api"), "rust".into())
    .cpp_optional(CppOptional::Std17)
    .cpp_variant(CppVariant::Std17)
    .cpp_str_view(CppStrView::Std17);
    let swig_gen = flapigen::Generator::new(LanguageConfig::CppConfig(cpp_cfg));
    swig_gen.expand(
        "c++-api-for-rust", Path::new("src/cpp_glue.rs.in"), &Path::new(&out_dir).join("cpp_glue.rs"));
    println!("cargo:rerun-if-changed={}", Path::new("src").join("cpp_glue.rs.in").display());
}
