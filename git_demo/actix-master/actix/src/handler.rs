use std::{fmt, future::Future, pin::Pin, sync::Arc};

pub use tokio::sync::oneshot::Sender as OneshotSender;

use crate::{
    actor::{Actor, AsyncContext},
    address::Addr,
    fut::{ActorFuture, ActorFutureExt, LocalBoxActorFuture},
};

/// Describes how to handle messages of a specific type.
///
/// Implementing `Handler` is a general way to handle incoming
/// messages, streams, and futures.
///
/// The type `M` is a message which can be handled by the actor.
#[allow(unused_variables)]
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
{
    /// The type of value that this handler will return.
    ///
    /// Check the [`MessageResponse`] trait for some details
    /// on how a message can be responded to.
    type Result: MessageResponse<Self, M>;

    /// This method is called for every message received by this actor.
    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result;
}

/// Represent message that can be handled by an actor.
pub trait Message {
    /// The type of value that this message will resolved with if it is
    /// successful.
    type Result: 'static;
}

/// Allow users to use `Arc<M>` as a message without having to re-impl `Message`
impl<M> Message for Arc<M>
where
    M: Message,
{
    type Result = M::Result;
}

/// Allow users to use `Box<M>` as a message without having to re-impl `Message`
impl<M> Message for Box<M>
where
    M: Message,
{
    type Result = M::Result;
}

/// A helper type that implements the [`MessageResponse`] trait.
///
/// # Examples
/// ```no_run
/// use actix::prelude::*;
///
/// #[derive(Message)]
/// #[rtype(result = "Response")]
/// struct Msg;
///
/// struct Response;
///
/// struct MyActor;
///
/// impl Actor for MyActor {
///     type Context = Context<Self>;
/// }
///
/// impl Handler<Msg> for MyActor {
///     type Result = MessageResult<Msg>;
///
///     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {
///         MessageResult(Response {})
///     }
/// }
/// ```
pub struct MessageResult<M: Message>(pub M::Result);

/// A specialized actor future holder for atomic asynchronous message handling.
///
/// Intended be used when the future returned will need exclusive access Actor's
/// internal state or context, e.g., it can yield at critical sessions.
/// When the actor starts to process this future, it will not pull any other
/// spawned futures until this one as been completed.
/// Check [`ActorFuture`] for available methods for accessing Actor's
/// internal state.
///
/// ## Note
/// The runtime itself is not blocked in the process, only the Actor,
/// other futures, and therefore, other actors are still allowed to make
/// progress when this [`AtomicResponse`] is used.
///
/// # Examples
/// On the following example, the response to `Msg` would always be 29
/// even if there are multiple `Msg` sent to `MyActor`.
/// ```no_run
/// # use actix::prelude::*;
/// # use actix::clock::sleep;
/// # use std::time::Duration;
/// #
/// # #[derive(Message)]
/// # #[rtype(result = "usize")]
/// # struct Msg;
/// #
/// # struct MyActor(usize);
/// #
/// # impl Actor for MyActor {
/// #    type Context = Context<Self>;
/// # }
/// #
/// impl Handler<Msg> for MyActor {
///     type Result = AtomicResponse<Self, usize>;
///
///     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {
///         AtomicResponse::new(Box::pin(
///             async {}
///                 .into_actor(self)
///                 .map(|_, this, _| {
///                     this.0 = 30;
///                 })
///                 .then(|_, this, _| {
///                     sleep(Duration::from_secs(3)).into_actor(this)
///                 })
///                 .map(|_, this, _| {
///                     this.0 -= 1;
///                     this.0
///                 }),
///         ))
///     }
/// }
/// ```
pub struct AtomicResponse<A, T>(ResponseActFuture<A, T>);

impl<A, T> AtomicResponse<A, T> {
    pub fn new(fut: ResponseActFuture<A, T>) -> Self {
        AtomicResponse(fut)
    }
}

