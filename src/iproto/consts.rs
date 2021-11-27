#![allow(dead_code)]

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

// Unknown error
pub const ER_UNKNOWN: u8 = 0;
// Illegal parameters, %s
pub const ER_ILLEGAL_PARAMS: u8 = 1;
// Failed to allocate %u bytes in %s for %s
pub const ER_MEMORY_ISSUE: u8 = 2;
// Duplicate key exists in unique index "%s" in space "%s" with old tuple - %s and new tuple - %s
pub const ER_TUPLE_FOUND: u8 = 3;
// Tuple doesn't exist in index '%s' in space '%s'
pub const ER_TUPLE_NOT_FOUND: u8 = 4;
// %s does not support %s
pub const ER_UNSUPPORTED: u8 = 5;
// Can't modify data on a replication slave. My master is: %s
pub const ER_NONMASTER: u8 = 6;
// Can't modify data on a read-only instance
pub const ER_READONLY: u8 = 7;
// Error injection '%s'
pub const ER_INJECTION: u8 = 8;
// Failed to create space '%s': %s
pub const ER_CREATE_SPACE: u8 = 9;
// Space '%s' already exists
pub const ER_SPACE_EXISTS: u8 = 10;
// Can't drop space '%s': %s
pub const ER_DROP_SPACE: u8 = 11;
// Can't modify space '%s': %s
pub const ER_ALTER_SPACE: u8 = 12;
// Unsupported index type supplied for index '%s' in space '%s'
pub const ER_INDEX_TYPE: u8 = 13;
// Can't create or modify index '%s' in space '%s': %s
pub const ER_MODIFY_INDEX: u8 = 14;
// Can't drop the primary key in a system space, space '%s'
pub const ER_LAST_DROP: u8 = 15;
// Tuple format limit reached: %u
pub const ER_TUPLE_FORMAT_LIMIT: u8 = 16;
// Can't drop primary key in space '%s' while secondary keys exist
pub const ER_DROP_PRIMARY_KEY: u8 = 17;
// Supplied key type of part %u does not match index part type: expected %s
pub const ER_KEY_PART_TYPE: u8 = 18;
// Invalid key part count in an exact match (expected %u, got %u)
pub const ER_EXACT_MATCH: u8 = 19;
// Invalid MsgPack - %s
pub const ER_INVALID_MSGPACK: u8 = 20;
// msgpack.encode: can not encode Lua type '%s'
pub const ER_PROC_RET: u8 = 21;
// Tuple/Key must be MsgPack array
pub const ER_TUPLE_NOT_ARRAY: u8 = 22;
// Tuple field %s type does not match one required by operation: expected %s, got %s
pub const ER_FIELD_TYPE: u8 = 23;
// Field %s has type '%s' in one index, but type '%s' in another
pub const ER_INDEX_PART_TYPE_MISMATCH: u8 = 24;
// SPLICE error on field %s: %s
pub const ER_UPDATE_SPLICE: u8 = 25;
// Argument type in operation '%c' on field %s does not match field type: expected %s
pub const ER_UPDATE_ARG_TYPE: u8 = 26;
// Field %s has type '%s' in space format, but type '%s' in index definition
pub const ER_FORMAT_MISMATCH_INDEX_PART: u8 = 27;
// Unknown UPDATE operation #%d: %s
pub const ER_UNKNOWN_UPDATE_OP: u8 = 28;
// Field %s UPDATE error: %s
pub const ER_UPDATE_FIELD: u8 = 29;
// Transaction is active at return from function
pub const ER_FUNCTION_TX_ACTIVE: u8 = 30;
// Invalid key part count (expected [0..%u], got %u)
pub const ER_KEY_PART_COUNT: u8 = 31;
// %s
pub const ER_PROC_LUA: u8 = 32;
// Procedure '%.*s' is not defined
pub const ER_NO_SUCH_PROC: u8 = 33;
// Trigger '%s' doesn't exist
pub const ER_NO_SUCH_TRIGGER: u8 = 34;
// No index #%u is defined in space '%s'
pub const ER_NO_SUCH_INDEX_ID: u8 = 35;
// Space '%s' does not exist
pub const ER_NO_SUCH_SPACE: u8 = 36;
// Field %d was not found in the tuple
pub const ER_NO_SUCH_FIELD_NO: u8 = 37;
// Tuple field count %u does not match space field count %u
pub const ER_EXACT_FIELD_COUNT: u8 = 38;
// Tuple field %s required by space format is missing
pub const ER_FIELD_MISSING: u8 = 39;
// Failed to write to disk
pub const ER_WAL_IO: u8 = 40;
// Get() doesn't support partial keys and non-unique indexes
pub const ER_MORE_THAN_ONE_TUPLE: u8 = 41;
// s access to %s '%s' is denied for user '%s'
pub const ER_ACCESS_DENIED: u8 = 42;
// Failed to create user '%s': %s
pub const ER_CREATE_USER: u8 = 43;
// Failed to drop user or role '%s': %s
pub const ER_DROP_USER: u8 = 44;
// User '%s' is not found
pub const ER_NO_SUCH_USER: u8 = 45;
// User '%s' already exists
pub const ER_USER_EXISTS: u8 = 46;
// Incorrect password supplied for user '%s'
pub const ER_PASSWORD_MISMATCH: u8 = 47;
// Unknown request type %u
pub const ER_UNKNOWN_REQUEST_TYPE: u8 = 48;
// Unknown object type '%s'
pub const ER_UNKNOWN_SCHEMA_OBJECT: u8 = 49;
// Failed to create function '%s': %s
pub const ER_CREATE_FUNCTION: u8 = 50;
// Function '%s' does not exist
pub const ER_NO_SUCH_FUNCTION: u8 = 51;
// Function '%s' already exists
pub const ER_FUNCTION_EXISTS: u8 = 52;
// Invalid return value of space:before_replace trigger: expected tuple or nil, got %s
pub const ER_BEFORE_REPLACE_RET: u8 = 53;
//  not perform %s in a multi-statement transaction
pub const ER_MULTISTATEMENT_TRANSACTION: u8 = 54;
// Trigger '%s' already exists
pub const ER_TRIGGER_EXISTS: u8 = 55;
// A limit on the total number of users has been reached: %u
pub const ER_USER_MAX: u8 = 56;
// Space engine '%s' does not exist
pub const ER_NO_SUCH_ENGINE: u8 = 57;
// Can't set option '%s' dynamically
pub const ER_RELOAD_CFG: u8 = 58;
// Incorrect value for option '%s': %s
pub const ER_CFG: u8 = 59;
// Can not set a savepoint in an empty transaction
pub const ER_SAVEPOINT_EMPTY_TX: u8 = 60;
// Can not rollback to savepoint: the savepoint does not exist
pub const ER_NO_SUCH_SAVEPOINT: u8 = 61;
// Replica %s is not registered with replica set %s
pub const ER_UNKNOWN_REPLICA: u8 = 62;
// Replica set UUID mismatch: expected %s, got %s
pub const ER_REPLICASET_UUID_MISMATCH: u8 = 63;
// Invalid UUID: %s
pub const ER_INVALID_UUID: u8 = 64;
// Can't reset replica set UUID: it is already assigned
pub const ER_REPLICASET_UUID_IS_RO: u8 = 65;
// Instance UUID mismatch: expected %s, got %s
pub const ER_INSTANCE_UUID_MISMATCH: u8 = 66;
// Can't initialize replica id with a reserved value %u
pub const ER_REPLICA_ID_IS_RESERVED: u8 = 67;
// Invalid LSN order for instance %u: previous LSN = %llu, new lsn = %llu
pub const ER_INVALID_ORDER: u8 = 68;
// Missing mandatory field '%s' in request
pub const ER_MISSING_REQUEST_FIELD: u8 = 69;
// Invalid identifier '%s' (expected printable symbols only or it is too long)
pub const ER_IDENTIFIER: u8 = 70;
// Can't drop function %u: %s
pub const ER_DROP_FUNCTION: u8 = 71;
// Unknown iterator type '%s'
pub const ER_ITERATOR_TYPE: u8 = 72;
// Replica count limit reached: %u
pub const ER_REPLICA_MAX: u8 = 73;
// Failed to read xlog: %lld
pub const ER_INVALID_XLOG: u8 = 74;
// Invalid xlog name: expected %lld got %lld
pub const ER_INVALID_XLOG_NAME: u8 = 75;
// Invalid xlog order: %lld and %lld
pub const ER_INVALID_XLOG_ORDER: u8 = 76;
// Connection is not established
pub const ER_NO_CONNECTION: u8 = 77;
// Timeout exceeded
pub const ER_TIMEOUT: u8 = 78;
// Operation is not permitted when there is an active transaction 
pub const ER_ACTIVE_TRANSACTION: u8 = 79;
// The transaction the cursor belongs to has ended
pub const ER_CURSOR_NO_TRANSACTION: u8 = 80;
// A multi-statement transaction can not use multiple storage engines
pub const ER_CROSS_ENGINE_TRANSACTION: u8 = 81;
// Role '%s' is not found
pub const ER_NO_SUCH_ROLE: u8 = 82;
// Role '%s' already exists
pub const ER_ROLE_EXISTS: u8 = 83;
// Failed to create role '%s': %s
pub const ER_CREATE_ROLE: u8 = 84;
// Index '%s' already exists
pub const ER_INDEX_EXISTS: u8 = 85;
// Session is closed
pub const ER_SESSION_CLOSED: u8 = 86;
// Granting role '%s' to role '%s' would create a loop
pub const ER_ROLE_LOOP: u8 = 87;
// Incorrect grant arguments: %s
pub const ER_GRANT: u8 = 88;
// User '%s' already has %s access on %s%s
pub const ER_PRIV_GRANTED: u8 = 89;
// User '%s' already has role '%s'
pub const ER_ROLE_GRANTED: u8 = 90;
// User '%s' does not have %s access on %s '%s'
pub const ER_PRIV_NOT_GRANTED: u8 = 91;
// User '%s' does not have role '%s'
pub const ER_ROLE_NOT_GRANTED: u8 = 92;
// Can't find snapshot
pub const ER_MISSING_SNAPSHOT: u8 = 93;
// Attempt to modify a tuple field which is part of index '%s' in space '%s'
pub const ER_CANT_UPDATE_PRIMARY_KEY: u8 = 94;
// Integer overflow when performing '%c' operation on field %s
pub const ER_UPDATE_INTEGER_OVERFLOW: u8 = 95;
// Setting password for guest user has no effect
pub const ER_GUEST_USER_PASSWORD: u8 = 96;
// Transaction has been aborted by conflict
pub const ER_TRANSACTION_CONFLICT: u8 = 97;
// Unsupported %s privilege '%s'
pub const ER_UNSUPPORTED_PRIV: u8 = 98;
// Failed to dynamically load function '%s': %s
pub const ER_LOAD_FUNCTION: u8 = 99;
// Unsupported language '%s' specified for function '%s'
pub const ER_FUNCTION_LANGUAGE: u8 = 100;
// RTree: %s must be an array with %u (point) or %u (rectangle/box) numeric coordinates
pub const ER_RTREE_RECT: u8 = 101;
// %s
pub const ER_PROC_C: u8 = 102;
// Unknown RTREE index distance type %s
pub const ER_UNKNOWN_RTREE_INDEX_DISTANCE_TYPE: u8 = 103;
// %s
pub const ER_PROTOCOL: u8 = 104;
// Space %s has a unique secondary index and does not support UPSERT
pub const ER_UPSERT_UNIQUE_SECONDARY_KEY: u8 = 105;
// Wrong record in _index space: got {%s}, expected {%s}
pub const ER_WRONG_INDEX_RECORD: u8 = 106;
// Wrong index parts: %s; expected field1 id (number), field1 type (string), ...
pub const ER_WRONG_INDEX_PARTS: u8 = 107;
// Wrong index options (field %u): %s
pub const ER_WRONG_INDEX_OPTIONS: u8 = 108;
// Wrong schema version, current: %d, in request: %u
pub const ER_WRONG_SCHEMA_VERSION: u8 = 109;
// Failed to allocate %u bytes for tuple: tuple is too large. Check 'memtx_max_tuple_size' configuration option.
pub const ER_MEMTX_MAX_TUPLE_SIZE: u8 = 110;
// Wrong space options (field %u): %s
pub const ER_WRONG_SPACE_OPTIONS: u8 = 111;
// Index '%s' (%s) of space '%s' (%s) does not support %s
pub const ER_UNSUPPORTED_INDEX_FEATURE: u8 = 112;
// View '%s' is read-only
pub const ER_VIEW_IS_RO: u8 = 113;
// No active transaction
pub const ER_NO_TRANSACTION: u8 = 114;
// %s
pub const ER_SYSTEM: u8 = 115;
// Instance bootstrap hasn't finished yet
pub const ER_LOADING: u8 = 116;
// Connection to self
pub const ER_CONNECTION_TO_SELF: u8 = 117;
// Key part is too long: %u of %u bytes
pub const ER_KEY_PART_IS_TOO_LONG: u8 = 118;
// Compression error: %s
pub const ER_COMPRESSION: u8 = 119;
// Snapshot is already in progress
pub const ER_CHECKPOINT_IN_PROGRESS: u8 = 120;
// Can not execute a nested statement: nesting limit reached
pub const ER_SUB_STMT_MAX: u8 = 121;
// Can not commit transaction in a nested statement
pub const ER_COMMIT_IN_SUB_STMT: u8 = 122;
// Rollback called in a nested statement
pub const ER_ROLLBACK_IN_SUB_STMT: u8 = 123;
// Decompression error: %s
pub const ER_DECOMPRESSION: u8 = 124;
// Invalid xlog type: expected %s, got %s
pub const ER_INVALID_XLOG_TYPE: u8 = 125;
// Failed to lock WAL directory %s and hot_standby mode is off
pub const ER_ALREADY_RUNNING: u8 = 126;
// Indexed field count limit reached: %d indexed fields
pub const ER_INDEX_FIELD_COUNT_LIMIT: u8 = 127;
// The local instance id %u is read-only
pub const ER_LOCAL_INSTANCE_ID_IS_READ_ONLY: u8 = 128;
// Backup is already in progress
pub const ER_BACKUP_IN_PROGRESS: u8 = 129;
// The read view is aborted
pub const ER_READ_VIEW_ABORTED: u8 = 130;
// Invalid INDEX file %s: %s
pub const ER_INVALID_INDEX_FILE: u8 = 131;
// Invalid RUN file: %s
pub const ER_INVALID_RUN_FILE: u8 = 132;
// Invalid VYLOG file: %s
pub const ER_INVALID_VYLOG_FILE: u8 = 133;
// WAL has a rollback in progress
pub const ER_CASCADE_ROLLBACK: u8 = 134;
// Timed out waiting for Vinyl memory quota
pub const ER_VY_QUOTA_TIMEOUT: u8 = 135;
// s index  does not support selects via a partial key (expected %u parts, got %u). Please Consider changing index type to TREE.
pub const ER_PARTIAL_KEY: u8 = 136;
// Can't truncate a system space, space '%s'
pub const ER_TRUNCATE_SYSTEM_SPACE: u8 = 137;
// Failed to dynamically load module '%.*s': %s
pub const ER_LOAD_MODULE: u8 = 138;
// Failed to allocate %u bytes for tuple: tuple is too large. Check 'vinyl_max_tuple_size' configuration option.
pub const ER_VINYL_MAX_TUPLE_SIZE: u8 = 139;
// Wrong _schema version: expected 'major.minor[.patch]'
pub const ER_WRONG_DD_VERSION: u8 = 140;
// Wrong space format (field %u): %s
pub const ER_WRONG_SPACE_FORMAT: u8 = 141;
// Failed to create sequence '%s': %s
pub const ER_CREATE_SEQUENCE: u8 = 142;
// Can't modify sequence '%s': %s
pub const ER_ALTER_SEQUENCE: u8 = 143;
// Can't drop sequence '%s': %s
pub const ER_DROP_SEQUENCE: u8 = 144;
// Sequence '%s' does not exist
pub const ER_NO_SUCH_SEQUENCE: u8 = 145;
// Sequence '%s' already exists
pub const ER_SEQUENCE_EXISTS: u8 = 146;
// Sequence '%s' has overflowed
pub const ER_SEQUENCE_OVERFLOW: u8 = 147;
// No index '%s' is defined in space '%s'
pub const ER_NO_SUCH_INDEX_NAME: u8 = 148;
// Space field '%s' is duplicate
pub const ER_SPACE_FIELD_IS_DUPLICATE: u8 = 149;
// Failed to initialize collation: %s.
pub const ER_CANT_CREATE_COLLATION: u8 = 150;
// Wrong collation options (field %u): %s
pub const ER_WRONG_COLLATION_OPTIONS: u8 = 151;
// Primary index of space '%s' can not contain nullable parts
pub const ER_NULLABLE_PRIMARY: u8 = 152;
// Field '%s' was not found in space '%s' format
pub const ER_NO_SUCH_FIELD_NAME_IN_SPACE: u8 = 153;
// Transaction has been aborted by a fiber yield
pub const ER_TRANSACTION_YIELD: u8 = 154;
// Replication group '%s' does not exist
pub const ER_NO_SUCH_GROUP: u8 = 155;
// Bind value for parameter %s is out of range for type %s
pub const ER_SQL_BIND_VALUE: u8 = 156;
// Bind value type %s for parameter %s is not supported
pub const ER_SQL_BIND_TYPE: u8 = 157;
// SQL bind parameter limit reached: %d
pub const ER_SQL_BIND_PARAMETER_MAX: u8 = 158;
// Failed to execute SQL statement: %s
pub const ER_SQL_EXECUTE: u8 = 159;
// Decimal overflow when performing operation '%c' on field %s
pub const ER_UPDATE_DECIMAL_OVERFLOW: u8 = 160;
// Parameter %s was not found in the statement
pub const ER_SQL_BIND_NOT_FOUND: u8 = 161;
// Field %s contains %s on conflict action, but %s in index parts
pub const ER_ACTION_MISMATCH: u8 = 162;
// Space declared as a view must have SQL statement
pub const ER_VIEW_MISSING_SQL: u8 = 163;
// Can not commit transaction: deferred foreign keys violations are not resolved
pub const ER_FOREIGN_KEY_CONSTRAINT: u8 = 164;
// Module '%s' does not exist
pub const ER_NO_SUCH_MODULE: u8 = 165;
// Collation '%s' does not exist
pub const ER_NO_SUCH_COLLATION: u8 = 166;
// Failed to create foreign key constraint '%s': %s
pub const ER_CREATE_FK_CONSTRAINT: u8 = 167;
// Failed to drop foreign key constraint '%s': %s
pub const ER_DROP_FK_CONSTRAINT: u8 = 168;
// Constraint '%s' does not exist in space '%s'
pub const ER_NO_SUCH_CONSTRAINT: u8 = 169;
// s constraint '%s' already exists in space '%s'
pub const ER_CONSTRAINT_EXISTS: u8 = 170;
// Type mismatch: can not convert %s to %s
pub const ER_SQL_TYPE_MISMATCH: u8 = 171;
// Rowid is overflowed: too many entries in ephemeral space
pub const ER_ROWID_OVERFLOW: u8 = 172;
// Can't drop collation %s : %s
pub const ER_DROP_COLLATION: u8 = 173;
// Illegal mix of collations
pub const ER_ILLEGAL_COLLATION_MIX: u8 = 174;
// Pragma '%s' does not exist
pub const ER_SQL_NO_SUCH_PRAGMA: u8 = 175;
// Can’t resolve field '%s'
pub const ER_SQL_CANT_RESOLVE_FIELD: u8 = 176;
// Index '%s' already exists in space '%s'
pub const ER_INDEX_EXISTS_IN_SPACE: u8 = 177;
// Inconsistent types: expected %s got %s
pub const ER_INCONSISTENT_TYPES: u8 = 178;
// Syntax error at line %d at or near position %d: %s
pub const ER_SQL_SYNTAX_WITH_POS: u8 = 179;
// Failed to parse SQL statement: parser stack limit reached
pub const ER_SQL_STACK_OVERFLOW: u8 = 180;
// Failed to expand '*' in SELECT statement without FROM clause
pub const ER_SQL_SELECT_WILDCARD: u8 = 181;
// Failed to execute an empty SQL statement
pub const ER_SQL_STATEMENT_EMPTY: u8 = 182;
// At line %d at or near position %d: keyword '%.*s' is reserved. Please use double quotes if '%.*s' is an identifier.
pub const ER_SQL_KEYWORD_IS_RESERVED: u8 = 183;
// Syntax error at line %d near '%.*s'
pub const ER_SQL_SYNTAX_NEAR_TOKEN: u8 = 184;
// At line %d at or near position %d: unrecognized token '%.*s'
pub const ER_SQL_UNKNOWN_TOKEN: u8 = 185;
// %s
pub const ER_SQL_PARSER_GENERIC: u8 = 186;
// ANALYZE statement argument %s is not a base table
pub const ER_SQL_ANALYZE_ARGUMENT: u8 = 187;
// Failed to create space '%s': space column count %d exceeds the limit (%d)
pub const ER_SQL_COLUMN_COUNT_MAX: u8 = 188;
// Hex literal %s%s length %d exceeds the supported limit (%d)
pub const ER_HEX_LITERAL_MAX: u8 = 189;
// Integer literal %s%s exceeds the supported range [-9223372036854775808, 18446744073709551615]
pub const ER_INT_LITERAL_MAX: u8 = 190;
// s %d exceeds the limit (%d)
pub const ER_SQL_PARSER_LIMIT: u8 = 191;
// s are prohibited in an index definition
pub const ER_INDEX_DEF_UNSUPPORTED: u8 = 192;
// s are prohibited in a ck constraint definition
pub const ER_CK_DEF_UNSUPPORTED: u8 = 193;
// Field %s is used as multikey in one index and as single key in another
pub const ER_MULTIKEY_INDEX_MISMATCH: u8 = 194;
// Failed to create check constraint '%s': %s
pub const ER_CREATE_CK_CONSTRAINT: u8 = 195;
// Check constraint failed '%s': %s
pub const ER_CK_CONSTRAINT_FAILED: u8 = 196;
// Unequal number of entries in row expression: left side has %u, but right side - %u
pub const ER_SQL_COLUMN_COUNT: u8 = 197;
// Failed to build a key for functional index '%s' of space '%s': %s
pub const ER_FUNC_INDEX_FUNC: u8 = 198;
// Key format doesn't match one defined in functional index '%s' of space '%s': %s
pub const ER_FUNC_INDEX_FORMAT: u8 = 199;
// Wrong functional index definition: %s
pub const ER_FUNC_INDEX_PARTS: u8 = 200;
// Field '%s' was not found in the tuple
pub const ER_NO_SUCH_FIELD_NAME: u8 = 201;
// Wrong number of arguments is passed to %s(): expected %s, got %d
pub const ER_FUNC_WRONG_ARG_COUNT: u8 = 202;
// Trying to bootstrap a local read-only instance as master
pub const ER_BOOTSTRAP_READONLY: u8 = 203;
// SQL expects exactly one argument returned from %s, got %d
pub const ER_SQL_FUNC_WRONG_RET_COUNT: u8 = 204;
// Function '%s' returned value of invalid type: expected %s got %s
pub const ER_FUNC_INVALID_RETURN_TYPE: u8 = 205;
//  line %d at or near position %d: %s
pub const ER_SQL_PARSER_GENERIC_WITH_POS: u8 = 206;
// Replica '%s' is not anonymous and cannot register.
pub const ER_REPLICA_NOT_ANON: u8 = 207;
// Couldn't find an instance to register this replica on.
pub const ER_CANNOT_REGISTER: u8 = 208;
// Session setting %s expected a value of type %s
pub const ER_SESSION_SETTING_INVALID_VALUE: u8 = 209;
// Failed to prepare SQL statement: %s
pub const ER_SQL_PREPARE: u8 = 210;
// Prepared statement with id %u does not exist
pub const ER_WRONG_QUERY_ID: u8 = 211;
// Sequence '%s' is not started
pub const ER_SEQUENCE_NOT_STARTED: u8 = 212;
// Session setting %s doesn't exist
pub const ER_NO_SUCH_SESSION_SETTING: u8 = 213;
// Found uncommitted sync transactions from other instance with id %u
pub const ER_UNCOMMITTED_FOREIGN_SYNC_TXNS: u8 = 214;
// CONFIRM message arrived for an unknown master id %d, expected %d
pub const ER_SYNC_MASTER_MISMATCH: u8 = 215;
// Quorum collection for a synchronous transaction is timed out
pub const ER_SYNC_QUORUM_TIMEOUT: u8 = 216;
// A rollback for a synchronous transaction is received
pub const ER_SYNC_ROLLBACK: u8 = 217;
// Can't create tuple: metadata size %u is too big
pub const ER_TUPLE_METADATA_IS_TOO_BIG: u8 = 218;
// %s
pub const ER_XLOG_GAP: u8 = 219;
// Can't subscribe non-anonymous replica %s until join is done
pub const ER_TOO_EARLY_SUBSCRIBE: u8 = 220;
// Can't add AUTOINCREMENT: space %s can't feature more than one AUTOINCREMENT field
pub const ER_SQL_CANT_ADD_AUTOINC: u8 = 221;
// Couldn't wait for quorum %d: %s
pub const ER_QUORUM_WAIT: u8 = 222;
// Instance with replica id %u was promoted first
pub const ER_INTERFERING_PROMOTE: u8 = 223;
// Elections were turned off
pub const ER_ELECTION_DISABLED: u8 = 224;
// Transaction was rolled back
pub const ER_TXN_ROLLBACK: u8 = 225;
// The instance is not a leader. New leader is %u
pub const ER_NOT_LEADER: u8 = 226;
// The synchronous transaction queue doesn't belong to any instance
pub const ER_SYNC_QUEUE_UNCLAIMED: u8 = 227;
// The synchronous transaction queue belongs to other instance with id %u
pub const ER_SYNC_QUEUE_FOREIGN: u8 = 228;
// Unable to process %s request in stream
pub const ER_UNABLE_TO_PROCESS_IN_STREAM: u8 = 229;
// Unable to process %s request out of stream
pub const ER_UNABLE_TO_PROCESS_OUT_OF_STREAM: u8 = 230;
// Transaction has been aborted by timeout
pub const ER_TRANSACTION_TIMEOUT: u8 = 231;
// Operation is not permitted if timer is already running
pub const ER_ACTIVE_TIMER: u8 = 232;
