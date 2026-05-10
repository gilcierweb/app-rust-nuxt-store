use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_orders() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/orders/").await;
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
        let res = request.get("/orders/list").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_get_one() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/orders/get_one").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