impl<A, M> MessageResponse<A, M> for AtomicResponse<A, M::Result>
where
    A: Actor,
    M: Message,
    A::Context: AsyncContext<A>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        ctx.wait(self.0.map(|res, _, _| tx.send(res)));
    }
}

/// A specialized actor future for asynchronous message handling.
///
/// Intended be used when the future returned will, at some point, need to access Actor's internal
/// state or context in order to finish. Check [`ActorFuture`] for available methods for accessing
/// Actor's internal state.
///
/// # Note
/// It's important to keep in mind that the provided [`AsyncContext`], does not enforce the poll of
/// any [`ActorFuture`] to be exclusive. Therefore, if other instances of [`ActorFuture`] are spawned
/// into this Context **their execution won't necessarily be atomic**. Check [`AtomicResponse`] if you
/// need exclusive access over the actor.
///
/// # Examples
/// ```no_run
/// use actix::prelude::*;
///
/// #[derive(Message)]
/// #[rtype(result = "Result<usize, ()>")]
/// struct Msg;
///
/// struct MyActor;
///
/// impl Actor for MyActor {
///     type Context = Context<Self>;
/// }
///
/// impl Handler<Msg> for MyActor {
///     type Result = ResponseActFuture<Self, Result<usize, ()>>;
///
///     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {
///         Box::pin(
///             async {
///                 // Some async computation
///                 42
///             }
///             .into_actor(self) // converts future to ActorFuture
///             .map(|res, _act, _ctx| {
///                 // Do some computation with actor's state or context
///                 Ok(res)
///             }),
///         )
///     }
/// }
/// ```
pub type ResponseActFuture<A, I> = LocalBoxActorFuture<A, I>;

/// A specialized future for asynchronous message handling.
///
/// Intended be used for when the future returned doesn't
/// need to access Actor's internal state or context to progress, either
/// because it's completely agnostic to it, or because the required data has
/// already been moved inside the future and it won't need Actor state to continue.
///
/// # Examples
/// ```no_run
/// use actix::prelude::*;
///
/// #[derive(Message)]
/// #[rtype(result = "Result<(), ()>")]
/// struct Msg;
///
/// struct MyActor;
///
/// impl Actor for MyActor {
///     type Context = Context<Self>;
/// }
///
/// impl Handler<Msg> for MyActor {
///     type Result = ResponseFuture<Result<(), ()>>;
///
///     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {
///         Box::pin(async move {
///             // Some async computation
///             Ok(())
///         })
///     }
/// }
/// ```
pub type ResponseFuture<I> = Pin<Box<dyn Future<Output = I>>>;

/// A trait which defines message responses.
///
/// We offer implementation for some common language types, if you need
/// to respond with a new type you can use [`MessageResult`].
///
/// If `Actor::Context` implements [`AsyncContext`] it's possible to handle
/// the message asynchronously.
/// For asynchronous message handling we offer the following possible response types:
/// - [`ResponseFuture`] should be used for when the future returned doesn't
///   need to access Actor's internal state or context to progress, either
///   because it's completely agnostic to it or because the required data has
///   already been moved to it and it won't need Actor state to continue.
/// - [`ResponseActFuture`] should be used when the future returned
///   will, at some point, need to access Actor's internal state or context
///   in order to finish.
/// - [`AtomicResponse`] should be used when the future returned needs exclusive
///   access to  Actor's internal state or context.
pub trait MessageResponse<A: Actor, M: Message> {
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>);
}

impl<A, M> MessageResponse<A, M> for MessageResult<M>
where
    A: Actor,
    M: Message,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        tx.send(self.0)
    }
}

impl<A, M, I, E> MessageResponse<A, M> for Result<I, E>
where
    A: Actor,
    M: Message<Result = Self>,
    I: 'static,
    E: 'static,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<Self>>) {
        tx.send(self)
    }
}

impl<A, M, I> MessageResponse<A, M> for Arc<I>
where
    A: Actor,
    M: Message<Result = Self>,
    I: 'static,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<Self>>) {
        tx.send(self)
    }
}

