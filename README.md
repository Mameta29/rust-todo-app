# Rust & JavaScript Todo アプリケーション

このプロジェクトは、Rust と JavaScript を使用して構築された完全なフルスタック Todo アプリケーションです。バックエンドに Rust（Actix-web）、フロントエンドに JavaScript（純粋な JS）、データベースに PostgreSQL を使用しています。

## 機能

- Todo タスクの作成、読み取り、更新、削除（CRUD 操作）
- タスクの完了/未完了の切り替え
- シンプルな UI

## 前提条件

プロジェクトを実行する前に、以下がインストールされていることを確認してください：

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Node.js](https://nodejs.org/) (フロントエンドサーバーの実行用)

## 使用技術

- バックエンド：Rust (Actix-web, tokio-postgres)
- フロントエンド：HTML, CSS, JavaScript
- データベース：PostgreSQL

## セットアップ

1. リポジトリをクローンします：

```

git clone https://github.com/yourusername/rust-js-todo-app.git
cd rust-js-todo-app

```

2. PostgreSQL データベースをセットアップします：

   a. PostgreSQL のコマンドラインインターフェースを開きます：

   ```
   psql -U postgres
   ```

   注意: システムによっては、異なるユーザー名やコマンドが必要な場合があります。

   b. 以下の SQL コマンドを実行してデータベースとテーブルを作成します：

   ```sql
   CREATE DATABASE todoapp;
   \c todoapp
   CREATE TABLE todos (
       id SERIAL PRIMARY KEY,
       title TEXT NOT NULL,
       completed BOOLEAN NOT NULL DEFAULT FALSE
   );
   ```

3. バックエンドの環境変数を設定します。`backend`ディレクトリに`.env`ファイルを作成し、以下の内容を追加します：

   ```
   DATABASE_URL=postgres://username:password@localhost/todoapp
   ```

   `username`と`password`を自分の PostgreSQL の認証情報に置き換えてください。

4. バックエンドの依存関係をインストールし、サーバーを起動します：

   ```
   cd backend
   cargo run
   ```

   サーバーは `http://localhost:8080` で起動します。

5. 新しいターミナルウィンドウを開き、フロントエンドサーバーを起動します：

   ```
   cd frontend
   npx http-server -p 3000
   ```

6. ブラウザで `http://localhost:3000` を開きます。

## プロジェクト構造

```
todo-app/
│
├── backend/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── .env
│
└── frontend/
    ├── index.html
    ├── styles.css
    └── app.js
```
