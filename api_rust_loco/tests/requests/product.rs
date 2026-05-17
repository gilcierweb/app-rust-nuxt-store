use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_products() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);
        let res = request.get("/api/products/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}
