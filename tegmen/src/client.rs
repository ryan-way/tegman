use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::Region;
use aws_credential_types::provider::ProvideCredentials;
use contract::payload::RequestPayload;
use contract::prelude::*;
use reqwest;
use reqwest::header::HeaderMap;

static SERVICE_ROLE_ARN: &str = "arn:aws:iam::058264156666:role/deploy_service_role";
static TEMPERATURE_API_URL: &str =
    "https://ghsk08xg9b.execute-api.us-west-1.amazonaws.com/prod/temperature";
static REGION: &str = "us-west-1";
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn get_default_headers(body: &str) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();

        let datetime = chrono::Utc::now();
        headers.insert(
            "X-Amz-Date",
            datetime
                .format("%Y%m%dT%H%M%SZ")
                .to_string()
                .parse()
                .unwrap(),
        );

        let credentials_chain = DefaultCredentialsChain::builder()
            .region(Region::new(REGION))
            .build()
            .await;

        let provider = aws_config::sts::AssumeRoleProvider::builder(SERVICE_ROLE_ARN)
            .build_from_provider(credentials_chain)
            .await;

        let credentials = provider
            .provide_credentials()
            .await
            .map_err(|e| e.to_string())?;

        let s = aws_sign_v4::AwsSign::new(
            "POST",
            TEMPERATURE_API_URL,
            &datetime,
            &headers,
            REGION,
            &credentials.access_key_id(),
            &credentials.secret_access_key(),
            "execute-api",
            body,
        );
        let signature = s.sign();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            signature.parse().map_err(|_| "Parsing error".to_owned())?,
        );
        Ok(headers)
    }
}

impl contract::Client<String> for Client {
    async fn list_temperatures(&self) -> Result<Vec<contract::prelude::Temperature>, String> {
        let body = RequestPayload {
            request: Request::ListTemperatures,
            operation: Operation::Query,
        };

        let body_str = serde_json::to_string(&body).map_err(|e| e.to_string())?;
        let headers = Self::get_default_headers(&body_str).await?;

        let res = self
            .client
            .post(TEMPERATURE_API_URL)
            .headers(headers)
            .body(body_str)
            .send()
            .await
            .unwrap();

        println!("Status: {}", res.status());
        let body = res.text().await.unwrap();
        println!("Body:\n\n{}", body);

        let response_payload: Response = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Response::ListTemperatures(temperatures) = response_payload {
            Ok(temperatures)
        } else {
            Err("Response did not contain proper data".to_owned())
        }
    }

    async fn log_temperature(
        &self,
        temperature: contract::prelude::LogTemperature,
    ) -> Result<contract::prelude::Temperature, String> {
        let body = RequestPayload {
            request: Request::LogTemperature(temperature),
            operation: Operation::Mutation,
        };

        let body_str = serde_json::to_string(&body).map_err(|e| e.to_string())?;
        let headers = Self::get_default_headers(&body_str).await?;

        let res = self
            .client
            .post(TEMPERATURE_API_URL)
            .headers(headers)
            .body(body_str)
            .send()
            .await
            .unwrap();

        println!("Status: {}", res.status());
        let body = res.text().await.unwrap();
        println!("Body:\n\n{}", body);

        let response_payload: Response = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Response::LogTemperature(temperature) = response_payload {
            Ok(temperature)
        } else {
            Err("Response did not contain proper data".to_owned())
        }
    }
}
