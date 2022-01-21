<!-- cargo-sync-readme start -->
# Rust Arango

> `rust_arango` is a fork of [arangors lite](https://github.com/ManevilleF/arangors) by ManevilleF  and [arangors](https://github.com/fMeow/arangors) by fMeow.

`rust_arango` enables you to connect with ArangoDB server, access to database,
execute AQL query, manage ArangoDB in an easy and intuitive way,
both `async` and plain synchronous code with any HTTP ecosystem you love.

## Philosophy of rust_arango

`rust_arango` is targeted at ergonomic, intuitive and OOP-like API for
ArangoDB, both top level and low level API for users' choice.

Overall architecture of ArangoDB:

> databases -> collections -> documents/edges

In fact, the design of `rust_arango` just mimic this architecture, with a
slight difference that in the top level, there is a connection object on top
of databases, containing a HTTP client with authentication information in
HTTP headers.

Hierarchy of rust_arango:
> connection -> databases -> collections -> documents/edges

## Features

By now, the available features of rust_arango are:

- make connection to ArangoDB
- get list of databases and collections
- fetch database and collection info
- create and delete database or collections
- full featured AQL query
- support both `async` and sync

## Glance

### Use Async or sync code

```toml
[dependencies]
## This one is async
rust_arango = { version = "0.1" }
## This one is synchronous
rust_arango = { version = "0.1", features = ["blocking"] }
```

Thanks to `maybe_async`, `rust_arango` can unify sync and async API and toggle
with a feature gate. rust_arango adopts async first policy.

By default `reqwest` uses OpenSSL. To use `rustls` you may disable default features and use the `rustls` feature:

```toml
[dependencies]
## This one uses openssl
rust_arango = { version = "0.1" }
## This one rustls
rust_arango = { version = "0.1", features = ["rustls"], default-features = false }
```

### Connection

There is three way to establish connections:
- jwt
- basic auth
- no authentication

So are the `rust_arango` API.

Example:

- With authentication

```rust
use rust_arango::Connection;

// (Recommended) Handy functions
let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
    .await
    .unwrap();
let conn = Connection::establish_basic_auth("http://localhost:8529", "username", "password")
    .await
    .unwrap();
```

- Without authentication, only use in evaluation setting

``` rust, ignore
let conn = Connection::establish_without_auth("http://localhost:8529").await.unwrap();
```

### Database && Collection

```rust
use rust_arango::Connection;

let db = conn.db("test_db").await.unwrap();
let collection = db.collection("test_collection").await.unwrap();
```

### AQL Query

All [AQL](https://www.arangodb.com/docs/stable/aql/index.html) query related functions are associated with database, as AQL query
is performed at database level.

There are several way to execute AQL query, and can be categorized into two
classes:

- batch query with cursor
    - `aql_query_batch`
    - `aql_next_batch`

- query to fetch all results
    - `aql_str`
    - `aql_bind_vars`
    - `aql_query`

This later ones provide a convenient high level API, whereas batch
queries offer more control.

#### Typed or Not Typed

Note that results from ArangoDB server, e.x. fetched documents, can be
strong typed given deserializable struct, or arbitrary JSON object with
`serde::Value`.

```rust

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

// Typed
let resp: Vec<User> = db
    .aql_str("FOR u IN test_collection RETURN u")
    .await
    .unwrap();
// Not typed: Arbitrary JSON objects
let resp: Vec<serde_json::Value> = db
    .aql_str("FOR u IN test_collection RETURN u")
    .await
    .unwrap();
```

#### Batch query

`rust_arango` offers a way to manually handle batch query.

Use `aql_query_batch` to get a cursor, and use `aql_next_batch` to fetch
next batch and update cursor with the cursor.

```rust


let aql = AqlQuery::builder()
    .query("FOR u IN @@collection LIMIT 3 RETURN u")
    .bind_var("@collection", "test_collection")
    .batch_size(1)
    .count(true)
    .build();

// fetch the first cursor
let mut cursor = db.aql_query_batch(aql).await.unwrap();
// see metadata in cursor
println!("count: {:?}", cursor.count);
println!("cached: {}", cursor.cached);
let mut results: Vec<serde_json::Value> = Vec::new();
loop {
    if cursor.more {
        let id = cursor.id.unwrap().clone();
        // save data
        results.extend(cursor.result.into_iter());
        // update cursor
        cursor = db.aql_next_batch(id.as_str()).await.unwrap();
    } else {
        break;
    }
}
println!("{:?}", results);
```

#### Fetch All Results

There are three functions for AQL query that fetch all results from
ArangoDB. These functions internally fetch batch results one after another
to get all results.

The functions for fetching all results are listed as bellow:

##### `aql_str`

This function only accept a AQL query string.

Here is an example of strong typed query result with `aql_str`:

```rust

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

let result: Vec<User> = db
    .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
    .await
    .unwrap();
```

##### `aql_bind_vars`

This function can be used to start a AQL query with bind variables.

```rust
use rust_arango::{Connection, Document};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}


let mut vars = HashMap::new();
let user = User {
    username: "test".to_string(),
    password: "test_pwd".to_string(),
};
vars.insert("user", serde_json::value::to_value(&user).unwrap());
let result: Vec<Document<User>> = db
    .aql_bind_vars(r#"FOR i in test_collection FILTER i==@user return i"#, vars)
    .await
    .unwrap();
```

##### `aql_query`

This function offers all the options available to tweak a AQL query.
Users have to construct a `AqlQuery` object first. And `AqlQuery` offer all
the options needed to tweak AQL query. You can set batch size, add bind
vars, limit memory, and all others
options available.

```rust
use rust_arango::{AqlQuery, Connection, Cursor, Database};
use serde_json::value::Value;


let aql = AqlQuery::builder()
    .query("FOR u IN @@collection LIMIT 3 RETURN u")
    .bind_var("@collection", "test_collection")
    .batch_size(1)
    .count(true)
    .build();

let resp: Vec<Value> = db.aql_query(aql).await.unwrap();
println!("{:?}", resp);
```

### Contributing

Contributions and feed back are welcome following Github workflow.

### License

`rust_arango` is provided under the MIT license. See [LICENSE](./LICENSE).
An ergonomic [ArangoDB](https://www.arangodb.com/) client for rust.

<!-- cargo-sync-readme end -->
