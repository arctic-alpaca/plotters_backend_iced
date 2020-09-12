//! Listen to external events in your application.
mod tracker;

pub use tracker::Tracker;

use crate::BoxStream;

/// A request to listen to external events.
///
/// Besides performing async actions on demand with [`Command`], most
/// applications also need to listen to external events passively.
///
/// A [`Subscription`] is normally provided to some runtime, like a [`Command`],
/// and it will generate events as long as the user keeps requesting it.
///
/// For instance, you can use a [`Subscription`] to listen to a WebSocket
/// connection, keyboard presses, mouse events, time ticks, etc.
///
/// This type is normally aliased by runtimes with a specific `Event` and/or
/// `Hasher`.
///
/// [`Command`]: ../struct.Command.html
/// [`Subscription`]: struct.Subscription.html
pub struct Subscription<Hasher, Event, Output> {
    recipes: Vec<Box<dyn Recipe<Hasher, Event, Output = Output>>>,
}

impl<H, E, O> Subscription<H, E, O>
where
    H: std::hash::Hasher,
{
    /// Returns an empty [`Subscription`] that will not produce any output.
    ///
    /// [`Subscription`]: struct.Subscription.html
    pub fn none() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }

    /// Creates a [`Subscription`] from a [`Recipe`] describing it.
    ///
    /// [`Subscription`]: struct.Subscription.html
    /// [`Recipe`]: trait.Recipe.html
    pub fn from_recipe(
        recipe: impl Recipe<H, E, Output = O> + 'static,
    ) -> Self {
        Self {
            recipes: vec![Box::new(recipe)],
        }
    }

    /// Batches all the provided subscriptions and returns the resulting
    /// [`Subscription`].
    ///
    /// [`Subscription`]: struct.Subscription.html
    pub fn batch(
        subscriptions: impl IntoIterator<Item = Subscription<H, E, O>>,
    ) -> Self {
        Self {
            recipes: subscriptions
                .into_iter()
                .flat_map(|subscription| subscription.recipes)
                .collect(),
        }
    }

    /// Returns the different recipes of the [`Subscription`].
    ///
    /// [`Subscription`]: struct.Subscription.html
    pub fn recipes(self) -> Vec<Box<dyn Recipe<H, E, Output = O>>> {
        self.recipes
    }

    /// Adds a value to the [`Subscription`] context.
    ///
    /// The value will be part of the identity of a [`Subscription`].
    ///
    /// This is necessary if you want to use multiple instances of the same
    /// [`Subscription`] to produce different kinds of messages based on some
    /// external data.
    ///
    /// [`Subscription`]: struct.Subscription.html
    pub fn with<T>(mut self, value: T) -> Subscription<H, E, (T, O)>
    where
        H: 'static,
        E: 'static,
        O: 'static,
        T: std::hash::Hash + Clone + Send + Sync + 'static,
    {
        Subscription {
            recipes: self
                .recipes
                .drain(..)
                .map(|recipe| {
                    Box::new(With::new(recipe, value.clone()))
                        as Box<dyn Recipe<H, E, Output = (T, O)>>
                })
                .collect(),
        }
    }

    /// Transforms the [`Subscription`] output with the given function.
    ///
    /// [`Subscription`]: struct.Subscription.html
    pub fn map<A>(
        mut self,
        f: impl Fn(O) -> A + Send + Sync + 'static,
    ) -> Subscription<H, E, A>
    where
        H: 'static,
        E: 'static,
        O: 'static,
        A: 'static,
    {
        let function = std::sync::Arc::new(f);

        Subscription {
            recipes: self
                .recipes
                .drain(..)
                .map(|recipe| {
                    Box::new(Map::new(recipe, function.clone()))
                        as Box<dyn Recipe<H, E, Output = A>>
                })
                .collect(),
        }
    }
}

impl<I, O, H> std::fmt::Debug for Subscription<I, O, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription").finish()
    }
}

