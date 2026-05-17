use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_carts() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);
        let res = request.get("/api/carts/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_list() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/carts/list").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_add_item() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/carts/add_item").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_remove_item() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/carts/remove_item").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_update_item() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/carts/update_item").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_clear() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/carts/clear").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
