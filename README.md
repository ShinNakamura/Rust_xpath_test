# これは何? #

XMLを解析して値を取り出すテスト

XPATH記法を使う


## 目的 ##

* `//foo`のようにXpathで値を取得する

* `<![CDATA[ ... ]]>`の解析

* `&lt;p>` => `<p>`

## Crate ##

```rust
extern crate sxd_document;
extern crate sxd_xpath;
```

## 参考 ##

[Crate sxd_xpath](https://docs.rs/sxd-xpath/0.4.2/sxd_xpath/)

[クローラ作成に必須！XPATHの記法まとめ](https://qiita.com/rllllho/items/cb1187cec0fb17fc650a)
