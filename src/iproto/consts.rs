pub const IPROTO_SELECT: u8 = 0x01;
pub const IPROTO_INSERT: u8 = 0x02;
pub const IPROTO_REPLACE: u8 = 0x03;
pub const IPROTO_UPDATE: u8 = 0x04;
pub const IPROTO_DELETE: u8 = 0x05;
pub const IPROTO_CALL_16: u8 = 0x06;
pub const IPROTO_AUTH: u8 = 0x07;
pub const IPROTO_EVAL: u8 = 0x08;
pub const IPROTO_UPSERT: u8 = 0x09;
pub const IPROTO_CALL: u8 = 0x0a;
pub const IPROTO_EXECUTE: u8 = 0x0b;
pub const IPROTO_NOP: u8 = 0x0c;
pub const IPROTO_PREPARE: u8 = 0x0d;
pub const IPROTO_CONFIRM: u8 = 0x28;
pub const IPROTO_ROLLBACK: u8 = 0x29;
pub const IPROTO_PING: u8 = 0x40;
pub const IPROTO_JOIN: u8 = 0x41;
pub const IPROTO_SUBSCRIBE: u8 = 0x42;
pub const IPROTO_VOTE_DEPRECATED: u8 = 0x43;
pub const IPROTO_VOTE: u8 = 0x44;
pub const IPROTO_FETCH_SNAPSHOT: u8 = 0x45;
pub const IPROTO_REGISTER: u8 = 0x46;

pub const IPROTO_OK: u8 = 0x00;
pub const IPROTO_REQUEST_TYPE: u8 = 0x00;
pub const IPROTO_SYNC: u8 = 0x01;
pub const IPROTO_REPLICA_ID: u8 = 0x02;
pub const IPROTO_LSN: u8 = 0x03;
pub const IPROTO_TIMESTAMP: u8 = 0x04;
pub const IPROTO_SCHEMA_VERSION: u8 = 0x05;
pub const IPROTO_FLAGS: u8 = 0x09;
pub const IPROTO_SPACE_ID: u8 = 0x10;
pub const IPROTO_INDEX_ID: u8 = 0x11;
pub const IPROTO_LIMIT: u8 = 0x12;
pub const IPROTO_OFFSET: u8 = 0x13;
pub const IPROTO_ITERATOR: u8 = 0x14;
pub const IPROTO_INDEX_BASE: u8 = 0x15;
pub const IPROTO_KEY: u8 = 0x20;
pub const IPROTO_TUPLE: u8 = 0x21;
pub const IPROTO_FUNCTION_NAME: u8 = 0x22;
pub const IPROTO_USER_NAME: u8 = 0x23;
pub const IPROTO_INSTANCE_UUID: u8 = 0x24;
pub const IPROTO_CLUSTER_UUID: u8 = 0x25;
pub const IPROTO_VCLOCK: u8 = 0x26;
pub const IPROTO_EXPR: u8 = 0x27;
pub const IPROTO_OPS: u8 = 0x28;
pub const IPROTO_BALLOT: u8 = 0x29;
pub const IPROTO_BALLOT_IS_RO_CFG: u8 = 0x01;
pub const IPROTO_BALLOT_VCLOCK: u8 = 0x02;
pub const IPROTO_BALLOT_GC_VCLOCK: u8 = 0x03;
pub const IPROTO_BALLOT_IS_RO: u8 = 0x04;
pub const IPROTO_BALLOT_IS_ANON: u8 = 0x05;
pub const IPROTO_BALLOT_IS_BOOTED: u8 = 0x06;
pub const IPROTO_TUPLE_META: u8 = 0x2a;
pub const IPROTO_OPTIONS: u8 = 0x2b;
pub const IPROTO_DATA: u8 = 0x30;
pub const IPROTO_ERROR_24: u8 = 0x31;
pub const IPROTO_METADATA: u8 = 0x32;
pub const IPROTO_BIND_METADATA: u8 = 0x33;
pub const IPROTO_BIND_COUNT: u8 = 0x34;
pub const IPROTO_SQL_TEXT: u8 = 0x40;
pub const IPROTO_SQL_BIND: u8 = 0x41;
pub const IPROTO_SQL_INFO: u8 = 0x42;
pub const IPROTO_STMT_ID: u8 = 0x43;
pub const IPROTO_ERROR: u8 = 0x52;
pub const IPROTO_FIELD_NAME: u8 = 0x00;
pub const IPROTO_FIELD_TYPE: u8 = 0x01;
pub const IPROTO_FIELD_COLL: u8 = 0x02;
pub const IPROTO_FIELD_IS_NULLABLE: u8 = 0x03;
pub const IPROTO_FIELD_IS_AUTOINCREMENT: u8 = 0x04;
pub const IPROTO_FIELD_SPAN: u8 = 0x05;