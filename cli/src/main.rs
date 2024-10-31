use clap_derive::{Parser, Subcommand};
use anyhow::{Context, Result};
use clap::Parser;
use tokio;
use std::env;
use griptape::{apis::{configuration::Configuration, knowledge_bases_api::*, data_connectors_api::*},
    models::{self, CreateDataConnectorRequestContent, DataConnectorConfigInputUnion, Webscraper}};
use env_logger;
use log::{debug, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Query {
        #[arg(long, short)]
        name: String,
        question: String,
    },
    Record {
        #[arg(long, short)]
        name: String,
        #[arg(long, short)]
        url: Vec<String>,
    },
}

trait ConnectorLookup {

    /// Find a DataConnector by name
    fn get_by_name(&self, name: &str) -> Option<models::DataConnectorDetail>;
}

impl ConnectorLookup for Vec<models::DataConnectorDetail> {
    fn get_by_name(&self, name: &str) -> Option<models::DataConnectorDetail> {
        self.iter().find(|c| c.name.to_uppercase() == name.to_uppercase()).cloned()
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let api_key = if let Ok(v) = env::var("GT_CLOUD_API_KEY") {
        v
    } else {
        eprintln!("Missing GT_CLOUD_API_KEY env. variable.");
        std::process::exit(1);
    };

    env_logger::init();

    let cli = Cli::parse();

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match &cli.command {
        Commands::Query { name, question } => {
            let answer = ask(&config, &name, &question).await.context("Couldn't ask the question")?;
            println!("Question: {question}");
            println!("Answer:\n{answer}");
        }
        Commands::Record { name, url } => {
            let _ = record(&config, &name, url.to_vec()).await;
        }
    }

    Ok(())
}

// Stubbed async function for `query`
async fn ask(config: &Configuration, name: &str, question: &str) -> Result<String> {
    let kbs = list_knowledge_bases(&config, None, None).await
        .context("Couldn't list knowledge bases")?
        .knowledge_bases.context("No knowledge bases found")?;
    let kb = kbs.iter().find(|k| k.name.to_uppercase() == name.to_uppercase()).context("Couldn't find knowledge base")?;

    let req = models::SearchKnowledgeBaseRequestContent::new(question.to_string());
    let res = search_knowledge_base(&config, &kb.knowledge_base_id, req).await.context("Couldn't query knowledge base")?;
    let answer = get_knowledge_base_search(&config, &res.knowledge_base_search_id).await.context("Couldn't get answer")?;

    debug!("Answer is {answer:?}");
    
    Ok(answer.result)
}

// Record the given URLs identified by name
async fn record(config: &Configuration, name: &str, url: Vec<String>) -> Result<()> {
    // call to another module would go here
    //FIXME support pagination
    let connectors = list_data_connectors(&config, None, None).await
        .context("Couldn't download data connectors from Griptape Cloud")?
        .data_connectors.context("No data connectors found")?;

    debug!("Found {} connectors", connectors.len());
    let dc = connectors.get_by_name(name);
    let dc_id = if let Some(dc) = dc {
        debug!("Found connector {dc:?} for {name}");
        dc.data_connector_id
    } else {
        debug!("Connector {name} not found. Creating");
        //FIXME support schedule, with default
        let req = CreateDataConnectorRequestContent::new(
            name.to_string(),
            DataConnectorConfigInputUnion::Webscraper(
                Box::new(Webscraper::new(models::WebscraperInput::new(
                            url.to_vec(),
                ))),
            ),
            "webscraper".to_string(),
        );
        //FIXME fails with "missing field `data_job_id` - raise bug
        let res = create_data_connector(&config, req).await.context("Couldn't create Data Connector")?;
        res.data_connector_id
    };

    //FIXME support pagination
    let kbs = list_knowledge_bases(&config, None, None).await
        .context("Couldn't list knowledge bases")?
        .knowledge_bases.context("No knowledge bases found")?;
    let kb = kbs.iter().find(|k| k.name.to_uppercase() == name.to_uppercase()).context("Couldn't find knowledge base").ok();

    if let Some(kb) = kb {
        debug!("Knowledge base exists {}", kb.knowledge_base_id)
    } else {
        debug!("Creating knowledge base {name}");
        let req = models::CreateKnowledgeBaseRequestContent::new(
            name.to_string(),
            vec![dc_id]
        );
        let res = create_knowledge_base(&config, req).await.context("Couldn't create knowledge base")?;
        debug!("Created knowlege base {name} {}", res.knowledge_base_id);
    }
    Ok(())
}

