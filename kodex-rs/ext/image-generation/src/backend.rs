use http::HeaderMap;
use http::HeaderValue;
use kodex_api::ImageEditRequest;
use kodex_api::ImageGenerationRequest;
use kodex_api::ImageResponse;
use kodex_api::ImagesClient;
use kodex_api::ReqwestTransport;
use kodex_api::map_api_error;
use kodex_login::default_client::add_originator_header;
use kodex_login::default_client::create_client;
use kodex_model_provider::SharedModelProvider;
use kodex_protocol::error::KodexErr;

const X_KODEX_IMAGE_TURN_ID_HEADER: &str = "x-kodex-image-turn-id";

pub(crate) struct ImageBackendError {
    message: String,
    kodex_error: KodexErr,
}

impl ImageBackendError {
    fn from_api(error: kodex_api::ApiError) -> Self {
        let message = error.to_string();
        Self {
            message,
            kodex_error: map_api_error(error),
        }
    }

    fn from_message(message: String) -> Self {
        Self {
            kodex_error: KodexErr::Stream(message.clone()),
            message,
        }
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn kodex_error(&self) -> &KodexErr {
        &self.kodex_error
    }
}

#[derive(Clone)]
pub(crate) struct KodexImagesBackend {
    provider: SharedModelProvider,
    originator: Option<String>,
}

impl KodexImagesBackend {
    /// Creates a backend that sends image requests through the active model provider.
    pub(crate) fn new(provider: SharedModelProvider, originator: Option<String>) -> Self {
        Self {
            provider,
            originator,
        }
    }

    /// Resolves the provider and auth required for the current image API request.
    async fn client(&self) -> Result<ImagesClient<ReqwestTransport>, ImageBackendError> {
        let provider = self
            .provider
            .api_provider()
            .await
            .map_err(|err| ImageBackendError::from_message(err.to_string()))?;
        let auth = self
            .provider
            .api_auth()
            .await
            .map_err(|err| ImageBackendError::from_message(err.to_string()))?;
        Ok(ImagesClient::new(
            ReqwestTransport::from_http_client(create_client()),
            provider,
            auth,
        ))
    }

    /// Sends a standalone image generation request through the configured Images client.
    pub(crate) async fn generate(
        &self,
        request: ImageGenerationRequest,
        turn_id: &str,
    ) -> Result<(ImageResponse, Option<String>), ImageBackendError> {
        self.client()
            .await?
            .generate(
                &request,
                image_request_headers(self.originator.as_deref(), turn_id),
            )
            .await
            .map_err(ImageBackendError::from_api)
    }

    /// Sends a standalone image edit request through the configured Images client.
    pub(crate) async fn edit(
        &self,
        request: ImageEditRequest,
        turn_id: &str,
    ) -> Result<(ImageResponse, Option<String>), ImageBackendError> {
        self.client()
            .await?
            .edit(
                &request,
                image_request_headers(self.originator.as_deref(), turn_id),
            )
            .await
            .map_err(ImageBackendError::from_api)
    }
}

fn image_request_headers(originator: Option<&str>, turn_id: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Ok(turn_id) = HeaderValue::from_str(turn_id) {
        headers.insert(X_KODEX_IMAGE_TURN_ID_HEADER, turn_id);
    }
    if let Some(originator) = originator {
        add_originator_header(&mut headers, originator);
    }
    headers
}
