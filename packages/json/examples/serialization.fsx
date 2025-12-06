// JSON Serialization Example
// Demonstrates creating and serializing JSON structures

use json::*;

// Example 1: Create and serialize a simple object
print("Example 1: Simple Object Serialization");
let person = Json.object([
  ("name", Json.string("Bob")),
  ("age", Json.number(25)),
  ("email", Json.string("bob@example.com")),
  ("verified", Json.bool(true))
]);

let personJson = Json.stringify(person);
print(personJson);
print("");

// Example 2: Nested objects
print("Example 2: Nested Object Serialization");
let user = Json.object([
  ("username", Json.string("johndoe")),
  ("profile", Json.object([
    ("firstName", Json.string("John")),
    ("lastName", Json.string("Doe")),
    ("age", Json.number(28))
  ])),
  ("settings", Json.object([
    ("theme", Json.string("dark")),
    ("notifications", Json.bool(true))
  ]))
]);

let userJson = Json.prettyPrint(user, 2);
print(userJson);
print("");

// Example 3: Arrays
print("Example 3: Array Serialization");
let numbers = Json.array([
  Json.number(1),
  Json.number(2),
  Json.number(3),
  Json.number(4),
  Json.number(5)
]);

print("Numbers: " + Json.stringify(numbers));

let fruits = Json.array([
  Json.string("apple"),
  Json.string("banana"),
  Json.string("cherry")
]);

print("Fruits: " + Json.stringify(fruits));
print("");

// Example 4: Mixed array with objects
print("Example 4: Array of Objects");
let employees = Json.array([
  Json.object([
    ("id", Json.number(1)),
    ("name", Json.string("Alice")),
    ("role", Json.string("Developer"))
  ]),
  Json.object([
    ("id", Json.number(2)),
    ("name", Json.string("Bob")),
    ("role", Json.string("Designer"))
  ]),
  Json.object([
    ("id", Json.number(3)),
    ("name", Json.string("Charlie")),
    ("role", Json.string("Manager"))
  ])
]);

let employeesJson = Json.prettyPrint(employees, 2);
print(employeesJson);
print("");

// Example 5: Null values
print("Example 5: Handling Null Values");
let config = Json.object([
  ("apiKey", Json.string("secret123")),
  ("timeout", Json.number(5000)),
  ("proxy", Json.null()),
  ("retries", Json.number(3)),
  ("customHeader", Json.null())
]);

print(Json.prettyPrint(config, 2));
