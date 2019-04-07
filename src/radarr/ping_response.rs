use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_success_returns_true() {
        let r = PingResponse{ response: String::from("pong") };

        assert!(r.is_success());
    }

    #[test]
    fn is_success_returns_false() {
        let r = PingResponse { response: String::from("err") };

        assert!(! r.is_success());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    #[serde(rename = "Response")]
    pub response: String,
}

impl PingResponse {
    pub fn is_success(&self) -> bool {
        self.response == "pong"
    }
}
