use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use api_rust_loco::{
    app::App,
    workers::payment_webhook_retry::{Worker, WorkerArgs},
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_payment_webhook_retry_worker() {
    let boot = boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(
        Worker::perform_later(&boot.app_context,WorkerArgs {})
            .await
            .is_ok()
    );
    // Include additional assert validations after the execution of the worker
}
