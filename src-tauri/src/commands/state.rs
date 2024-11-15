pub trait CommandState<'r, T: Send + Sync + 'static> {
    fn state(&self) -> &'r T;
}

pub struct CommandStateImpl<'r, T: Send + Sync + 'static> {
    state: tauri::State<'r, T>,
}

impl<'r, T: Send + Sync + 'static> CommandStateImpl<'r, T> {
    pub fn new(state: tauri::State<'r, T>) -> Self {
        CommandStateImpl { state }
    }
}

impl<'r, T: Send + Sync + 'static> CommandState<'r, T> for CommandStateImpl<'r, T> {
    fn state(&self) -> &'r T {
        self.state.inner()
    }
}

pub mod tests {
    use super::*;

    pub struct CommandStateMock<'r, T: Send + Sync + 'static> {
        state: &'r T,
    }
    impl<'r, T: Send + Sync + 'static> CommandStateMock<'r, T> {
        pub fn new(state: &'r T) -> Self {
            Self { state }
        }
    }
    impl<'r, T: Send + Sync + 'static> CommandState<'r, T> for CommandStateMock<'r, T> {
        fn state(&self) -> &'r T {
            &self.state
        }
    }
}
