//! PHF compile-time maps
extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file = BufWriter::new(File::create(path).unwrap());

    write_pinyin_map(&mut out_file);
    write_zhuyin_map(&mut out_file);
}

#[rustfmt::skip]
fn write_pinyin_map<W: Write>(file: &mut W) {
    write!(file, "static MAP_P2Z: phf::Map<&str, &str> = {}",
    phf_codegen::Map::new()
        .entry("b", "\"ㄅ\"").entry("d", "\"ㄉ\"").entry("g", "\"ㄍ\"")
        .entry("p", "\"ㄆ\"").entry("t", "\"ㄊ\"").entry("k", "\"ㄎ\"")
        .entry("m", "\"ㄇ\"").entry("n", "\"ㄋ\"").entry("h", "\"ㄏ\"")
        .entry("f", "\"ㄈ\"").entry("l", "\"ㄌ\"")

        .entry("j", "\"ㄐ\"").entry("zh", "\"ㄓ\"").entry("z", "\"ㄗ\"")
        .entry("q", "\"ㄑ\"").entry("ch", "\"ㄔ\"").entry("c", "\"ㄘ\"")
        .entry("x", "\"ㄒ\"").entry("sh", "\"ㄕ\"").entry("s", "\"ㄙ\"")
                             .entry("r" , "\"ㄖ\"")

        .entry("i", "\"ㄧ\"").entry("a", "\"ㄚ\"").entry("ai", "\"ㄞ\"").entry("an" , "\"ㄢ\"")
        .entry("u", "\"ㄨ\"").entry("o", "\"ㄛ\"").entry("ei", "\"ㄟ\"").entry("en" , "\"ㄣ\"")
        .entry("v", "\"ㄩ\"").entry("e", "\"ㄜ\"").entry("ao", "\"ㄠ\"").entry("ang", "\"ㄤ\"")
                                                  .entry("ou", "\"ㄡ\"").entry("eng", "\"ㄥ\"")

        .entry("ia" , "\"ㄧㄚ\"").entry("ua" , "\"ㄨㄚ\"").entry("ing", "\"ㄧㄥ\"").entry("iang", "\"ㄧㄤ\"")
        .entry("ie" , "\"ㄧㄝ\"").entry("uo" , "\"ㄨㄛ\"").entry("ong", "\"ㄨㄥ\"").entry("uang", "\"ㄨㄤ\"")
        .entry("iao", "\"ㄧㄠ\"").entry("uai", "\"ㄨㄞ\"").entry("ue" , "\"ㄩㄝ\"").entry("iong", "\"ㄩㄥ\"")
        .entry("iu" , "\"ㄧㄡ\"").entry("ui" , "\"ㄨㄟ\"").entry("ve" , "\"ㄩㄝ\"")
        .entry("ian", "\"ㄧㄢ\"").entry("uan", "\"ㄨㄢ\"").entry("van", "\"ㄩㄢ\"")
        .entry("in" , "\"ㄧㄣ\"").entry("un" , "\"ㄨㄣ\"").entry("vn" , "\"ㄩㄣ\"")

        .entry("er", "\"ㄦ\"")
        .entry("y", "\"ㄧ\"").entry("w", "\"ㄨ\"")

        .build()
    )
    .unwrap();
    writeln!(file, ";").unwrap();
}

#[rustfmt::skip]
fn write_zhuyin_map<W: Write>(file: &mut W) {
    write!(
        file,
        "static MAP_Z2P: phf::Map<&str, &str> = {}",
        phf_codegen::Map::new()
            .entry("ㄅ", "\"b\"").entry("ㄉ", "\"d\"").entry("ㄍ", "\"g\"")
            .entry("ㄆ", "\"p\"").entry("ㄊ", "\"t\"").entry("ㄎ", "\"k\"")
            .entry("ㄇ", "\"m\"").entry("ㄋ", "\"n\"").entry("ㄏ", "\"h\"")
            .entry("ㄈ", "\"f\"").entry("ㄌ", "\"l\"")

            .entry("ㄐ", "\"j\"").entry("ㄓ", "\"zh\"").entry("ㄗ", "\"z\"")
            .entry("ㄑ", "\"q\"").entry("ㄔ", "\"ch\"").entry("ㄘ", "\"c\"")
            .entry("ㄒ", "\"x\"").entry("ㄕ", "\"sh\"").entry("ㄙ", "\"s\"")
                                .entry("ㄖ", "\"r\"")

            .entry("ㄧ", "\"i\"").entry("ㄚ", "\"a\"").entry("ㄞ", "\"ai\"").entry("ㄢ", "\"an\"")
            .entry("ㄨ", "\"u\"").entry("ㄛ", "\"o\"").entry("ㄟ", "\"ei\"").entry("ㄣ", "\"en\"")
            .entry("ㄩ", "\"v\"").entry("ㄜ", "\"e\"").entry("ㄠ", "\"ao\"").entry("ㄤ", "\"ang\"")
                                .entry("ㄝ", "\"e\"").entry("ㄡ", "\"ou\"").entry("ㄥ", "\"eng\"")

            .entry("ㄧㄚ", "\"ia\"") .entry("ㄨㄚ", "\"ua\"") .entry("ㄧㄥ", "\"ing\"").entry("ㄧㄤ", "\"iang\"")
            .entry("ㄧㄝ", "\"ie\"") .entry("ㄨㄛ", "\"uo\"") .entry("ㄨㄥ", "\"ong\"").entry("ㄨㄤ", "\"uang\"")
            .entry("ㄧㄠ", "\"iao\"").entry("ㄨㄞ", "\"uai\"").entry("ㄩㄝ", "\"ve\"") .entry("ㄩㄥ", "\"iong\"")
            .entry("ㄧㄡ", "\"iu\"") .entry("ㄨㄟ", "\"ui\"")
            .entry("ㄧㄢ", "\"ian\"").entry("ㄨㄢ", "\"uan\"").entry("ㄩㄢ", "\"van\"")
            .entry("ㄧㄣ", "\"in\"") .entry("ㄨㄣ", "\"un\"") .entry("ㄩㄣ", "\"vn\"")

            .entry("ㄦ", "\"er\"")

            .build()
    )
    .unwrap();
    writeln!(file, ";").unwrap();
}
