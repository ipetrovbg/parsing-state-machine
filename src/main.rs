use std::collections::HashMap;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error as DynamoDBError};
use aws_sdk_dynamodb::model::AttributeValue;

use crate::models::{
    ConvertDefinition,
    ConvertOption,
    ConvertStrType,
    HttpDefinition,
    HttpStrType,
    HttpType,
    ParseDefinition,
    ParseStrType,
    ParseType,
    StateMachine,
    Step,
    StepDefinition,
    StrOrNum,
    StrStep,
};

mod models;
mod serializer;
mod utils;

const DB_TABLE: &str = "ParseStep";

#[tokio::main]
async fn main() -> Result<(), DynamoDBError> {
    utils::print_wrap("configuring DynamoDB", '»');

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    utils::print_wrap("requesting steps from DynamoDB", '»');

    let resp = client.scan().table_name(DB_TABLE).send().await?;

    utils::print_wrap("response successful with steps from DynamoDB", '»');

    if let Some(item) = resp.items {
        let mut vec_steps = vec![];
        for i in item {
            vec_steps.push(value_to_item(i));
        }

        utils::print_wrap("parsing started", '»');

        let steps_str = serde_json::to_string_pretty(&vec_steps).unwrap();
        let steps = serializer::run_parse(steps_str.as_str());

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
        };
    }

    Ok(())
}

fn value_to_item(item: HashMap<String, AttributeValue>) -> StrStep {
    let typ = item.keys();
    let mut typ_str = format!("");
    let mut uuid = format!("");
    let mut name = format!("");
    let mut error_on_fail = format!("");
    let mut created_at = format!("");
    let mut next = None;
    let mut http = None;
    let mut parse = None;
    let mut convert = None;
    for key in typ {
        let value = item.get(key);
        match key.as_str() {
            "type" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(t) => {
                                typ_str = t.to_owned();
                            }
                            _ => {}
                        }
                    }
                }
            }
            "uuid" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(string) => {
                                uuid = string.to_owned()
                            }
                            _ => {}
                        }
                    }
                }
            }
            "error_on_fail" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(value) => {
                                error_on_fail = value.to_owned()
                            }
                            _ => {}
                        }
                    }
                }
            }
            "next" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(value) => {
                                match value.is_empty() {
                                    true => {}
                                    false => {
                                        next = Some(value.to_owned())
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            "created_at" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(value) => {
                                created_at = value.to_owned()
                            }
                            _ => {}
                        }
                    }
                }
            }
            "name" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::S(value) => {
                                name = value.to_owned()
                            }
                            _ => {}
                        }
                    }
                }
            }
            "http" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::M(http_) => {
                                let typ = extract_s_or_default(http_.get("type"));
                                let url = extract_s_or_default(http_.get("url"));
                                let body = extract_s_or_default(http_.get("body"));
                                http = match typ.to_lowercase().as_str() {
                                    "get" => {
                                        Some(HttpStrType {
                                            typ,
                                            url,
                                            body: None,
                                        })
                                    }
                                    "post" => {
                                        Some(HttpStrType {
                                            typ,
                                            url,
                                            body: Some(body),
                                        })
                                    }
                                    _ => None
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            "parse" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::M(parse_) => {
                                let typ = extract_s_or_default(parse_.get("type"));
                                let content = extract_s_or_default(parse_.get("content"));
                                parse = Some(ParseStrType {
                                    typ,
                                    content,
                                })
                            }
                            _ => {}
                        }
                    }
                }
            }
            "convert" => {
                match value {
                    None => {}
                    Some(value) => {
                        match value {
                            AttributeValue::M(convert_) => {
                                let typ = extract_s_or_default(convert_.get("type"));
                                let from = extract_s_or_default(convert_.get("from"));
                                let to = extract_s_or_default(convert_.get("to"));
                                let source = convert_.get("source");
                                match source {
                                    None => {}
                                    Some(source) => {
                                        convert = match source {
                                            AttributeValue::N(source) => {
                                                Some(ConvertStrType {
                                                    typ,
                                                    from,
                                                    to,
                                                    source: StrOrNum::Num(source.to_owned().parse::<i32>().unwrap()),
                                                })
                                            }
                                            AttributeValue::Null(_) => None,
                                            AttributeValue::S(source) => {
                                                Some(ConvertStrType {
                                                    typ,
                                                    from,
                                                    to,
                                                    source: StrOrNum::Str(source.to_owned()),
                                                })
                                            }
                                            _ => None
                                        };
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    StrStep {
        typ: typ_str,
        uuid,
        name,
        error_on_fail,
        created_at,
        next,
        http,
        parse,
        convert,
    }
}

fn extract_s_or_default(s: Option<&AttributeValue>) -> String {
    match s {
        None => format!(""),
        Some(s) => {
            match s {
                AttributeValue::S(string) => string.to_owned(),
                _ => format!("")
            }
        }
    }
}
