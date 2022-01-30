use std::error::Error;

use crate::models::{
    ConvertDefinition,
    ConvertOption,
    HttpDefinition, HttpType,
    ParseDefinition,
    ParseType, StateMachine,
    Step, StepDefinition,
};

mod models;

fn main() {
    let step_machine = init_step_machine();
    run(step_machine);
}

fn run(state_machine: StateMachine) {
    println!("\n");
    println!("==================================");
    run_next(Some(&state_machine.start), &state_machine);
    println!("==================================");
}

fn run_next(step: Option<&Step>, state_machine: &StateMachine) {
    match step {
        None => {}
        Some(some) => {
            match run_single(some) {
                Ok(result) => {
                    let next = state_machine.next(some);
                    run_next(next, state_machine);
                }
                Err(_) => {
                    println!("==== STATE MACHINE STOPPED :( ====");
                    println!("> [{}] {}", some.uuid, some.error_on_fail);
                }
            }
        }
    }
}

fn run_single(step: &Step) -> Result<(), Box<dyn Error>> {
    println!("> [Running started] for {}", step.name);
    if step.uuid.eq("step4") {
        return Err("random error".into());
    }
    println!("> Step {} Succeeded - {}", step.name, step.uuid);

    Ok(())
}


fn init_step_machine() -> StateMachine {
    let machine = StateMachine::new(Step {
        next: Some("http_call".to_string()),
        input_ref: None,
        uuid: "step1".to_string(),
        name: "body_parse".to_string(),
        error_on_fail: "Parsing body failed".to_string(),
        created_at: "2022-01-30T16:50:06.490Z".to_string(),
        definition: StepDefinition::Parse(ParseDefinition {
            parse_type: ParseType::Document("<html>
                          <head>
                            <title>Href Attribute Example</title>
                          </head>
                          <body>
                            <h1>Href Attribute Example</h1>
                            </body>
                    </html>"
                .to_string())
        }),
    }).insert(Step {
        next: Some(format!("post_http_call")),
        input_ref: None,
        uuid: "step2".to_string(),
        name: "convert_string_one_to_int".to_string(),
        error_on_fail: "Converting string one to int failed".to_string(),
        created_at: "2022-01-30T16:50:06.490Z".to_string(),
        definition: StepDefinition::Convert(ConvertDefinition {
            source: ConvertOption::FromIntToString(12),
        }),
    })
        .insert(Step {
            uuid: format!("step4"),
            name: format!("post_http_call"),
            error_on_fail: format!("Post request failed"),
            created_at: format!("2022-01-30T16:50:06.490Z"),
            definition: StepDefinition::Http(HttpDefinition {
                http_type: HttpType::Post(
                    format!("https://example.com"),
                    format!("some body"),
                )
            }),
            input_ref: None,
            next: Some(format!("convert_from_string_to_int")),
        })
        .insert(Step {
            uuid: format!("step5"),
            name: format!("convert_from_string_to_int"),
            error_on_fail: format!("Convert from string failed"),
            created_at: format!("2022-01-30T16:50:06.490Z"),
            definition: StepDefinition::Convert(ConvertDefinition {
                source: ConvertOption::FromStringToInt(format!("12"))
            }),
            input_ref: None,
            next: None,
        })
        .insert(Step {
            next: Some(format!("convert_string_one_to_int")),
            input_ref: Some(format!("convert_string_one_to_int")),
            uuid: format!("step3"),
            name: format!("http_call"),
            error_on_fail: format!("Http request failed"),
            created_at: format!("2022-01-30T16:50:06.490Z"),
            definition: StepDefinition::Http(HttpDefinition {
                http_type: HttpType::Get(format!("https://google.com"))
            }),
        });
    machine
}