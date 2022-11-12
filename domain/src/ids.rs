use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Id<T>(Uuid, PhantomData<T>);

#[derive(Debug, Clone, Copy)]
pub struct ClipId;
