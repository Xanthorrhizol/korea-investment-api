pub mod exec;
pub mod my_exec;
pub mod ordb;

use crate::{
    types::{Time, TrId},
    Error,
};
pub use exec::Exec;
pub use my_exec::MyExec;
pub use ordb::Ordb;

#[derive(Debug, Clone)]
pub struct Header {
    tr_id: TrId,
    datetime: Time,
}
impl Header {
    pub fn tr_id(&self) -> &TrId {
        &self.tr_id
    }

    pub fn datetime(&self) -> &Time {
        &self.datetime
    }
}

pub trait StreamParser<T>
where
    Self: Sized + 'static,
    T: Clone,
{
    fn parse(s: String) -> Result<Self, Error>;
    fn header(&self) -> &Header;
    fn body(&self) -> &Option<T>;
}
