use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let request_url = format!("https://career.programmers.co.kr/api/job_positions?min_salary=6000&order=recent&page=10&job_category_ids[]=1");
    let response = reqwest::get(&request_url).await?;
    let json = response.json::<serde_json::Value>().await?;
    println!("{}", json["totalPages"]);

    Ok(())
}
