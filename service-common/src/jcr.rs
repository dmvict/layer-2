use actix_web::{HttpResponse, Responder, ResponseError};

pub trait JCR: Sized + Responder + ResponseError + From<anyhow::Error> {
  fn ok(self) -> Result<Self, Self> {
    Ok(self)
  }

  fn err(self) -> Result<Self, Self> {
    Err(self)
  }

  fn success<S: ToString>(msg: S) -> Result<Self, Self>;

  fn error<S: ToString>(err: S) -> Result<Self, Self>;

  fn http(&self) -> HttpResponse;
}

#[macro_export]
macro_rules! jcr_full {
  ($jcr:tt {}) => {
    pub type JCResult = Result<$jcr, $jcr>;

    ::service_common::jcr_trait!($jcr {});

    ::service_common::jcr_todo!($jcr {});

    ::service_common::jcr_actix!($jcr {});

    ::service_common::jcr_display!($jcr {});

    ::service_common::jcr_anyhow!($jcr {});

    ::service_common::jcr_error!($jcr {});
  };
}

#[macro_export]
macro_rules! jcr_trait {
  ($jcr:tt {}) => {
    impl ::service_common::JCR for $jcr {
      fn success<S: ::std::string::ToString>(msg: S) -> Result<Self, Self> {
        $jcr::Success {
          message: msg.to_string(),
        }
        .ok()
      }

      fn error<S: ::std::string::ToString>(err: S) -> Result<Self, Self> {
        $jcr::Error {
          description: vec![err.to_string()],
        }
        .err()
      }

      fn http(&self) -> ::actix_web::HttpResponse {
        match self {
          $jcr::Error { description: _ } => {
            ::actix_web::HttpResponse::InternalServerError().json(self)
          }
          $jcr::TODO => ::actix_web::HttpResponse::InternalServerError().body("Unimplemented"),
          _ => ::actix_web::HttpResponse::Ok().json(self),
        }
      }
    }
  };
}

#[macro_export]
macro_rules! jcr_todo {
  ($jcr:tt {}) => {
    impl $jcr {
      pub fn todo() -> Result<Self, Self> {
        use ::service_common::JCR;
        $jcr::TODO.err()
      }
    }
  };
}

#[macro_export]
macro_rules! jcr_actix {
  ($jcr:tt {}) => {
    impl ::actix_web::Responder for $jcr {
      type Body = ::actix_web::body::BoxBody;
      fn respond_to(
        self,
        _req: &::actix_web::HttpRequest,
      ) -> ::actix_web::HttpResponse<Self::Body> {
        use ::service_common::JCR;
        self.http()
      }
    }

    impl ::actix_web::ResponseError for $jcr {
      fn status_code(&self) -> ::actix_web::http::StatusCode {
        ::actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }

      fn error_response(&self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
        use ::service_common::JCR;
        self.http()
      }
    }
  };
}

#[macro_export]
macro_rules! jcr_display {
  ($jcr:tt {}) => {
    impl ::std::fmt::Display for $jcr {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(&::serde_json::to_string(self).map_err(|_| ::std::fmt::Error)?)
      }
    }
  };
}

#[macro_export]
macro_rules! jcr_anyhow {
  ($jcr:tt {}) => {
    impl From<::anyhow::Error> for $jcr {
      fn from(err: ::anyhow::Error) -> Self {
        $jcr::Error {
          description: err.chain().map(|e| e.to_string()).collect(),
        }
      }
    }
  };
}

#[macro_export]
macro_rules! jcr_error {
  ($jcr:tt {}) => {
    impl $jcr {
      pub fn result(self) -> ::anyhow::Result<Self> {
        if let $jcr::Error { description } = self {
          let mut err_ds = description.into_iter().rev();
          let first = err_ds.next().unwrap_or_default();
          let err = err_ds.fold(::anyhow::anyhow!(first), |acc, e| acc.context(e));
          return Err(err);
        }
        Ok(self)
      }
    }
  };
}
