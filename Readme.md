# Postgres Extension

Library to write Postgres extensions in Rust! Overall, the Postgres codebase has some pretty narly C macros **everywhere**, so we have to work around them a little bit.


## Getting Started

This library doesn't make any huge assumptions on the version of Postgres that is needed. Each extension specifies a version of postgres that it's compatible with, such as `90500`. Postgres does some compatibility checks on the shared library before using it.

This library is fully compatible with Cargo! Just include the two crates that are needed (one for the actual library and one for macros) and you'll be good to go!

```toml
# Cargo.toml
[package]

name = "is_zero"
version = "0.0.1"
authors = ["Daniel Fagnan <dnfagnan@gmail.com>"]

[lib]
name = "is_zero"
crate-type = ["dylib"]

[dependencies.postgres_extension]
path = "https://github.com/thehydroimpulse/postgres-extension.rs"

[dependencies.postgres_extension_macros]
path = "https://github.com/thehydroimpulse/postgres-extension.rs"
```

## Hello World!

Let's create a simple, "hello world"-like extension.

The first task is to link in the appropriate crates that we need:

```rust
// lib.rs
#![feature(phase)]

extern crate postgres_extension;

#[phase(plugin)]
extern crate postgres_extension_macros;
```

The `phase` feature allows us to specify when to link the specified crate (compile-time? run-time?).

The reason we need two crates is because syntax extensions need to link against `rustc` and `libsyntax`, the Rust compiler and parser (among other things), respectively. These are both fairly big crates and we only have a *compile-time* requirement on them. Meaning, when we run our program (or whatever final output we have) we never ever need access to those compiler crates.

As a result, we'll use the `phase` feature to selectively choose to only link the macro crate during compilation and *not* during runtime.

**Compatibility Checks:**

As I mentioned above, Postgres does compatibility check on the loaded shared library. If these do not match, it won't load. Postgers has it's own `PG_MODULE_MAGIC` *C* macro that handles this automatically. Luckily, we have our own `pg_module!` macro in Rust.

So, continuing from the previous code we wrote:

```rust
// lib.rs
// ...

pg_module!(version: 90500)
```

We're just specifying that this extension is compatible with Postgres 9.5, that's it!

**Exporting Functions:**

Next up is being able to export a function to Postgres. This is done through the `pg_export` attribute that can be placed on any function.

```rust
// lib.rs
// ...

#[pg_export]
pub fn is_zero(a: i32) -> i32 {
  if a == 0 {
    return 1;
  } else {
    return 0;
  }
}
```

**Importing Into Postgres:**

Simply run `psql` (with whatever options you need/want).

```sql
CREATE FUNCTION is_zero(int4) RETURNS Boolean AS '/path/to/target/libis_zero-*.dylib' LANGUAGE c;
```

Replacing the path with the real location to the `dylib`, of course.

Now we can use the extension we just wrote in Rust within a SQL statement. Again in `psql`, we can do:

```sql
select is_zero(1);
```

And you should get something like:

```
postgres=# select is_zero(1);
 is_zero
---------
 f
(1 row)
```

# License

The MIT License (MIT)

Copyright (c) 2014 Daniel Fagnan <dnfagnan@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
