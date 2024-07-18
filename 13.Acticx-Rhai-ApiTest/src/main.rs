// In this project, you will use actix 、rhail
// You can refer the usage in their ofiicial document
use actix_web::{HttpServer,get,App,Responder,web::Path};
use rhai::Engine;

//The main function will be the entrance of HttpServer,
//So, we will create the HttpServer in `main`
#[actix_web::main]
async fn main()-> std::io::Result<()>{

    // 1.Create the HttpServer
    HttpServer::new(|| {
        App::new().service(multiply).service(add)
    })
    .bind(("127.0.0.1",8080))
    .unwrap()
    .run()
    .await
}

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64,i64)>) -> impl Responder {
    
    // 1.get the numbers from the url path
    let (num1,num2) = path.into_inner();

    // 2.create an instance of the rhai engine
    let mut engine = Engine::new(); 

    // 3.register an API that exposes the numbers to Rhai
    engine.register_fn("num1", move|| num1);
    engine.register_fn("num2", move|| num2);

    let result= engine.eval_file::<i64>(("src/multiply.rhai").into()).unwrap();

    format!("{result}!")
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64,i64)>) -> impl Responder{

    // Like multiply
    
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result: i64 = engine.eval_file(("src/add.rhai").into()).unwrap();
    format!("{result}")
}