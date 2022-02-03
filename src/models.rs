use std::collections::HashMap;
use std::error::Error;
use crate::utils;

#[derive(Debug, PartialEq, Clone)]
pub enum StepDefinition {
    Convert(ConvertDefinition),
    Parse(ParseDefinition),
    Http(HttpDefinition),
}


#[derive(Debug, PartialEq, Clone)]
pub struct Step {
    pub uuid: String,
    pub name: String,
    pub error_on_fail: String,
    pub created_at: String,
    pub definition: StepDefinition,
    pub next: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseType {
    Document(String),
}

impl PartialEq<ParseType> for ParseDefinition {
    fn eq(&self, other: &ParseType) -> bool {
        self.parse_type == *other
    }
}

impl PartialEq<ParseDefinition> for ParseType {
    fn eq(&self, other: &ParseDefinition) -> bool {
        *self == other.parse_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseDefinition {
    pub parse_type: ParseType,
}

#[derive(Debug, Clone)]
pub enum ConvertOption {
    FromIntToString(i32),
    FromStringToInt(String),
}

impl PartialEq for ConvertOption {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConvertDefinition {
    pub source: ConvertOption,
}

#[derive(Debug, Clone)]
pub enum HttpType {
    Get(String),
    Post(String, String),
}

impl PartialEq for HttpType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HttpDefinition {
    pub http_type: HttpType,
}

#[derive(Debug, PartialEq)]
pub struct StateMachine {
    pub start: Step,
    pub steps: HashMap<String, Step>,
}

impl StateMachine {
    pub fn new(step: Step) -> Self {
        StateMachine {
            start: step,
            steps: HashMap::new(),
        }
    }

    pub fn run(&self) {
        utils::print_wrap("state machine started", '»');
        self.run_next(Some(&self.start));
        utils::print_wrap("end", '»');
    }

    pub fn run_next(&self, step: Option<&Step>) {
        match step {
            None => {}
            Some(some) => match self.run_single(some) {
                Ok(_result) => {
                    let next = self.get_next(some);
                    self.run_next(next);
                }
                Err(_) => {
                    utils::print_wrap("state machine stopped :(", '»');
                    println!("> [{}] {}", some.uuid, some.error_on_fail);
                }
            },
        }
    }

    pub fn run_single(&self, step: &Step) -> Result<(), Box<dyn Error>> {
        println!("[Running started] for {}", step.name);

        // TODO: some compute here based on the step

        println!("> Step {} Succeeded - {}", step.name, step.uuid);

        Ok(())
    }

    pub fn insert_batch(mut self, steps: Vec<Step>) -> Self {
        for step in steps {
            let name = step.name.clone();
            self.steps.insert(name, step);
        }
        self
    }

    pub fn get_next(&self, step: &Step) -> Option<&Step> {
        match step.next.clone() {
            None => None,
            Some(ref_name) => {
                let name = step.name.clone();
                match ref_name.eq(&name) {
                    true => None,
                    false => {
                        let ref_step_option = self.steps.get(&ref_name);
                        match ref_step_option {
                            None => None,
                            Some(ref_step) => Some(ref_step),
                        }
                    }
                }
            }
        }
    }
}
