use actix_web::{HttpRequest, HttpResponse, body::BoxBody, Responder};
use serde::Serialize;



#[derive(Serialize)]
pub enum MyResponse<S,E,IncorrectAddress>{
    Success(S),
    Error(E),
    IncorrectAddr(IncorrectAddress)
}

impl <E,S,IncorrectAddress> Responder for MyResponse<E,S,IncorrectAddress>
where 
    IncorrectAddress: Serialize,
    E: Serialize,
    S: Serialize {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            match self {
                MyResponse::Success(success) => HttpResponse::Ok().json(success),
                MyResponse::Error(error) => HttpResponse::ExpectationFailed().json(error),
                MyResponse::IncorrectAddr(i) => HttpResponse::BadRequest().json(i) 
            }
        }
    }
