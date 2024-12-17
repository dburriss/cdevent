use std::collections::HashMap;
use std::error::Error;
use cdevents_sdk::{CDEvent, Subject, service_deployed_0_1_1};
use clap::{arg, Arg, ArgMatches};
use serde_json::{to_value};
use crate::args;

// ========= Service Deployed =========
#[derive(Clone)]
pub struct ServiceDeployedArgs {
    pub id: String,
    pub source: String,
    pub subject_id: String,
    pub env_id: String,
    pub env_name: Option<String>,
    pub env_source: Option<String>,
    pub artifact: Option<String>,
    pub custom_data: Option<HashMap<String,String>>
}

impl From<ServiceDeployedArgs> for CDEvent {
    fn from(args: ServiceDeployedArgs) -> Self {
        let mut cd_event = CDEvent::from(
            Subject::from(service_deployed_0_1_1::Content{
                artifact_id: args.artifact.unwrap().try_into().unwrap(),
                environment: (service_deployed_0_1_1::ContentEnvironment{
                    id: args.env_id.try_into().unwrap(),
                    source: args.env_source.map(move |t| {return t.try_into().unwrap()})
                })
            })
                .with_id(args.subject_id.try_into().unwrap())
                .with_source(args.source.clone().try_into().unwrap())
        )
            .with_id(args.id.try_into().unwrap())
            .with_source(args.source.try_into().unwrap());

        if let Some(custom_data) = args.custom_data {
            cd_event = cd_event.with_custom_data(to_value(custom_data).unwrap());
        }

        cd_event
    }
}
pub fn deployed_args() -> [Arg; 6] {
    [
        arg!(--subid <SUBJECT_ID> "The unique ID or name of the service").required(true),
        arg!(--envid <ENVIRONMENT_ID> "The unique environment ID").required(true),
        arg!(--envname <ENVIRONMENT_NAME> "The name of the environment eg. prod"),
        arg!(--envsource <ENVIRONMENT_SOURCE> "The source of the environment"),
        arg!(--artifact <ARTIFACT_ID> "Identifier of the artifact deployed with this service").required(true),
        arg!(--custom <CUSTOM_DATA> "Additional data added to the event").value_parser(parse_custom_data),
    ]
}

fn parse_custom_data(s: &str) -> Result<Vec<(String, String)>, Box<dyn Error + Send + Sync + 'static>>
{
    args::parse_comma_delimited(s, "No custom data provided")
}

pub fn deployed_parse(matches: &ArgMatches) -> ServiceDeployedArgs {
    let id = matches.get_one::<String>("id").unwrap().into();
    let source = matches.get_one::<String>("source").unwrap().into();
    let subject_id = matches.get_one::<String>("subid").unwrap().into();
    let env_id = matches.get_one::<String>("envid").unwrap().into();
    let env_name = matches.try_get_one::<String>("envname").unwrap().cloned();
    let env_source = matches.try_get_one("envsource").unwrap().cloned();
    let artifact = matches.try_get_one("artifact").unwrap().cloned();
    let custom_data:Option<HashMap<String,String>> = matches.try_get_one::<Vec<(String,String)>>("custom")
        .unwrap()
        .map(|c| c.into_iter().map(move |t| { let x = t.clone(); (x.0, x.1)}).collect());
    ServiceDeployedArgs {
        id,
        source,
        subject_id,
        env_id,
        env_name,
        env_source,
        artifact,
        custom_data
    }
}

pub fn to_cloud_event(args: &ServiceDeployedArgs) -> cloudevents::Event {
    let cd_event:CDEvent = CDEvent::from(args.clone());
    cd_event.clone().try_into().unwrap()
}
