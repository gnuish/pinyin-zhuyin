# pinyin_zhuyin

A library to convert between and work with pinyin and zhuyin.

The original library is Bomin Zhang's Go library "zhuyin".

https://github.com/localvar/zhuyin (Golang version)
https://github.com/DictPedia/ZhuyinPinyin (PHP version)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pinyin_zhuyin = "0.1"
```

and this to your crate root:

```rust
extern crate pinyin_zhuyin;
```

### API
```rust
encode_pinyin("zhang1") // zhǎng
decode_pinyin("zhǎng") // zhang1

encode_zhuyin("zhang1") // ㄓㄤ
decode_zhuyin("ㄓㄤ") // zhang1

pinyin_to_zhuyin("zhǎng") // ㄓㄤ
zhuyin_to_pinyin("ㄓㄤ") // zhǎng

split("zhang1") // ("zh", "ang", 1)
```
