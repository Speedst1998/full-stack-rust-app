#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;

pub struct MessageApp {
    port: u16,
}

#[derive(Clone, Serialize)]
struct Bike {
    model: String,
    price: i32,
}

#[derive(Clone)]
struct AppState {
    catalog: Arc<Mutex<Vec<Bike>>>,
    owned: Vec<Bike>,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let bike1 = Bike {
            model: "Yamaha".to_string(),
            price: 32000,
        };
        let bike2 = Bike {
            model: "Honda".to_string(),
            price: 24950,
        };
        let bike3 = Bike {
            model: "Suzuki".to_string(),
            price: 3539,
        };

        let catalog_local: Arc<Mutex<Vec<Bike>>> = Arc::new(Mutex::new(Vec::<Bike>::new()));
        catalog_local.lock().unwrap().push(bike1);
        catalog_local.lock().unwrap().push(bike2);
        catalog_local.lock().unwrap().push(bike3);
        let state: AppState = AppState {
            catalog: catalog_local,
            owned: Vec::<Bike>::new(),
        };
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .wrap(middleware::Logger::default())
                .service(index)
                .service(get_catalog)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
        .await
    }
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}
#[get("/")]
async fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    println!("Greeting client");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}

#[derive(Serialize)]
struct GetCatalogResponse {
    available_bikes: Vec<Bike>,
}

#[get("/getCatalog")]
async fn get_catalog(state: web::Data<AppState>) -> Result<web::Json<GetCatalogResponse>> {
    let bikes = &state.catalog;
    println!("Get Catalog Request");
    Ok(web::Json(GetCatalogResponse {
        available_bikes: bikes.lock().unwrap().clone(),
    }))
}
