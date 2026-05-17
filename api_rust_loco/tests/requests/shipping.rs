use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_shippings() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);
        let res = request.get("/api/shippings/").await;
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
        let res = request.get("/shippings/list").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_add() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/shippings/add").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_update() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/shippings/update").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_remove() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/shippings/remove").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
