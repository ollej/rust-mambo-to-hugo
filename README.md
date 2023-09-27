# rust-mambo15-to-hugo

A Rust program to convert a Mambo Open Source 4.0 database to Hugo content.

## Usage

Update the `DATABASE_URL` in `src/main.rs` to the MySQL/MariaDB Mambo 4.0
database. The Hugo content files will be created in the directory `content`.

## Setup MariaDB locally

Some helpful commands to setup a local MariaDB database server using Docker.

### Setup MariaDB docker

```bash
docker run --detach --name mambo-db -p 3306:3306 --env MARIADB_USER=mambo-user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=password mariadb:latest
```

### Create MariaDB database

```bash
docker exec -i mambo-db sh -c 'exec mariadb -uroot -p"password" -e "create database mambodb"'
```

### Import MariaDB database

```bash
docker exec -i mambo-db sh -c 'exec mariadb -uroot -p"password" -D mambodb' < mambo.sql
```

### Connect to MariaDB database

```bash
docker exec -it mambo-db sh -c 'exec mariadb -uroot -p"password" -D mambodb'
```

