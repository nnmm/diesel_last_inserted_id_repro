# Reproducer for `LAST_INSERT_ID()` prepared statement accumulation

```
docker run --rm -it --name maria -p 3306:3306 -e MARIADB_ROOT_PASSWORD=123 -e MARIADB_DATABASE=my_db mariadb:10.11.5 --performance-schema=ON
```

then

```
DATABASE_URL="mysql://root:123@127.0.0.1:3306/my_db" diesel migration run
DATABASE_URL="mysql://root:123@127.0.0.1:3306/my_db" cargo run
```
