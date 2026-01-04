# gzy

**gzy** は、日常的な Git 操作をより速く・簡単に行うためのシンプルな CLI ツールです。  
よく使う Git コマンドを、短く直感的な構文で実行できます。

## 対応コマンド
`init`, `add`, `commit`, `branch`, `push`, `remote`, `clone`,  
`checkout`, `status`, `log`, `conflict`, `sync`

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

---

## 使い方

```bash
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
gzy push main
gzy push main -u
gzy push main --force

# クローン
gzy clone https://github.com/user/repo.git

# 新しいブランチを作成してチェックアウト
gzy checkout develop

# 自分の変更を優先
gzy conflict ours

# 相手の変更を優先
gzy conflict theirs

# マージを中断
gzy conflict abort

# upstream/develop を基準に現在のブランチを同期
gzy sync develop

# 同期対象ブランチを明示
gzy sync develop feature/xxx


# ステータスを表示
gzy status

# ログを表示
gzy log
