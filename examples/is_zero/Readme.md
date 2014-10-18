# Extension: is_zero

A simple extension showing how to create a native function that can be called from SQL. It takes a 4-byte integer as in input and returns a boolean.

```sql
select is_zero(0); // true
select is_zero(1); // false
```

## Building

Simply build the library with Cargo:

```bash
cargo build
```

This will generate a new `libis_zero*.dylib` library ready to use in Postgres.

Now you can run `psql` to link against the library.

```sql
CREATE FUNCTION is_zero(int4) RETURNS Boolean AS '/path/to/target/libis_zero-*.dylib' LANGUAGE c;
```
