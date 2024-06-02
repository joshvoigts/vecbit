use actix_web::{
   http::{
      header::{self, ContentType, TryIntoHeaderValue},
      StatusCode,
   },
   HttpRequest, HttpResponse, Responder,
};

/// An HTML responder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html(pub String);

impl Responder for Html {
   type Body = String;

   fn respond_to(
      self,
      _req: &HttpRequest,
   ) -> HttpResponse<Self::Body> {
      let mut res = HttpResponse::with_body(StatusCode::OK, self.0);
      res.headers_mut().insert(
         header::CONTENT_TYPE,
         ContentType::html().try_into_value().unwrap(),
      );
      res
   }
}
