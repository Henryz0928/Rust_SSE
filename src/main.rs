use futures::stream::StreamExt;
use tokio::time::{self, Duration};
use warp::sse::Event;
use warp::Filter;


async fn sse_events() -> impl warp::Reply {
    // 实现事件流逻辑
    let mut interval = time::interval(Duration::from_secs(1));
    futures::stream::unfold((), move |_| async move {
        interval.tick().await;
        Some((Event::default().data("hello"), ()))
    })
}

#[tokio::main]
async fn main() {
    let sse_route = warp::path("events")
        .and(warp::get())
        .map(|| {
            let stream = sse_events();   //调用异步函数获取事件流
            warp::sse::reply(warp::sse::keep_alive().stream(stream))
        });
    warp::serve(sse_route).run(((127, 0, 0, 1), 3030)).await;
}
