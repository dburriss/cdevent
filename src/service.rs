use clap::{arg, Arg, ArgMatches};

// ========= Service Deployed =========
pub struct ServiceDeployedArgs {
    pub id: String,
    pub source: String,
    pub subject_id: String,
    pub env_id: String,
    pub env_name: Option<String>,
    pub env_source: Option<String>,
    pub artifact: Option<String>,
}
pub fn deployed_args() -> [Arg; 5] {
    [
        arg!(--subid <SUBJECT_ID> "The unique ID or name of the service").required(true),
        arg!(--envid <ENVIRONMENT_ID> "The unique environment ID").required(true),
        arg!(--envname <ENVIRONMENT_NAME> "The name of the environment eg. prod"),
        arg!(--envsource <ENVIRONMENT_SOURCE> "The source of the environment"),
        arg!(--artifact <ARTIFACT_ID> "Identifier of the artifact deployed with this service"),
    ]
}

pub fn deployed_parse(matches: &ArgMatches) -> ServiceDeployedArgs {
    let id = matches.get_one::<String>("id").unwrap().into();
    let source = matches.get_one::<String>("source").unwrap().into();
    let subject_id = matches.get_one::<String>("subid").unwrap().into();
    let env_id = matches.get_one::<String>("envid").unwrap().into();
    let env_name = matches.try_get_one::<String>("envname").unwrap().cloned();
    let env_source = matches.try_get_one("envsource").unwrap().cloned();
    let artifact = matches.try_get_one("artifact").unwrap().cloned();
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