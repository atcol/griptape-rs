use griptape::apis::configuration::{ApiKey, Configuration};
use griptape::models::{self, Webscraper};
use griptape::models::create_data_connector_request_content::CreateDataConnectorRequestContent;
use griptape::apis::data_connectors_api::*;
use griptape::models::DataConnectorConfigInputUnion;


#[tokio::main]
pub async fn main() {
    let resp = 
        reqwest::get("https://en.wikipedia.org/wiki/Special:Random")
        .await.unwrap();

    let url = resp.url();
    println!("Random wikipedia article  = {url:?}");

    let req = CreateDataConnectorRequestContent::new(
        format!("Wikipedia - {}", url.to_string()), 
        DataConnectorConfigInputUnion::Webscraper(
            Box::new(Webscraper::new(models::WebscraperInput::new(
                        vec![url.to_string()],
                        ))),
        ),
        "webscraper".to_string(),
    );
    let mut config = Configuration::new();
    config.bearer_access_token = Some(std::env::var("GRIPTAPE_API_TOKEN").unwrap());

    let connector_resp = create_data_connector(&config, req).await;
    println!("Connector = {connector_resp:?}");
}
