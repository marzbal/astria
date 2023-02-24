extern crate base64;

use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;

use rs_cnc::error::*;
use rs_cnc::{CelestiaNodeClient, NamespacedDataResponse, PayForDataResponse};

#[tokio::test]
async fn test_data_roundtrip() {
    let base_url = String::from("http://localhost:26659");
    let client = CelestiaNodeClient::new(base_url).unwrap();

    let random_namespace_id = String::from("b860ccf0e97fdf6c");

    // create arbitrary vector of bytes
    let data = Bytes::from(&b"some random data"[..]);

    let res: Result<PayForDataResponse> = client
        .submit_pay_for_data(&random_namespace_id, &data, 2_000, 90_000)
        .await;

    assert!(res.is_ok());

    // use height from previous response to call namespaced data endpoint
    if let Some(height) = res.unwrap().height {
        let res: Result<NamespacedDataResponse> =
            client.namespaced_data(&random_namespace_id, height).await;
        assert!(res.is_ok());

        let namespaced_data_response = res.unwrap();
        // convert base64 encoded value from the response into a vector of bytes
        let res_data = namespaced_data_response.data.unwrap();
        let base64_data = &res_data[0];

        let bytes = general_purpose::STANDARD.decode(base64_data).unwrap();

        assert_eq!(bytes, data);
        assert_eq!(namespaced_data_response.height.unwrap(), height);
    }
}