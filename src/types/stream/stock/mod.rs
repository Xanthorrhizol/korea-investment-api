pub mod exec;
pub mod my_exec;
pub mod ordb;

use crate::{
    Error,
    types::{Time, TrId},
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
    fn body(&self) -> &Vec<T>;
    fn get_column_count(tr_id: &TrId) -> usize {
        match *tr_id {
            TrId::RealtimeExecKrx => 46,
            TrId::RealtimeOrdbKrx => 59,
            TrId::RealtimeExecNxt => 46,
            TrId::RealtimeOrdbNxt => 65,
            TrId::RealtimeExecUnion => 46,
            TrId::RealtimeOrdbUnion => 65,
            _ => unreachable!(),
        }
    }
}
