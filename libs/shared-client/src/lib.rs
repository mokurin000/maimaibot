use nyquest_preset::nyquest::{AsyncClient, ClientBuilder};
use tokio::sync::OnceCell;

static NYQUEST_CLIENT: OnceCell<AsyncClient> = OnceCell::const_new();

pub async fn nyquest_client() -> &'static AsyncClient {
    NYQUEST_CLIENT
        .get_or_init(async || ClientBuilder::default().build_async().await.unwrap())
        .await
}
