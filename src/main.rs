fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    programmers(Option::from(1))?;

    Ok(())
}

fn programmers(page: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://career.programmers.co.kr/api/job_positions";
    let params = [
        ("min_salary", "6000"),
        ("order", "recent"),
        ("page", &page.unwrap_or(1).to_string()),
        ("job_category_ids[]", "1")
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let body = reqwest::blocking::get(url)?.text()?;
    println!("{}", body);
    Ok(())
}