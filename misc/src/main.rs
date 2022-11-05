use std::pin::Pin;

use futures::{future::join_all, Future, FutureExt};

enum RecArg<'a> {
    Vec(Vec<&'a str>),
    String(&'a str),
}

fn rec_append<'a>(
    x: RecArg<'a>,
    mut init: Vec<&'a str>,
) -> Pin<Box<dyn Future<Output = Vec<&'a str>>>> {
    match x {
        RecArg::Vec(mut xs) => match xs.len() {
            0 => async { init }.boxed_local(),
            1 => rec_append(RecArg::String(xs.pop().unwrap()), init).boxed_local(),
            _ => {
                let x = xs.pop().unwrap();
                rec_append(RecArg::String(x), init)
                    .then(|x_init| rec_append(RecArg::Vec(xs), x_init))
                    .boxed_local()
            }
        },
        RecArg::String(x) => {
            let test = async move {
                init.extend(vec![x]);
                init
            };
            test.boxed_local()
        }
    }
}

#[tokio::main]
async fn main() {
    let chars = vec!["a", "b", "c", "d", "e", "f"];
    let init = vec!["init"];
    let test = rec_append(RecArg::Vec(chars), init).await;
    dbg!(test);
}
