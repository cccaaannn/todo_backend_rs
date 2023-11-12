# todo_backend_rs

Simple todo backend with rust.

![GitHub top language](https://img.shields.io/github/languages/top/cccaaannn/todo_backend_rs?color=blue) ![GitHub repo size](https://img.shields.io/github/repo-size/cccaaannn/todo_backend_rs?color=orange) [![GitHub](https://img.shields.io/github/license/cccaaannn/todo_backend_rs?color=green)](https://github.com/cccaaannn/todo_backend_rs/blob/master/LICENSE)

---

### Running for Development
1. Install sqlx cli
    ```shell
    cargo install sqlx-cli
    ```
2. Run migration
    ```shell
    sqlx migrate run
    ```
3. Install cargo watch
    ```shell
    cargo install cargo-watch
    ```
4. Run in watch mode
    ```shell
    cargo watch -x run
    ```

### Building
```shell
cargo build
```

### Docker
```shell
docker build -t todo_backend_rs .
```

```shell
docker run -p 8080:8080 --name todo_backend_rs todo_backend_rs
```
