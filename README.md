# Rust で WASM する際のメモ

## 環境構築

- Rust / wasm-pack のインストール

```shell
cargo new project-name --lib
```

でプロジェクトの作成

Cargo.toml に wasm-bindgen クレートを追加する。

## 作成

/src/lib.rs に コーディング

### ビルド

```shell
wasm-pack build
```

で/pkg 以下にファイルが生成される。

JS / TS で使用する際は、

- .wasm
- .wasm.d.ts
  の 2 種類が必要。

JS ファイルから直接 import でき、型補完まで有効になる。
