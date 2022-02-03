use crate::models::{
    ConvertDefinition,
    ConvertOption,
    HttpDefinition,
    HttpType,
    ParseDefinition,
    ParseType,
    StateMachine,
    Step,
    StepDefinition,
};

mod models;
mod serializer;
mod utils;

fn main() {
    let incoming_str_steps = STEPS;
    utils::print_wrap("parsing started", '»');
    let steps = serializer::run_parse(incoming_str_steps);

    utils::print_wrap("Parsing finished successfully", '»');

    match steps {
        Ok(steps) => {
            if !steps.is_empty() {
                let first = steps.first().unwrap().to_owned();

                StateMachine::new(first)
                    .insert_batch(steps)
                    .run();
            }
        }
        Err(e) => println!("{}", e)
    }
}


const STEPS: &str = r#"
    [{
      "uuid": "a204db2d-3c36-43ed-98d1-7700a3ad622a",
      "errorOnFail": "Custom error for http step",
      "name": "http_get_google",
      "next": "http_post_to_google",
      "type": "http",
      "http": {
        "type": "GET",
        "url": "https://google.com"
      },
      "createdAt": "2022-01-30T14:47:25.869Z"
    },
    {
      "uuid": "a204db2d-3c36-43ed-98d1-7700a3ad622a",
      "errorOnFail": "Custom error for http post step",
      "name": "http_post_to_google",
      "next": "parse_document",
      "type": "http",
      "http": {
        "type": "POSt",
        "url": "https://google.com",
        "body": "some body string"
      },
      "createdAt": "2022-01-30T14:47:25.869Z"
    },
    {
      "uuid": "a204db2d-3c36-43ed-98d1-7700a3ad622a",
      "errorOnFail": "Parsing document body failed",
      "name": "parse_document",
      "next": "convert_from_string_to_int",
      "unknownFive": 5,
      "parse": {
        "type": "document",
        "content": "<html><head><title>Href Attribute Example</title></head><body><h1>Href Attribute Example</h1></body></html>"
      },
      "type": "parse",
      "createdAt": "2022-01-30T14:47:25.869Z"
    },
    {
      "uuid": "2639f3c3-9e49-4802-92e3-7b8a68c25e4d",
      "next": "convert_from_int_to_string",
      "createdAt": "2022-01-30T14:47:25.869Z",
      "convert": {
        "from": "string",
        "to": "int",
        "source": "6"
      },
      "name": "convert_from_string_to_int",
      "errorOnFail": "Failed to convert from string to integer",
      "type": "convert"
    },
    {
      "uuid": "2639f3c3-9e49-4802-92e3-7b8a68c25e4d",
      "next": "",
      "createdAt": "2022-01-30T14:47:25.869Z",
      "convert": {
        "from": "int",
        "to": "string",
        "source": 123
      },
      "name": "convert_from_int_to_string",
      "errorOnFail": "Failed to convert from integer to string",
      "type": "convert"
    }
    ]
    "#;
