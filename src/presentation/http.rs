pub struct HttpRequest<T>
where
    Self: Send,
{
    body: Option<T>,
}

impl<T> HttpRequest<T>
where
    Self: Send,
{
    pub fn new(body: Option<T>) -> Self {
        Self { body }
    }

    /// Get a reference to the http request's body.
    pub fn body(&self) -> Option<&T> {
        self.body.as_ref()
    }
}

pub struct HttpResponse<T>
where
    Self: Send,
{
    status_code: u32,
    body: T,
}

impl<T> HttpResponse<T>
where
    Self: Send,
{
    pub fn new(status_code: u32, body: T) -> Self {
        Self { status_code, body }
    }

    /// Get the http response's status code.
    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    /// Get a reference to the http response's body.
    pub fn body(&self) -> &T {
        &self.body
    }
}