impl<A, M, I> MessageResponse<A, M> for Option<I>
where
    A: Actor,
    M: Message<Result = Self>,
    I: 'static,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<Self>>) {
        tx.send(self)
    }
}

impl<A, M, I> MessageResponse<A, M> for Vec<I>
where
    A: Actor,
    M: Message<Result = Self>,
    I: 'static,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<Self>>) {
        tx.send(self)
    }
}

impl<A, M, B> MessageResponse<A, M> for Addr<B>
where
    A: Actor,
    M: Message<Result = Self>,
    B: Actor,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<Self>>) {
        tx.send(self)
    }
}

impl<A, M> MessageResponse<A, M> for ResponseActFuture<A, M::Result>
where
    A: Actor,
    M: Message,
    A::Context: AsyncContext<A>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        ctx.spawn(self.map(|res, _, _| tx.send(res)));
    }
}

/// [`MessageResponse`] trait impl to enable the use of any `I: 'static` with asynchronous
/// message handling
///
/// # Examples
/// Usage with `Result<I,E>`:
/// ```
/// # pub struct MyActorAsync {}
/// # impl Actor for MyActorAsync { type Context = actix::Context<Self>; }
/// # use actix::prelude::*;
/// # use core::pin::Pin;
/// #
/// pub struct MyQuestion{}
/// impl Message for MyQuestion {
///     type Result = Result<u8,u8>;
/// }
/// impl Handler<MyQuestion> for MyActorAsync {
///     type Result = ResponseFuture<Result<u8,u8>>;
///     fn handle(&mut self, question: MyQuestion, _ctx: &mut Context<Self>) -> Self::Result {
///         Box::pin(async {Ok(5)})
///     }
/// }
/// ```
/// Usage with `Option<I>`:
/// ```
/// # pub struct MyActorAsync {}
/// # impl Actor for MyActorAsync { type Context = actix::Context<Self>; }
/// # use actix::prelude::*;
/// # use core::pin::Pin;
/// pub struct MyQuestion{}
/// impl Message for MyQuestion {
///     type Result = Option<u8>;
/// }
/// impl Handler<MyQuestion> for MyActorAsync {
///     type Result = ResponseFuture<Option<u8>>;
///     fn handle(&mut self, question: MyQuestion, _ctx: &mut Context<Self>) -> Self::Result {
///         Box::pin(async {Some(5)})
///     }
/// }
/// ```
/// Usage with any `I: 'static`:
/// ```
/// # pub struct MyActorAsync {}
/// # impl Actor for MyActorAsync { type Context = actix::Context<Self>; }
/// # use actix::prelude::*;
/// # use core::pin::Pin;
/// pub struct MyQuestion{}
/// impl Message for MyQuestion {
///     type Result = u8;
/// }
/// impl Handler<MyQuestion> for MyActorAsync {
///     type Result = ResponseFuture<u8>;
///     fn handle(&mut self, question: MyQuestion, _ctx: &mut Context<Self>) -> Self::Result {
///         Box::pin(async {5})
///     }
/// }
/// ```
impl<A, M> MessageResponse<A, M> for ResponseFuture<M::Result>
where
    A: Actor,
    M: Message,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        actix_rt::spawn(async { tx.send(self.await) });
    }
}

enum ResponseTypeItem<I> {
    Result(I),
    Fut(Pin<Box<dyn Future<Output = I>>>),
}

/// Helper type for representing different type of message responses
pub struct Response<I> {
    item: ResponseTypeItem<I>,
}

impl<I> fmt::Debug for Response<I> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_struct("Response");
        match self.item {
            ResponseTypeItem::Result(_) => fmt.field("item", &"Result(_)".to_string()),
            ResponseTypeItem::Fut(_) => fmt.field("item", &"Fut(_)".to_string()),
        }
        .finish()
    }
}

