use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Description {
    publicTime: String,
    publicTimeFormatted: String,
    headlineText: String,
    bodyText: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct WeatherDetail {
    weather: String,
    wind: String,
    wave: String,
}

#[derive(Serialize, Deserialize)]
struct Temperature {
    celsius: u32,
    fahrenheit: u32,
}

#[derive(Serialize, Deserialize)]
struct ChanceOfRain {
    T00_06: u32,
    T06_12: u32,
    T12_18: u32,
    T18_24: u32,
}

#[derive(Serialize, Deserialize)]
struct Image {
    title: String,
    url: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize)]
struct Forecasts {
    date: String,
    dateLabel: String,
    telop: String,
    detail: WeatherDetail,
    temperature: Temperature,
    chanceOfRain: ChanceOfRain,
    image:Image,
}

#[derive(Serialize, Deserialize)]
struct Location {
    area: String,
    prefecture: String,
    district: String,
    city: String,
}

#[derive(Serialize, Deserialize)]
struct Provider {
    link: String,
    name: String,
    note: String,
}

#[derive(Serialize, Deserialize)]
struct Copyright {
    title: String,
    link: String,
    image: Image,
    provider: Provider,
}

#[derive(Serialize, Deserialize)]
struct Weather {
    publicTime: String,
    publicTimeFormatted: String,
    publishingOffice:String,
    title: String,
    link: String,
    description: Description,
    forecasts: Vec<Forecasts>,
    location: Location,
    copyright: Copyright,
}



pub async fn get_weather(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    
    // レスポンスのステータスコードをチェック
    if response.status().is_success() {
        // レスポンスの本文を取得して返す
        let body = response.text().await?;
        Ok(parse_weather_data(&body))
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn parse_weather_data(body: &str) -> String {
    let parsed_data: Result<Weather, _> = serde_json::from_str(body);
    match parsed_data {
        Ok(data) => {
            // data.title
            format!("Perse was successful!:{}", data.title)
        }
        Err(e) => {
            // エラーの場合の処理
            print!("Error:{}",e);
            format!("Parse Failed. ごめんね！\n ({})", e)
        }
    }
}
