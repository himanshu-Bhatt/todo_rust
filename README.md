
# TODO(Rust)

 A todo api writtern in rust using sqlx and postgresql.
 


### Run Locally(docker)

Clone the project

```bash
  git clone https://github.com/himanshu-Bhatt/todo_rust.git
```

Go to the project directory

```bash
  cd todo_rust
```

Install dependencies

```bash
  cargo build
```

Start the database

```bash
  sudo sh src/db/create_db.sh
```
Create table

```bash
  sudo sh src/db/create_table.sh
```

Insert mock data

```bash
  sudo sh src/db/insert_mock_data.sh
```
Start Server

```bash
  Cargo run
```
