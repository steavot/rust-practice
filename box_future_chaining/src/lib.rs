use futures::{
    future::{ok, Either, FutureResult},
    Future,
};

struct ApiError;
enum MyResponse {
    C,
    Success,
}
enum OtherResponse {
    A,
    B,
    Success,
}

// fn top_level() -> Box<dyn Future<Item = MyResponse, Error = ApiError>> {
//     let x = other();
// .and_then(|rsp: OtherResponse| -> Box<dyn Future<Item = Result<String, MyResponse>, Error = ApiError>> {
//     Box::new(ok(handle_other(rsp)))
// });

// .and_then(|rsp: Result<String, MyResponse>| -> Box<dyn Future<Item = Result<String, MyResponse>, Error = ApiError>> {
//             match rsp {
//                 Err(rsp) => Box::new(ok(Err(rsp))),
//                 Ok(_data) => (),
//             }
//         })
// Box::new(x)
//     x.map(|r| Box::new(other_into_my(r)))
// }

fn other() -> Box<dyn Future<Item = OtherResponse, Error = ApiError>> {
    Box::new(ok(OtherResponse::Success))
}

fn other_into_my(rsp: OtherResponse) -> MyResponse {
    match rsp {
        OtherResponse::A | OtherResponse::B => MyResponse::C,
        OtherResponse::Success => MyResponse::Success,
    }
}

fn other_into_res(rsp: OtherResponse) -> Result<String, MyResponse> {
    match rsp {
        OtherResponse::A | OtherResponse::B => Err(MyResponse::C),
        OtherResponse::Success => Ok("success".to_string()),
    }
}

fn using_either() -> Box<dyn Future<Item = MyResponse, Error = ApiError>> {
    Box::new(
        other()
            .and_then(
                |rsp: OtherResponse| -> FutureResult<Result<String, MyResponse>, ApiError> {
                    ok(other_into_res(rsp))
                },
            )
            .and_then(|rsp: Result<String, MyResponse>| -> Either<_, _> {
                // Item=Result<String, MyResponse>
                // Error=ApiError
                match rsp {
                    Err(r) => Either::A(ok(Err(r))),
                    Ok(_data) => Either::B(other().and_then(|r| ok(other_into_res(r)))),
                }
            })
            .and_then(|rsp: Result<String, MyResponse>| -> Either<_, _> {
                match rsp {
                    Err(r) => Either::A(ok(r)),
                    Ok(_data) => Either::B(other().and_then(|r| ok(other_into_my(r)))),
                }
            }),
    )
}
