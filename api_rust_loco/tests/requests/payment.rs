use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_payments() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/payments/").await;
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
        let res = request.get("/payments/list").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_add() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/payments/add").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_get_one() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/payments/get_one").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_update() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/payments/update").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

