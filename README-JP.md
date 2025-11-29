# gzy

**gzy** は Git 操作をより速く・簡単にするためのシンプルな CLI ツールです。  
Git の `add` / `commit` / `branch` / `push` などを短いコマンドで実行できます。

---

## 🚀 インストール

```bash
cargo install gzy

# ファイルを追加
gzy add <file>

# コミット
gzy commit "message"

# ブランチ作成
gzy branch dev

# リモートリポジトリ追加
gzy remote https://github.com/user/repo.git

# プッシュ
gzy push main

# 新規ブランチ作成
gzy checkout develop
