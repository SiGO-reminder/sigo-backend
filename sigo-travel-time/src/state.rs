// sigo-travel-time/src/state.rs
// 애플리케이션 전역 상태 데이터 AppState를 정의해두는 파일

use reqwest::Client;

pub struct AppState {
    // reqwest::Client는 재사용 가능한 상태를 가지고 있어,
    // 여러 요청에서 사용할 경우 매번 새 클라이언트를 생성하는 것보다 성능이 더 좋습니다.
    // 따라서 클라이언트를 앱이 시작될 때 한 번 생성한 후 상태로 관리하는 것이 좋습니다.
    pub http_client: Client,
}
