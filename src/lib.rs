pub mod database;
pub mod models;
pub mod schema;
pub mod seed;
pub mod utils;

// #[cfg(test)]
// mod tests {
//     use actix_web::body;

//     use crate::models;

//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn project_data() {
//         // make a request to the server
//         // assert that the response is correct

//         let url = "http://localhost:8080/project?project_id=0xc290dd8e51ac35480d9872ce4484aac23bb812c47c0567bfd4beb9113726ed11&data=true";

//         let response = reqwest::blocking::get(url).unwrap();

//         assert_eq!(response.status(), 200);

//         let res: models::ProjectResponseData = response.json().unwrap();

//         assert_eq!(
//             res.data.unwrap().projectId,
//             "0xc290dd8e51ac35480d9872ce4484aac23bb812c47c0567bfd4beb9113726ed11"
//         );
//     }
// }
