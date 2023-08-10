use books::books_client::BooksClient;
use books::BookRequest;

pub mod books {
    tonic::include_proto!("books");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BooksClient::connect("http://127.0.0.1:8080").await?;

    let req = tonic::Request::new(BookRequest {
        name: "math".to_string(),
        author: "huyxu".to_string(),
        quantity: "5".to_string(),
        description: "test".to_string(),
    });
   let res=client.insert(req).await?;

   println!("Response={:?}",res);
   Ok(())
}
