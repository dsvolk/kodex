//! Triggers local rollout maintenance without waiting for the background pass.

use super::ThreadRequestProcessor;
use super::thread_processor::unsupported_thread_store_operation;
use kodex_app_server_protocol::ClientResponsePayload;
use kodex_app_server_protocol::JSONRPCErrorError;
use kodex_app_server_protocol::RolloutCompressResponse;
use kodex_thread_store::LocalThreadStore;

impl ThreadRequestProcessor {
    pub(crate) fn rollout_compress(
        &self,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        if !self.thread_store.as_any().is::<LocalThreadStore>() {
            return Err(unsupported_thread_store_operation("rollout/compress"));
        }

        kodex_rollout::spawn_rollout_compression_worker(
            self.config.kodex_home.to_path_buf(),
            kodex_rollout::RolloutCompressionTrigger::Rpc,
        );
        Ok(Some(RolloutCompressResponse {}.into()))
    }
}
