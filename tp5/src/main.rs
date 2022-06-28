use tide::prelude::*;
use tide::{Request, Body};

/// # async / await
/// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html

#[derive(Debug, Deserialize)]
struct Fact {
    name: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Animal {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fact/compute").post(fact_compute);
    app.at("/fact/compute/:n").get(fact_view);
    app.at("/acronyms").get(acronyms);
    app.listen("127.0.0.1:8086").await?;
    Ok(())
}

/// 
/// ````
/// curl -X POST http://127.0.0.1:8086/order/shoes -H "Content-Type: application/json" \
/// -d '{"name": "aled"}' -i
///
async fn fact_compute(mut req: Request<()>) -> tide::Result {
    let Fact { name } = req.body_json().await?;
    Ok(format!("Fact {}", name).into())
}

async fn fact_view(req: Request<()>) -> tide::Result {
    let n: usize = req.param("n")?.parse().unwrap_or(0);
    let Fact { name } = Fact { name: n };
    Ok(format!("Fact {}", name).into())
}

#[derive(Debug, Deserialize)]
struct AcronymQuery {
    animals: Vec<String>
}

///
///  ```sh
/// curl -X GET "http://127.0.0.1:8086/acronyms?animals[]=chien"
/// ```
async fn acronyms(req: Request<()>) -> tide::Result<Body> {
    let n: AcronymQuery = req.query().unwrap();

    let j: Vec<Animal> = n.animals
        .iter()
        .map(|v| Animal{name: v.into()})
        .collect();
    
    Body::from_json(&j)
}
