pub trait MyState<'r, T: Send + Sync + 'static> {
    fn state(&self) -> &'r T;
}

pub struct MyStateImpl<'r, T: Send + Sync + 'static> {
    state: tauri::State<'r, T>,
}

impl<'r, T: Send + Sync + 'static> MyStateImpl<'r, T> {
    pub fn new(state: tauri::State<'r, T>) -> Self {
        MyStateImpl { state }
    }
}

impl<'r, T: Send + Sync + 'static> MyState<'r, T> for MyStateImpl<'r, T> {
    fn state(&self) -> &'r T {
        self.state.inner()
    }
}

pub mod tests {
    use super::*;

    pub struct MyStateMock<'r, T: Send + Sync + 'static> {
        state: &'r T,
    }
    impl<'r, T: Send + Sync + 'static> MyStateMock<'r, T> {
        pub fn new(state: &'r T) -> Self {
            Self { state }
        }
    }
    impl<'r, T: Send + Sync + 'static> MyState<'r, T> for MyStateMock<'r, T> {
        fn state(&self) -> &'r T {
            &self.state
        }
    }
}
