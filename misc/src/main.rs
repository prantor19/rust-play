use futures::{future::BoxFuture, FutureExt};

enum RecArg<'a> {
    Vec(Vec<&'a str>),
    String(&'a str),
}

fn rec_append<'a>(x: RecArg<'a>, mut init: Vec<&'a str>) -> BoxFuture<'a, Vec<&'a str>> {
    match x {
        RecArg::Vec(mut xs) => match xs.len() {
            0 => async { init }.boxed(),
            1 => rec_append(RecArg::String(xs.pop().unwrap()), init).boxed(),
            _ => {
                let x = xs.pop().unwrap();
                rec_append(RecArg::String(x), init)
                    .then(|x_init| rec_append(RecArg::Vec(xs), x_init))
                    .boxed()
            }
        },
        RecArg::String(x) => {
            let test = async move {
                init.extend(vec![":", x]);
                init
            };
            test.boxed()
        }
    }
}

fn main() {
    let chars = vec!["a", "b", "c", "d", "e", "f"];
    let init = vec!["init"];
    let test = rec_append(RecArg::Vec(chars), init);
    dbg!(futures::executor::block_on(test));
}
