
quick_error! {
    #[derive(Debug)]
    pub enum DIDError {
        FormatError(err: String) {
            display("{}", err)
        }
    }
}

