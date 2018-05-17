#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[postgres(oid = "999", array_oid = "1999")]
#[sqlite_type = "Point"]
#[mysql_type = "Point"]
pub struct Point;