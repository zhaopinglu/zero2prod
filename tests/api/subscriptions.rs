use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        // We are not setting an expectation here anymore
        // The test is focused on another aspect of the app
        // behaviour.
        .mount(&app.email_server)
        .await;
    // Act
    app.post_subscriptions(body.into()).await;
    // Assert
    // Get the first intercepted request
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);

    // // Parse the body as JSON, starting from raw bytes
    // let body: serde_json::Value =
    //     serde_json::from_slice(&email_request.body).unwrap();
    // println!("body: {}", body);
    // // Extract the link from one of the request fields.
    // let get_link = |s: &str| {
    //     let links: Vec<_> = linkify::LinkFinder::new()
    //         .links(s)
    //         .filter(|l| *l.kind() == linkify::LinkKind::Url)
    //         .collect();
    //     println!("links:{:?}", links);
    //     assert_eq!(links.len(), 1);
    //     links[0].as_str().to_owned()
    // };
    // let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
    // let text_link = get_link(&body["TextBody"].as_str().unwrap());
    // The two links should be identical
    // assert_eq!(html_link, text_link);
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    // Act
    app.post_subscriptions(body.into()).await;
    // Assert
    // Mock asserts on drop
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.post_subscriptions(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    // let saved = sqlx::query!("select email, name from subscriptions",)
    //     .fetch_one(&app.db_pool)
    //     .await
    //     .expect("Failed to fetch saved subscriptions");
    // assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    // assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let app = spawn_app().await;
    let body = "username=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    // Act
    app.post_subscriptions(body.into()).await;
    // Assert
    let saved = sqlx::query!("SELECT email, name , status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("name=le%20guin&email=ursula_le_guin%40gmail.com", "Good"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body.into()).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
