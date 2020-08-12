use async_std::future::Future;
use async_std::task::{self, JoinHandle};
use futures::future::{abortable, Aborted, AbortHandle};

pub struct AbortWhenDrop(Option<(JoinHandle<Result<(), Aborted>>, AbortHandle)>);
impl AbortWhenDrop {
  pub fn spawn<Fut: Future<Output = ()> + Send + 'static>(f: Fut) -> AbortWhenDrop {
    let (ft, abort) = abortable(f);
    AbortWhenDrop(Some((task::spawn(ft), abort)))
  }

  pub async fn abort(&mut self) {
    if let Some((join, abort)) = self.0.take() {
      abort.abort();
      let _ = join.await;
    }
  }
}
impl Drop for AbortWhenDrop {
  fn drop(&mut self) {
    if let Some((_, abort)) = self.0.take() {
      abort.abort();
    }
  }
}
