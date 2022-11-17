use serde::{Serialize, Deserialize};

// appData: "/config"
// authentication: "none"
// branch: "develop"
// buildTime: "2019-01-10T15:11:37Z"
// isAdmin: false
// isDebug: false
// isLinux: true
// isMono: true
// isMonoRuntime: true
// isOsx: false
// isProduction: true
// isUserInteractive: false
// isWindows: false
// osVersion: "3.13.0.40"
// runtimeVersion: "5.18.1.0 (tarball Fri Mar 15 20:45:47 UTC 2019)"
// sqliteVersion: "3.11.0"
// startupPath: "/opt/radarr"
// urlBase: ""
// version: "0.2.0.1293"

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    #[serde(rename = "appData")]
    pub app_data: String,

    pub authentication: String,
    pub branch: String,

    #[serde(rename = "buildTime")]
    pub build_time: String,

    #[serde(rename = "isAdmin")]
    pub is_admin: bool,

    #[serde(rename = "isDebug")]
    pub is_debug: bool,

    #[serde(rename = "isLinux")]
    pub is_linux: bool,

    #[serde(rename = "isOsx")]
    pub is_osx: bool,

    #[serde(rename = "isProduction")]
    pub is_production: bool,

    #[serde(rename = "isUserInteractive")]
    pub is_user_interactive: bool,

    #[serde(rename = "isWindows")]
    pub is_windows: bool,

    #[serde(rename = "runtimeVersion")]
    pub runtime_version: String,

    #[serde(rename = "startupPath")]
    pub startup_path: String,

    #[serde(rename = "urlBase")]
    pub url_base: String,

    pub version: String,
}