impl<I> Response<I> {
    /// Creates an asynchronous response.
    pub fn fut<T>(fut: T) -> Self
    where
        T: Future<Output = I> + 'static,
    {
        Self {
            item: ResponseTypeItem::Fut(Box::pin(fut)),
        }
    }

    /// Creates a response.
    pub fn reply(val: I) -> Self {
        Self {
            item: ResponseTypeItem::Result(val),
        }
    }
}

impl<A, M> MessageResponse<A, M> for Response<M::Result>
where
    A: Actor,
    M: Message,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        match self.item {
            ResponseTypeItem::Fut(fut) => {
                actix_rt::spawn(async { tx.send(fut.await) });
            }
            ResponseTypeItem::Result(res) => tx.send(res),
        }
    }
}

enum ActorResponseTypeItem<A, I> {
    Result(I),
    Fut(Pin<Box<dyn ActorFuture<A, Output = I>>>),
}

/// A helper type for representing different types of message responses.
pub struct ActorResponse<A, I> {
    item: ActorResponseTypeItem<A, I>,
}

impl<A, I> fmt::Debug for ActorResponse<A, I> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_struct("ActorResponse");
        match self.item {
            ActorResponseTypeItem::Result(_) => fmt.field("item", &"Result(_)".to_string()),
            ActorResponseTypeItem::Fut(_) => fmt.field("item", &"Fut(_)".to_string()),
        }
        .finish()
    }
}

impl<A: Actor, I> ActorResponse<A, I> {
    /// Creates a response.
    pub fn reply(val: I) -> Self {
        Self {
            item: ActorResponseTypeItem::Result(val),
        }
    }

    /// Creates an asynchronous response.
    pub fn r#async<T>(fut: T) -> Self
    where
        T: ActorFuture<A, Output = I> + 'static,
    {
        Self {
            item: ActorResponseTypeItem::Fut(Box::pin(fut)),
        }
    }
}

impl<A, M> MessageResponse<A, M> for ActorResponse<A, M::Result>
where
    A: Actor,
    M: Message,
    A::Context: AsyncContext<A>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        match self.item {
            ActorResponseTypeItem::Fut(fut) => {
                let fut = fut.map(|res, _, _| tx.send(res));
                ctx.spawn(fut);
            }
            ActorResponseTypeItem::Result(res) => tx.send(res),
        }
    }
}

impl<A: Actor, I> From<Pin<Box<dyn ActorFuture<A, Output = I>>>> for ActorResponse<A, I> {
    fn from(fut: Pin<Box<dyn ActorFuture<A, Output = I>>>) -> Self {
        Self {
            item: ActorResponseTypeItem::Fut(fut),
        }
    }
}

macro_rules! SIMPLE_RESULT {
    ($type:ty) => {
        impl<A, M> MessageResponse<A, M> for $type
        where
            A: Actor,
            M: Message<Result = $type>,
        {
            fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<$type>>) {
                tx.send(self)
            }
        }
    };
}

SIMPLE_RESULT!(());
SIMPLE_RESULT!(u8);
SIMPLE_RESULT!(u16);
SIMPLE_RESULT!(u32);
SIMPLE_RESULT!(u64);
SIMPLE_RESULT!(usize);
SIMPLE_RESULT!(i8);
SIMPLE_RESULT!(i16);
SIMPLE_RESULT!(i32);
SIMPLE_RESULT!(i64);
SIMPLE_RESULT!(isize);
SIMPLE_RESULT!(f32);
SIMPLE_RESULT!(f64);
SIMPLE_RESULT!(String);
SIMPLE_RESULT!(bool);

// Helper trait for send one shot message from Option<Sender> type.
// None and error are ignored.
trait OneshotSend<M> {
    fn send(self, msg: M);
}

impl<M> OneshotSend<M> for Option<OneshotSender<M>> {
    fn send(self, msg: M) {
        if let Some(tx) = self {
            let _ = tx.send(msg);
        }
    }
}
