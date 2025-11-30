# gzy

**gzy** は Git 操作をより速く・簡単にするためのシンプルな CLI ツールです。  
Git の `add` / `commit` / `branch` / `push` などを短いコマンドで実行できます。

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

---

## 使い方

```bash
# インストール
cargo install gzy

# リポジトリを初期化
gzy init

# ファイルを追加
gzy add <file>

# コミット
gzy commit "メッセージ"

# ブランチ作成
gzy branch dev

# リモートリポジトリを追加
gzy remote https://github.com/user/repo.git

# プッシュ
gzy push main

# クローン
gzy clone https://github.com/user/repo.git

# 新しいブランチを作成してチェックアウト
gzy checkout develop

# ステータスを表示
gzy status

# ログを表示
gzy log
