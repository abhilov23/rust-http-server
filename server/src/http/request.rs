use crate::http::request;

use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;

pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method, 
}

impl TryFrom<&[u8]> for Request{
  type Error = ParseError;
  
  
  //GET /search?name=abc&sort=1 HTTP/1.1
  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
     let request =  str::from_utf8(buf)?; //checking for error
      unimplemented!()
  }
}

fn get_next_word(request: &str)-> Option<(&str, &str)>{  
  for (i, c) in request.chars().enumerate(){
  if c == ' '{
    return Some((&request[..i], &request[i + 1..]));

  }
  }
   None

}



pub enum ParseError{
  InvalidRequest,
  InvalidEncoding,
  InvalidMethod,
  InvalidProtocol,
}

impl ParseError {
  fn message(&self) -> &str{
    match self{
      Self::InvalidRequest => "Invalid request",
      Self::InvalidEncoding => "Invalid encoding",
      Self::InvalidMethod => "Invalid method",
      Self::InvalidProtocol => "Invalid protocol",
    }
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}



impl Display for ParseError{
  fn fmt(&self, f: &mut Formatter) -> FmtResult{
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError{
  fn fmt(&self, f: &mut Formatter) -> FmtResult{
    write!(f, "{}", self.message())
  }
}


impl Error for ParseError{

}