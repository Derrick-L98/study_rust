use common::constant::MarketToCurrency;
// use futures::{
//   channel::mpsc::{channel, Receiver},
//   SinkExt, StreamExt,
// };
// use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
// use std::path::Path;

/// Async, futures channel based event watching
fn main() {
  //   let path = std::env::args().nth(1).expect("Argument 1 needs to be a path");
  // let path = Path::new("config/default.toml");
  // if path.exists() {
  //   println!("exist..........")
  // }
  // println!("watching {:?}", &path);
  // let marketid = 101;
  // let strret = MarketToCurrency::XSHG.to_string();
  // println!("{}", strret);
  // let comret = common::
  // futures::executor::block_on(async {
  //   if let Err(e) = async_watch(path).await {
  //     println!("error: {:?}", e)
  //   }
  // });
  // loop {}
}

// fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
//   let (mut tx, rx) = channel(1);

//   // Automatically select the best implementation for your platform.
//   // You can also access each implementation directly e.g. INotifyWatcher.
//   let watcher = RecommendedWatcher::new(
//     move |res| {
//       futures::executor::block_on(async {
//         tx.send(res).await.unwrap();
//       })
//     },
//     Config::default(),
//   )?;

//   Ok((watcher, rx))
// }

// async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
//   let (mut watcher, mut rx) = async_watcher()?;

//   // Add a path to be watched. All files and directories at that path and
//   // below will be monitored for changes.
//   watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

//   while let Some(res) = rx.next().await {
//     match res {
//       Ok(event) => println!("changed: {:?}", event),
//       Err(e) => println!("watch error: {:?}", e),
//     }
//   }

//   Ok(())
// }
