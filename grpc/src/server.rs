use books::books_server::{Books, BooksServer};
use books::{
    BookDeleteRequest, BookInfomation, BookInsertRequest, BookResponse, BookSelectRequest,
    BookUpdateRequest,
};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{pool, MySql, MySqlConnection, MySqlPool, Pool};
use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc;
use tonic::metadata::MetadataValue;
use tonic::{transport::Server, Request, Response, Status};
pub mod books {
    tonic::include_proto!("books");
}

//struct store data
// #[derive(Debug, Default)]
// pub struct BooksStore {
//     listbooks: HashMap<i32, BookInfo>,
// }
// impl BooksStore {
//     pub fn new() -> BooksStore {
//         Self { listbooks:HashMap::new() }
//     }
//     pub fn add(&mut self, book: BookInfo) {
//         let maxid = Self::maxKey(&self.listbooks).unwrap();
//         if Some(maxid).is_some() {
//             self.listbooks.insert(*maxid+1, book);
//         } else {
//             self.listbooks.insert(1, book);
//         }
//     }
//     fn maxKey<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
//     where
//         V: Ord,
//     {
//         a_hash_map
//             .iter()
//             .max_by(|a, b| a.1.cmp(&b.1))
//             .map(|(k, _v)| k)
//     }
// }
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookInfo {
    pub name: String,
    pub author: String,
    pub quantity: i32,
    pub description: String,
}
impl BookInfo {
    pub fn new(name: String, author: String, quantity: i32, description: String) -> BookInfo {
        BookInfo {
            name,
            author,
            quantity,
            description,
        }
    }
}

#[derive(Debug)]
pub struct BookService {
    con: Pool<MySql>,
}
impl BookService {
    fn new(con: Pool<MySql>) -> BookService {
        Self { con }
    }
}
#[tonic::async_trait]
impl Books for BookService {
    async fn insert(
        &self,
        request: Request<BookInsertRequest>,
    ) -> Result<Response<BookResponse>, Status> {
        println!("Get a request: {:?}", request);
        let req = request.into_inner();
        let book = req
            .bookinfo
            .map(|f| BookInfo {
                name: f.name,
                author: f.author,
                quantity: f.quantity.parse::<i32>().unwrap(),
                description: f.description,
            })
            .unwrap();
        let sql_query = format!(
            "insert into books (name,author,quantity,description) values ('{}','{}',{},'{}')",
            book.name, book.author, book.quantity, book.description
        );
        let excute = sqlx::query(&sql_query).execute(&self.con).await;
        let mut reply;
        //check insert 
        match excute {
            Ok(_) => {
                reply = BookResponse {
                    status: true,
                    message: format!("Insert successful"),
                };
            },
            Err(err) => {
                reply = BookResponse {
                    status: false,
                    message: err.to_string(),
                };
            },
        }
        
        Ok(Response::new(reply))
    }
    async fn delete(
        &self,
        request: Request<BookDeleteRequest>,
    ) -> Result<Response<BookResponse>, Status> {
        println!("Get a request: {:?}", request);
        let req = request.into_inner();
        let reply = BookResponse {
            status: true,
            message: format!("Delete successful"),
        };
        Ok(Response::new(reply))
    }
    async fn select(
        &self,
        request: Request<BookSelectRequest>,
    ) -> Result<Response<BookResponse>, Status> {
        println!("Get a request: {:?}", request);
        let req = request.into_inner();
        let reply = BookResponse {
            status: true,
            message: format!("Select successful"),
        };
        Ok(Response::new(reply))
    }
    async fn update(
        &self,
        request: Request<BookUpdateRequest>,
    ) -> Result<Response<BookResponse>, Status> {
        println!("Get a request: {:?}", request);
        let req = request.into_inner();
        let reply = BookResponse {
            status: true,
            message: format!("Update successful"),
        };
        Ok(Response::new(reply))
    }
}

fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-auth-token".parse().unwrap();
    match req.metadata().get("authorization") {
        Some(tk) if token == tk => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
    // let token=match req.metadata().get("authorization") {
    //     Some(token)=>token.to_str(),
    //     None => return Err(Status::unauthenticated("Token not found"))
    // };
    // Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //mysql connect
    let database_url = "mysql://root:123456@localhost:3306/testdb"; //std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    //Build service
    let address = "[::1]:50051".parse()?;
    let book_services = BookService::new(pool.clone());
    let svc = BooksServer::with_interceptor(book_services, interceptor);

    Server::builder().add_service(svc).serve(address).await?;
    Ok(())
}
