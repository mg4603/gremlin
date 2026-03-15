use tokio::task::JoinHandle;

use gremlin_core::generator::JobGenerator;
use gremlin_core::queue::TaskSender;
use gremlin_core::request::ScanRequest;

pub async fn run_generator(
    mut generator: JobGenerator,
    sender: TaskSender<ScanRequest>,
    mut shutdown: JoinHandle<()>,
) {
    loop {
        tokio::select! {
            _ = &mut shutdown => {
                println!("shutdown signal received");
                break;
            }

            job = generator.next() => {
                match job {
                    Ok(Some(request)) => {
                        if sender.send(request).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("generator error: {e}");
                        break;
                    }
                }
            }
        }
    }

    drop(sender);
}
