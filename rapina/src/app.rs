use std::net::SocketAddr;

use crate::router::Router;
use crate::server::serve;
use crate::state::AppState;

pub struct Rapina {
    router: Router,
    state: AppState,
}

impl Rapina {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            state: AppState::new(),
        }
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub fn state<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.state = self.state.with(value);
        self
    }

    pub async fn listen(self, addr: &str) -> std::io::Result<()> {
        let addr: SocketAddr = addr.parse().expect("invalid address");
        serve(self.router, self.state, addr).await
    }
}

impl Default for Rapina {
    fn default() -> Self {
        Self::new()
    }
}
