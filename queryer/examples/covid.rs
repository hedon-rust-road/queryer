use queryer::query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
     FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 10",
        url
    );

    match query(sql).await {
        Ok(df) => {
            println!("{:?}", df);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);
            Err(e)
        }
    }
}
