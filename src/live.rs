/*
Person = {
    "name": "String",
    "age": "?Int",
    "address_history": [{
        "line1": "String",
        "line2": "String",
        "postcode": "String",
    }],
}

person = {
    'name': 'John',
    'age': 25,
    'address_history': [
        {
            "line1": "10 Downing Street",
            "line2": "London",
            "postcode": "...",
        },
        {
            ...
        }
    ],
}
*/
use std::collections::HashMap;
enum JsonSchema {
    Property(HashMap<String, JsonSchema>),
    Number,
    String,
    List(Vec<JsonSchema>),
    Null,
}

enum Json {
    Property(HashMap<String, Json>),
    Number(u32),
    String(String),
    List(Vec<Json>),
    Null,
}

fn validate(json: Json, schema: JsonSchema) -> Result<()> {}

fn main() {
    // let schema : JsonSchema = parseJsonSchema(<schema data here>)
    // let data : JsonData = parseJsonData(<json data here>)
    validate(data, schema)
}
