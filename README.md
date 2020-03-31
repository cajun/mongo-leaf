# Mongo Leaf

## Example:
```rust
#[macro_use]
extern crate bson;
use mongo_leaf::prelude::*;
use std::env;

fn main() -> Result<()> {
  env::set_var("MONGODB_URI","mongodb://standard");
  let builder = Builder::new();
  let pool = builder.connect()?;
  let mut client = pool.pop();

  let db = client.default_database();
  let collection = db.get_collection("test");
  let doc = doc!{"name": "omg"};
  collection.insert_one(doc)?;

  let doc = doc!{"name": "foo"};
  collection.insert_one(doc)?;

  let count = collection.count(None)?;
  assert_eq!(2, count);

  let maybe: Result<Vec<bson::Document>> = collection.find(doc!{"name": "foo"}).collect();

  assert!(maybe.is_ok());
  let records = maybe.unwrap();

  assert_eq!(1, records.len());

  db.destroy(); // Drops the database
  Ok(())
}
  ```


# Testing Instructions

```shell
cargo make up # NOTE: This runs docker-compose up -d
cargo make logs # This runs multi tail on the logs files
cargo make down # NOTE: This runs docker-compose down

docker-compose logs -f lib # This will run for a single service
```
