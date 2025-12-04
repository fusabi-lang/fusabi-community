// JSON Combinators for Fusabi
// Provides utilities for working with JSON data using the stdlib Json module

// ============================================================================
// PARSING HELPERS
// ============================================================================

let parseOrDefault default str =
    match Json.parse str with
    | Some value -> value
    | None -> default

let parseOrEmpty str =
    parseOrDefault (Json.object []) str

// ============================================================================
// PATH TRAVERSAL
// ============================================================================

let get path value =
    let segments = String.split "." path
    let rec traverse segs current =
        match segs with
        | [] -> Some current
        | seg :: rest ->
            match String.toInt seg with
            | Some idx ->
                match Json.toArray current with
                | Some arr -> 
                    match List.nth idx arr with
                    | Some item -> traverse rest item
                    | None -> None
                | None -> None
            | None ->
                match Json.get seg current with
                | Some next -> traverse rest next
                | None -> None
    in
    traverse segments value

let getOr default path value =
    match get path value with
    | Some v -> v
    | None -> default

// ============================================================================
// TYPE COERCION
// ============================================================================

let asString value =
    Json.toString value

let asStringOr default value =
    match Json.toString value with
    | Some s -> s
    | None -> default

let asInt value =
    Json.toInt value

let asIntOr default value =
    match Json.toInt value with
    | Some n -> n
    | None -> default

let asFloat value =
    Json.toFloat value

let asFloatOr default value =
    match Json.toFloat value with
    | Some f -> f
    | None -> default

let asBool value =
    Json.toBool value

let asBoolOr default value =
    match Json.toBool value with
    | Some b -> b
    | None -> default

let asArray value =
    Json.toArray value

let asArrayOr default value =
    match Json.toArray value with
    | Some arr -> arr
    | None -> default

let asObject value =
    Json.toObject value

let asObjectOr default value =
    match Json.toObject value with
    | Some obj -> obj
    | None -> default

// ============================================================================
// OBJECT ACCESSORS (legacy, uses path traversal)
// ============================================================================

let getField key obj =
    get key obj

let getFieldOr default key obj =
    getOr default key obj

let getString key obj =
    match get key obj with
    | Some value -> asString value
    | None -> None

let getStringOr default key obj =
    match getString key obj with
    | Some s -> s
    | None -> default

let getInt key obj =
    match get key obj with
    | Some value -> asInt value
    | None -> None

let getIntOr default key obj =
    match getInt key obj with
    | Some n -> n
    | None -> default

let getBool key obj =
    match get key obj with
    | Some value -> asBool value
    | None -> None

let getBoolOr default key obj =
    match getBool key obj with
    | Some b -> b
    | None -> default

let getArray key obj =
    match get key obj with
    | Some value -> asArray value
    | None -> None

let getArrayOr default key obj =
    match getArray key obj with
    | Some arr -> arr
    | None -> default

// ============================================================================
// OBJECT BUILDERS
// ============================================================================

let empty () =
    Json.object []

let singleton key value =
    Json.object [(key, value)]

let merge obj1 obj2 =
    match (Json.toObject obj1, Json.toObject obj2) with
    | (Some pairs1, Some pairs2) -> Json.object (List.concat [pairs1, pairs2])
    | (Some _, None) -> obj1
    | (None, Some _) -> obj2
    | (None, None) -> empty ()

let withField key value obj =
    merge obj (singleton key value)

let withString key str obj =
    withField key (Json.string str) obj

let withInt key n obj =
    withField key (Json.int n) obj

let withBool key b obj =
    withField key (Json.bool b) obj

let withArray key arr obj =
    withField key (Json.array arr) obj

// ============================================================================
// ARRAY HELPERS
// ============================================================================

let mapArray fn jsonArray =
    match Json.toArray jsonArray with
    | Some arr -> Json.array (List.map fn arr)
    | None -> Json.array []

let filterArray pred jsonArray =
    match Json.toArray jsonArray with
    | Some arr -> Json.array (List.filter pred arr)
    | None -> Json.array []

let findInArray pred jsonArray =
    match Json.toArray jsonArray with
    | Some arr -> List.find pred arr
    | None -> None

// ============================================================================
// PRETTY PRINTING
// ============================================================================

let pretty json =
    Json.stringify json

let compact json =
    Json.stringify json
