use std::error::Error;
use cdevents_sdk::{CDEvent, Subject, service_deployed_0_1_1};
use clap::{arg, Arg, ArgMatches};
use cloudevents::event::EventBuilderV10;
use cdevents_sdk::cloudevents::BuilderExt;
use clap::builder::Str;

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
}

impl From<ServiceDeployedArgs> for CDEvent {
    fn from(args: ServiceDeployedArgs) -> Self {
        CDEvent::from(
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
            .with_source(args.source.try_into().unwrap())
    }
}
pub fn deployed_args() -> [Arg; 6] {
    [
        arg!(--subid <SUBJECT_ID> "The unique ID or name of the service").required(true),
        arg!(--envid <ENVIRONMENT_ID> "The unique environment ID").required(true),
        arg!(--envname <ENVIRONMENT_NAME> "The name of the environment eg. prod"),
        arg!(--envsource <ENVIRONMENT_SOURCE> "The source of the environment"),
        arg!(--artifact <ARTIFACT_ID> "Identifier of the artifact deployed with this service").required(true),
        arg!(--custom <CUSTOM_DATA> "Additional data added to the event").value_parser(parse_key_val::<String,String>)
    ]
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}


pub fn deployed_parse(matches: &ArgMatches) -> ServiceDeployedArgs {
    let id = matches.get_one::<String>("id").unwrap().into();
    let source = matches.get_one::<String>("source").unwrap().into();
    let subject_id = matches.get_one::<String>("subid").unwrap().into();
    let env_id = matches.get_one::<String>("envid").unwrap().into();
    let env_name = matches.try_get_one::<String>("envname").unwrap().cloned();
    let env_source = matches.try_get_one("envsource").unwrap().cloned();
    let artifact = matches.try_get_one("artifact").unwrap().cloned();
    let custom:Vec<(String,String)> = matches.get_many::<(String,String)>("custom").into_iter().flatten().map(move |t| { let x = t.clone(); (x.0, x.1)}).collect();
    ServiceDeployedArgs {
        id,
        source,
        subject_id,
        env_id,
        env_name,
        env_source,
        artifact
    }
}

pub fn to_cloud_event(args: &ServiceDeployedArgs) -> cloudevents::Event {
    let cd_event:CDEvent = CDEvent::from(args.clone());
    cd_event.clone().try_into().unwrap()
}