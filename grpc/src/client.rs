use books::books_client::BooksClient;
use books::{BookInfomation, BookInsertRequest};
use tonic::Request;
use tonic::{metadata::MetadataValue};
use tonic::transport::Channel;

pub mod books {
    tonic::include_proto!("books");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let bookinfo= BookInfomation {
        name: "math".to_string(),
        author: "huyxu".to_string(),
        quantity: "5".to_string(),
        description: "test".to_string(),
    };
    let token: MetadataValue<_>="Bearer some-auth-token".parse()?;
    let mut client=BooksClient::with_interceptor(channel,move|mut req:Request<()>|{
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    let req = tonic::Request::new(BookInsertRequest {
        bookinfo:Some(bookinfo),
    });
    let res = client.insert(req).await?;

    println!("Response={:?}", res);
    Ok(())
}
