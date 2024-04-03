//! tests/api/subscriptions_confirm.rs
use crate::helpers::spawn_app;
// use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // let body: serde_json::Value =
    //     serde_json::from_slice(&email_request.body).unwrap();
    // // Extract the link from one of the request fields.
    // let get_link = |s: &str| {
    //     let links: Vec<_> = linkify::LinkFinder::new()
    //         .links(s)
    //         .filter(|l| *l.kind() == linkify::LinkKind::Url)
    //         .collect();
    //     assert_eq!(links.len(), 1);
    //     links[0].as_str().to_owned()
    // };
    // let raw_confirmation_link = &get_link(&body["HtmlBody"].as_str().unwrap());
    // let mut confirmation_link = Url::parse(raw_confirmation_link).unwrap();
    // // Let's make sure we don't call random APIs on the web
    // assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
    // confirmation_link.set_port(Some(app.port)).unwrap();
    // println!("{:?}", confirmation_link);
    // // Act
    // let response = reqwest::get(confirmation_link).await.unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let response =
        reqwest::get(&format!("{}/subscriptions/confirm", app.address))
            .await
            .unwrap();
    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);
    // Act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");
}