/// The description of a [`Subscription`].
///
/// A [`Recipe`] is the internal definition of a [`Subscription`]. It is used
/// by runtimes to run and identify subscriptions. You can use it to create your
/// own!
///
/// [`Subscription`]: struct.Subscription.html
/// [`Recipe`]: trait.Recipe.html
///
/// # Examples
/// The repository has a couple of [examples] that use a custom [`Recipe`]:
///
/// - [`download_progress`], a basic application that asynchronously downloads
/// a dummy file of 100 MB and tracks the download progress.
/// - [`stopwatch`], a watch with start/stop and reset buttons showcasing how
/// to listen to time.
///
/// [examples]: https://github.com/hecrj/iced/tree/0.1/examples
/// [`download_progress`]: https://github.com/hecrj/iced/tree/0.1/examples/download_progress
/// [`stopwatch`]: https://github.com/hecrj/iced/tree/0.1/examples/stopwatch
pub trait Recipe<Hasher: std::hash::Hasher, Event> {
    /// The events that will be produced by a [`Subscription`] with this
    /// [`Recipe`].
    ///
    /// [`Subscription`]: struct.Subscription.html
    /// [`Recipe`]: trait.Recipe.html
    type Output;

    /// Hashes the [`Recipe`].
    ///
    /// This is used by runtimes to uniquely identify a [`Subscription`].
    ///
    /// [`Subscription`]: struct.Subscription.html
    /// [`Recipe`]: trait.Recipe.html
    fn hash(&self, state: &mut Hasher);

    /// Executes the [`Recipe`] and produces the stream of events of its
    /// [`Subscription`].
    ///
    /// It receives some stream of generic events, which is normally defined by
    /// shells.
    ///
    /// [`Subscription`]: struct.Subscription.html
    /// [`Recipe`]: trait.Recipe.html
    fn stream(
        self: Box<Self>,
        input: BoxStream<Event>,
    ) -> BoxStream<Self::Output>;
}

struct Map<Hasher, Event, A, B> {
    recipe: Box<dyn Recipe<Hasher, Event, Output = A>>,
    mapper: std::sync::Arc<dyn Fn(A) -> B + Send + Sync>,
}

impl<H, E, A, B> Map<H, E, A, B> {
    fn new(
        recipe: Box<dyn Recipe<H, E, Output = A>>,
        mapper: std::sync::Arc<dyn Fn(A) -> B + Send + Sync + 'static>,
    ) -> Self {
        Map { recipe, mapper }
    }
}

impl<H, E, A, B> Recipe<H, E> for Map<H, E, A, B>
where
    A: 'static,
    B: 'static,
    H: std::hash::Hasher,
{
    type Output = B;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<B>().hash(state);
        self.recipe.hash(state);
    }

    fn stream(self: Box<Self>, input: BoxStream<E>) -> BoxStream<Self::Output> {
        use futures::StreamExt;

        let mapper = self.mapper;

        Box::pin(
            self.recipe
                .stream(input)
                .map(move |element| mapper(element)),
        )
    }
}

struct With<Hasher, Event, A, B> {
    recipe: Box<dyn Recipe<Hasher, Event, Output = A>>,
    value: B,
}

impl<H, E, A, B> With<H, E, A, B> {
    fn new(recipe: Box<dyn Recipe<H, E, Output = A>>, value: B) -> Self {
        With { recipe, value }
    }
}

impl<H, E, A, B> Recipe<H, E> for With<H, E, A, B>
where
    A: 'static,
    B: 'static + std::hash::Hash + Clone + Send + Sync,
    H: std::hash::Hasher,
{
    type Output = (B, A);

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<B>().hash(state);
        self.value.hash(state);
        self.recipe.hash(state);
    }

    fn stream(self: Box<Self>, input: BoxStream<E>) -> BoxStream<Self::Output> {
        use futures::StreamExt;

        let value = self.value;

        Box::pin(
            self.recipe
                .stream(input)
                .map(move |element| (value.clone(), element)),
        )
    }
}
