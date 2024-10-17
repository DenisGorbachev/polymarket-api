use crate::{NextCursorRef, Payload, FINAL_NEXT_CURSOR};
use async_stream::stream;
use futures::Stream;
use std::future::Future;

pub fn get_all_stream<T, F: Future<Output = Payload<T>>>(mut f: impl FnMut(&NextCursorRef) -> F) -> impl Stream<Item = Vec<T>> {
    stream! {
        let mut next_cursor = String::new();
        while next_cursor != FINAL_NEXT_CURSOR {
            let payload = f(&next_cursor).await;
            next_cursor = payload.next_cursor;
            yield payload.data;
        }
    }
}

pub async fn get_all_vec<T, F: Future<Output = Payload<T>>>(mut f: impl FnMut(&NextCursorRef) -> F) -> Vec<Vec<T>> {
    let mut output = vec![];
    let mut next_cursor = String::new();
    while next_cursor != FINAL_NEXT_CURSOR {
        let payload = f(&next_cursor).await;
        output.push(payload.data);
        next_cursor = payload.next_cursor;
    }
    output
}
