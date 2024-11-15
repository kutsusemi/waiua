#[cfg_attr(test, mockall::automock)]
pub trait CommandState<T: Send + Sync + 'static> {
    fn state(&self) -> &T;
}

pub struct CommandStateImpl<'r, T: Send + Sync + 'static> {
    state: tauri::State<'r, T>,
}

impl<'r, T: Send + Sync + 'static> CommandStateImpl<'r, T> {
    pub fn new(state: tauri::State<'r, T>) -> Self {
        CommandStateImpl { state }
    }
}

impl<'r, T: Send + Sync + 'static> CommandState<T> for CommandStateImpl<'r, T> {
    fn state(&self) -> &'r T {
        self.state.inner()
    }
}
