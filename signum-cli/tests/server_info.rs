// use std::str::FromStr;

// use signum_cli::sub_commands::server_info::get_my_info::{
//     handle_serverinfo_getmyinfo, GetMyInfoResponse,
// };
// use uuid::Uuid;
// use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

// #[actix_rt::test]
// async fn serverinfo_getmyinfo_works() {
//     // Arrange
//     let app = TestApp::spawn().await;

//     let response_struct_template = GetMyInfoResponse {
//         host: "24.59.137.35".to_string(),
//         address: "24.59.137.35".to_string(),
//         uuid: Uuid::from_str("c9c0e3b0-26eb-4c36-8307-19e05de4110e").unwrap(),
//         request_processing_time: 0,
//     };

//     let response_body_template = serde_json::json!({
//         "host": "24.59.137.35",
//         "address": "24.59.137.35",
//         "UUID": "c9c0e3b0-26eb-4c36-8307-19e05de4110e",
//         "requestProcessingTime": 0,
//     });

//     Mock::given(any())
//         .respond_with(ResponseTemplate::new(200).set_body_json(response_body_template))
//         .named("MOCK: serverInfo::getMyInfo")
//         .expect(1)
//         .mount(&app.node_server)
//         .await;

//     // Act
//     let response = handle_serverinfo_getmyinfo(&app.address).await.unwrap();
//     // let response = reqwest::Client::new()
//     //     .get(&format!("{}/burst?requestType=getPeers", &app.address))
//     //     .send()
//     //     .await
//     //     .expect("Failed to execute request.");

//     // Assert
//     // assert_eq!(response.status().as_u16(), 200);
//     assert_eq!(response_struct_template, response);
// }

// pub struct TestApp {
//     pub node_server: MockServer,
//     pub address: String,
// }
// impl TestApp {
//     pub async fn spawn() -> Self {
//         let node_server = MockServer::start().await;
//         let address = &node_server.uri();
//         TestApp {
//             node_server,
//             address: address.to_string(),
//         }
//     }
// }
