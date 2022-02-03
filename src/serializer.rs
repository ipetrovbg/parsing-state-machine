use serde::{
    Deserialize, Deserializer,
};
use serde_json;
use crate::{
    ConvertDefinition,
    ConvertOption,
    HttpDefinition,
    HttpType,
    ParseDefinition,
    ParseType,
    Step,
    StepDefinition,
};

impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        const NAME: &str = "name";
        const HTTP: &str = "http";
        const PARSE: &str = "parse";
        const CONVERT: &str = "convert";
        const UUID: &str = "uuid";
        const TYPE: &str = "type";
        const NEXT: &str = "next";
        const ERROR_ON_FAIL: &str = "errorOnFail";
        const CREATED_AT: &str = "createdAt";

        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;
        let typ = json.get(TYPE).expect(TYPE).as_str().unwrap();
        let uuid = json.get(UUID).expect(UUID).as_str().unwrap();
        let name = json.get(NAME).expect(NAME).as_str().unwrap();
        let error_on_fail = json.get(ERROR_ON_FAIL).expect(ERROR_ON_FAIL).as_str().unwrap();
        let next: Option<String> = match json.get(NEXT).expect(NEXT).as_str() {
            None => None,
            Some(some) => {
                Some(some.to_owned())
            }
        };
        let created_at = json.get(CREATED_AT).expect(CREATED_AT).as_str().unwrap();

        let definition = match typ {
            CONVERT => {
                let convert = json.get(CONVERT);
                match convert {
                    None => {
                        return Err(serde::de::Error::custom("missing convert object"));
                    }
                    Some(convert) => {
                        let from = convert.get("from").expect("from").as_str().unwrap();
                        let to = convert.get("to").expect("to").as_str().unwrap();

                        match (from, to) {
                            ("string", "string") => {
                                return Err(serde::de::Error::custom("cannot convert from string to string"));
                            }
                            ("string", "int") => {
                                let source = convert.get("source").expect("source").as_str().unwrap();
                                StepDefinition::Convert(ConvertDefinition {
                                    source: ConvertOption::FromStringToInt(source.to_owned())
                                })
                            }
                            ("int", "string") => {
                                let source = convert.get("source").expect("source").as_i64();
                                match source {
                                    None => {
                                        return Err(serde::de::Error::custom("Couldn't parse [convert –» source] to integer. Maybe you wanted to parse from string to integer?"));
                                    }
                                    Some(source) => {
                                        StepDefinition::Convert(ConvertDefinition {
                                            source: ConvertOption::FromIntToString(source as i32)
                                        })
                                    }
                                }
                            }
                            ("int", "int") => {
                                return Err(serde::de::Error::custom("cannot convert from integer to integer"));
                            }
                            (_, _) => {
                                return Err(serde::de::Error::custom("unknown convert options"));
                            }
                        }
                    }
                }
            }
            HTTP => {
                let http = json.get(HTTP);
                match http {
                    None => {
                        return Err(serde::de::Error::custom("http missing"));
                    }
                    Some(http) => {
                        let url = http.get("url").expect("url").as_str().unwrap();
                        let http_typ = http.get("type").expect("type").as_str().unwrap();
                        match http_typ.to_lowercase().as_str() {
                            "get" => {
                                StepDefinition::Http(HttpDefinition {
                                    http_type: HttpType::Get(url.to_owned())
                                })
                            }
                            "post" => {
                                let body = http.get("body");
                                match body {
                                    None => {
                                        return Err(serde::de::Error::custom("Missing body for post request step."));
                                    }
                                    Some(body) => {
                                        let body = body.as_str().unwrap().to_owned();
                                        StepDefinition::Http(HttpDefinition {
                                            http_type: HttpType::Post(url.to_owned(), body)
                                        })
                                    }
                                }
                            }
                            _ => {
                                return Err(serde::de::Error::custom("Unknown http type. Supported types are GET and POST."));
                            }
                        }
                    }
                }
            }
            PARSE => {
                let parse = json.get(PARSE);
                match parse {
                    None => {
                        return Err(serde::de::Error::custom("invalid parse step"));
                    }
                    Some(parse) => {
                        let type_parse = parse.get("type");
                        let content = parse.get("content");
                        match (type_parse, content) {
                            (Some(type_parse), Some(content)) => {
                                match (type_parse.as_str(), content.as_str()) {
                                    (None, None) => {
                                        return Err(serde::de::Error::custom("parse –» type is not a string and parse –» content is not a string"));
                                    }
                                    (Some(type_parse), Some(content)) => {
                                        match type_parse {
                                            "document" => {
                                                StepDefinition::Parse(ParseDefinition {
                                                    parse_type: ParseType::Document(content.to_owned())
                                                })
                                            }
                                            _ => {
                                                return Err(serde::de::Error::custom("invalid parse type"));
                                            }
                                        }
                                    }
                                    (Some(_), None) => {
                                        return Err(serde::de::Error::custom("parse –» content is not a string"));
                                    }
                                    (None, Some(_)) => {
                                        return Err(serde::de::Error::custom("parse –» type is not a string"));
                                    }
                                }
                            }
                            (None, Some(_)) => {
                                return Err(serde::de::Error::custom("missing parse –» type"));
                            }
                            (Some(_), None) => {
                                return Err(serde::de::Error::custom("missing parse –» content"));
                            }
                            (None, None) => {
                                return Err(serde::de::Error::custom("empty parse object"));
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(serde::de::Error::custom("Unknown step type"));
            }
        };

        Ok(Step {
            next,
            name: name.to_owned(),
            uuid: uuid.to_owned(),
            definition,
            error_on_fail: error_on_fail.to_owned(),
            created_at: created_at.to_owned(),
        })
    }
}

pub fn run_parse(steps_str: &str) -> serde_json::Result<Vec<Step>> {
    serde_json::from_str::<Vec<Step>>(steps_str)
}
