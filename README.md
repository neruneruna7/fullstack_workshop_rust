# Rust Fullstack Workshop
を実施した記録

https://bcnrust.github.io/devbcn-workshop/index.html

## 環境

- Rust ~~1.74.0~~ 1.75.0にバージョンアップ
- wsl2 2.0.9.0
- Ubuntu 22.04.3 LTS

# 進行状況

## サーバサイド
ほぼ完了
shuttle_static_folderが廃止になったので，その部分は適宜読み替えて実装した．
Shuttle.toml にassetsの設定を追加することで，静的ファイルを読み込めるようになった．

shuttleのversion0.37では，async-trait安定化後のRustを使わないとコンパイルできない様子（推測に過ぎないが，大きく外れてはいないだろう）

## クライアントサイド
dioxus-cliのコマンド名がdxになっていることに注意
それに伴い，Makefile.tomlのfront-buildのコマンドを修正する必要がある

~~現在dioxusがうまくビルドできないため，中断中~~
ビルドに成功
tailwindの問題だったのか？（結局原因は不明）

エラーメッセージに記述のあるように，wasm-bindgen-cliのバージョンを変更していみたりもしたが，改善せず

以下エラーメッセージ
```
 dx serve
[INFO] 🚅 Running build command...
\ ⚙️ Compiling front 0.1.0 (path+file:///home/hariboteereg/cod[INFO] 👑 Build done.
[WARN] failed to parse `name` custom section Invalid name type (at offset 27030253)
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: 

it looks like the Rust project used to create this wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust wasm file schema version: 0.2.89
     this binary schema version: 0.2.87

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or 
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise 0.2.87

don't forget to recompile your wasm file! Alternatively, you can update the 
binary with:

    cargo install -f wasm-bindgen-cli --version 0.2.89

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
', /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-cli-0.4.1/src/builder.rs:129:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: 🚫 Serving project failed: Build Failed: Bindgen build failed! 
This is probably due to the Bindgen version, dioxus-cli using `0.2.81` Bindgen crate.
```