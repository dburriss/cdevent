use clap::{arg, Arg, ArgMatches};

// ========= Service Deployed =========
struct ServiceDeployedArgs {
    subject_id: String,
    env_id: String,
    env_name: Option<String>,
    env_source: Option<String>,
    artifact: Option<String>,
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

pub fn deployed_map(matches: &ArgMatches) {
    let subid = matches.value_of("subid").unwrap();
    let envid = matches.value_of("envid").unwrap();
    let envname = matches.value_of("envname").unwrap_or("default");
    let envsource = matches.value_of("envsource").unwrap_or("default");
    let artifact = matches.value_of("artifact").unwrap();
    println!("Deployed service {} to environment {} with artifact {}", subid, envid, artifact);
}