use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use reqwest::Client;

/// GET /logs/export - export logs to CSV
pub async fn export_logs_csv() -> Result<impl IntoResponse, StatusCode> {
    let client = Client::new();
    
    // Graylog URL API for fetching CSV data
    // "graylog" is the service name in Docker Compose
    let graylog_url = "http://graylog:9000/api/search/messages?fields=timestamp,source,message";
    
    // API call using Basic Auth with admin/admin
    // Header "Accept: text/csv" ask Graylog to return data with CSV format
    // Header "X-Requested-By" is required, according to Graylog documentation 
    let response = client
        .get(graylog_url)
        .basic_auth("admin", Some("admin"))
        .header("Accept", "text/csv")
        .header("X-Requested-By", "rust-app")
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch data as text - CSV
    let csv_data = response.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Configure headers, browser will know it is a file to download
    let headers = [
        (header::CONTENT_TYPE, "text/csv"),
        (header::CONTENT_DISPOSITION, "attachment; filename=\"graylog_logs.csv\""),
    ];

    Ok((headers, csv_data))
}
