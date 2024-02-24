#![allow(dead_code)]
use std::{
    convert::Infallible,
    fmt::Debug,
    future::{poll_fn, IntoFuture},
    io,
    marker::PhantomData,
    task::{Context, Poll},
};


use tokio::io::{Stdin, Stdout, AsyncBufReadExt};
use tower::Service;

// TODO!
pub struct Request;
pub struct Response;
pub fn serve<M, S>(stdin: Stdin, stdout: Stdout, make_service: M) -> Serve<M, S>
where
    M: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S>,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
        Serve {
        stdin,
        stdout,
        make_service,
        _marker: PhantomData,
    }
}

pub struct Serve<M, S> {
    stdin: Stdin,
    stdout: Stdout,
    make_service: M,
    _marker: PhantomData<S>,
}
impl<M, S> Serve<M, S> {}

impl<M, S> Debug for Serve<M, S>
where
    M: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            stdin,
            stdout,
            make_service,
            _marker: _,
        } = self;

        f.debug_struct("Serve")
            .field("stdin", stdin)
            .field("stdout", stdout)
            .field("make_service", make_service)
            .finish()
    }
}

impl<M, S> IntoFuture for Serve<M, S>
where
    M: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <M as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    type Output = io::Result<()>;
    type IntoFuture = private::ServeFuture;

    fn into_future(self) -> Self::IntoFuture {
        private::ServeFuture(Box::pin(async move {
            let Self {
                stdin,
                stdout: _,
                mut make_service,
                _marker: _,
            } = self;
            
            let buf = tokio::io::BufReader::new(stdin);
            let mut lines = buf.lines();
            while let Ok(_line) = lines.next_line().await {
                poll_fn(|cx| make_service.poll_ready(cx))
                    .await
                    .unwrap_or_else(|err| match err {});

                // let _tower_service = make_service
                //     .call(unimplemented!())
                //     .await
                //     .unwrap_or_else(|err| match err {});

                tokio::spawn(async move {
                    todo!()
                });
            }
            todo!()
        }))
    }
}

mod private {
    use std::{
        future::Future,
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pub struct ServeFuture(pub(super) futures_util::future::BoxFuture<'static, io::Result<()>>);

    impl Future for ServeFuture {
        type Output = io::Result<()>;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.0.as_mut().poll(cx)
        }
    }

    impl std::fmt::Debug for ServeFuture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ServeFuture").finish_non_exhaustive()
        }
    }
}

#[derive(Debug)]
pub struct IncomingStream<'a> {
    msg: &'a str
}
#[derive(Clone, Default)]
pub struct Router {}

impl Router {
    pub fn new() -> Self { Self {  } }
        pub fn into_make_service(self) -> IntoMakeService<Self> {
        // call `Router::with_state` such that everything is turned into `Route` eagerly
        // rather than doing that per request
        IntoMakeService::new(self)
    }
}

pub struct IntoMakeService<S> {
    svc: S,
}

impl<S> IntoMakeService<S> {
    pub(crate) fn new(svc: S) -> Self {
        Self { svc }
    }
}
    pub type IntoMakeServiceFuture<S> =
        std::future::Ready<Result<S, Infallible>>;
impl<S, T> Service<T> for IntoMakeService<S>
where
    S: Clone,
{
    type Response = S;
    type Error = Infallible;
    type Future = IntoMakeServiceFuture<S>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _target: T) -> Self::Future {
        std::future::ready(Ok(self.svc.clone()))
    }
}

impl tower::Service<IncomingStream<'_>> for Router {
    type Response = Self;

    type Error = Infallible;

    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let _ = cx;
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: IncomingStream<'_>) -> Self::Future {
        std::future::ready(Ok(self.clone()))
    }
}
impl Service<Request> for Router
{
    type Response = Response;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: Request) -> Self::Future {
        let _ = req;
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[allow(dead_code, unused_must_use)]
    async fn if_it_compiles_it_works() {
        let router: Router = Router::new();

        // router
        serve(tokio::io::stdin(), tokio::io::stdout(), router.clone());
        serve(tokio::io::stdin(), tokio::io::stdout(), router.clone().into_make_service());

        // // method router
        // serve(tokio::io::stdin(), tokio::io::stdout(), get(handler));
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     get(handler).into_make_service(),
        // );
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     get(handler).into_make_service_with_connect_info::<SocketAddr>(),
        // );

        // // handler
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     handler.into_service(),
        // );
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     handler.with_state(()),
        // );
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     handler.into_make_service(),
        // );
        // serve(
        //     tokio::io::stdin(), tokio::io::stdout(),
        //     handler.into_make_service_with_connect_info::<SocketAddr>(),
        // );
    }

    async fn handler() {}
}