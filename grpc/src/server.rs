use  tonic::{transport::Server,Request,Response,Status};
use tokio::sync::mpsc;
use books::{BookRequest,BookResponse};
use books::books_server::{Books,BooksServer};
use std::collections::{HashMap,HashSet};

pub mod books{
    tonic::include_proto!("books");
}
#[derive(Debug,Default)]
pub struct BookService{
}

#[tonic::async_trait]
impl Books for BookService {
    async fn insert(&self,request:Request<BookRequest>)->Result<Response<BookResponse>,Status>{
        println!("Get a request: {:?}",request);
        let req=request.into_inner();
        let reply=BookResponse{
            status: true,
            message: format!("Insert successful")
        };
        Ok(Response::new(reply))
    }
    async fn delete(&self,request:Request<BookRequest>)->Result<Response<BookResponse>,Status>{
        println!("Get a request: {:?}",request);
        let req=request.into_inner();
        let reply=BookResponse{
            status: true,
            message: format!("Delete successful")
        };
        Ok(Response::new(reply))
    }
    async fn select(&self,request:Request<BookRequest>)->Result<Response<BookResponse>,Status>{
        println!("Get a request: {:?}",request);
        let req=request.into_inner();
        let reply=BookResponse{
            status: true,
            message: format!("Select successful")
        };
        Ok(Response::new(reply))
    }
    async fn update(&self,request:Request<BookRequest>)->Result<Response<BookResponse>,Status>{
        println!("Get a request: {:?}",request);
        let req=request.into_inner();
        let reply=BookResponse{
            status: true,
            message: format!("Update successful")
        };
        Ok(Response::new(reply))
    }
}



#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
  let address="0.0.0.0:8080".parse()?;
  let book_services=BookService::default();


  Server::builder()
         .add_service(BooksServer::new(book_services))
         .serve(address)
         .await?;
  Ok(())
}