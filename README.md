# pinyin_zhuyin

A library to convert between pinyin and zhuyin.

The original library is Bomin Zhang's Go library "zhuyin".

https://github.com/localvar/zhuyin (Golang version)

https://github.com/DictPedia/ZhuyinPinyin (PHP version)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pinyin_zhuyin = "0.2"
```

## API
```rust
encode_pinyin("zhang1") // zhāng
decode_pinyin("zhāng") // zhang1

encode_zhuyin("zhang1") // ㄓㄤ
decode_zhuyin("ㄓㄤ") // zhang1

pinyin_to_zhuyin("zhāng") // ㄓㄤ
zhuyin_to_pinyin("ㄓㄤ") // zhāng

split("zhang1") // ("zh", "ang", 1)
```
