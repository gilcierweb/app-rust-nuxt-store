use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_addresses() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/addresses/").await;
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
        let res = request.get("/addresses/list").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_add() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/addresses/add").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_update() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/addresses/update").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_remove() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/addresses/remove").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
