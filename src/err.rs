
quick_error! {
    #[derive(Debug)]
    pub enum DidError {
        FormatError(err: String) {
            display("{}", err)
        }
    }
}

