pub mod exec;
pub mod my_exec;
pub mod ordb;

use crate::types::{Time, TrId};
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
