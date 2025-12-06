// Basic JSON Parsing Example
// Demonstrates simple JSON parsing and data access

use json::*;

// Example 1: Simple object parsing
let simpleJson = "{\"name\": \"Alice\", \"age\": 30, \"city\": \"New York\"}";

match Json.parse(simpleJson) {
  Ok(value) => {
    print("Parsed successfully!");

    // Access string field
    match Json.getString(value, "name") {
      Some(name) => print("Name: " + name),
      None => print("Name not found")
    }

    // Access number field
    match Json.getInt(value, "age") {
      Some(age) => print("Age: " + toString(age)),
      None => print("Age not found")
    }

    // Access another string field
    match Json.getString(value, "city") {
      Some(city) => print("City: " + city),
      None => print("City not found")
    }
  }
  Err(e) => print("Parse error: " + e)
}

// Example 2: Array parsing
let arrayJson = "[1, 2, 3, 4, 5]";

match Json.parse(arrayJson) {
  Ok(value) => {
    match Json.asArray(value) {
      Some(arr) => {
        print("\nArray elements:");
        arr.forEach((item, idx) => {
          match Json.asInt(item) {
            Some(n) => print("  [" + toString(idx) + "] = " + toString(n)),
            None => print("  [" + toString(idx) + "] = (not a number)")
          }
        });
      }
      None => print("Not an array")
    }
  }
  Err(e) => print("Parse error: " + e)
}

// Example 3: Boolean and null handling
let mixedJson = "{\"active\": true, \"deleted\": false, \"data\": null}";

match Json.parse(mixedJson) {
  Ok(value) => {
    print("\nBoolean and null handling:");

    match Json.getBool(value, "active") {
      Some(active) => print("Active: " + toString(active)),
      None => print("Active not found")
    }

    match Json.getBool(value, "deleted") {
      Some(deleted) => print("Deleted: " + toString(deleted)),
      None => print("Deleted not found")
    }

    match Json.get(value, "data") {
      Some(data) => {
        if Json.isNull(data) {
          print("Data is null");
        } else {
          print("Data has a value");
        }
      }
      None => print("Data field not found")
    }
  }
  Err(e) => print("Parse error: " + e)
}
