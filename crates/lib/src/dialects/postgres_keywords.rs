use ahash::AHashSet;

fn priority_keyword_merge(
    args: &[&[(&'static str, &'static str)]],
) -> Vec<(&'static str, &'static str)> {
    let mut keyword_lists = args.iter().map(|x| x.to_vec()).collect::<Vec<_>>();

    if keyword_lists.len() == 1 {
        return keyword_lists.remove(0);
    }

    let mut base_list = vec![];

    while keyword_lists.len() > 1 {
        base_list = keyword_lists.remove(0);
        let priority_list = keyword_lists.remove(0);

        let keyword_set: AHashSet<&'static str> = base_list.iter().map(|x| x.0).collect();

        for item in priority_list {
            if keyword_set.contains(item.0) {
                base_list.retain(|keyword| keyword.0 != item.0);
            }
            base_list.push(item);
        }

        keyword_lists.insert(0, base_list.clone());
    }

    base_list
}

pub fn get_keywords(
    keyword_list: &[(&'static str, &'static str)],
    keyword_type: &str,
) -> Vec<&'static str> {
    keyword_list
        .iter()
        .filter_map(|&(keyword, typ)| typ.starts_with(keyword_type).then_some(keyword))
        .collect()
}

const POSTGRES_DOCS_KEYWORDS: &[(&str, &str)] = &[
    ("A", "not-keyword"),
    ("ABORT", "non-reserved"),
    ("ABS", "not-keyword"),
    ("ABSENT", "not-keyword"),
    ("ABSOLUTE", "non-reserved"),
    ("ACCESS", "non-reserved"),
    ("ACCORDING", "not-keyword"),
    ("ACOS", "not-keyword"),
    ("ACTION", "non-reserved"),
    ("ADA", "not-keyword"),
    ("ADD", "non-reserved"),
    ("ADMIN", "non-reserved"),
    ("AFTER", "non-reserved"),
    ("AGGREGATE", "non-reserved"),
    ("ALL", "reserved"),
    ("ALLOCATE", "not-keyword"),
    ("ALSO", "non-reserved"),
    ("ALTER", "non-reserved"),
    ("ALWAYS", "non-reserved"),
    ("ANALYSE", "reserved"),
    ("ANALYZE", "reserved"),
    ("AND", "reserved"),
    ("ANY", "reserved"),
    ("ARE", "not-keyword"),
    ("ARRAY", "reserved"),
    ("ARRAY_AGG", "not-keyword"),
    ("ARRAY_MAX_CARDINALITY", "not-keyword"),
    ("AS", "reserved"),
    ("ASC", "reserved"),
    ("ASENSITIVE", "not-keyword"),
    ("ASIN", "not-keyword"),
    ("ASSERTION", "non-reserved"),
    ("ASSIGNMENT", "non-reserved"),
    ("ASYMMETRIC", "reserved"),
    ("AT", "non-reserved"),
    ("ATAN", "not-keyword"),
    ("ATOMIC", "not-keyword"),
    ("ATTACH", "non-reserved"),
    ("ATTRIBUTE", "non-reserved"),
    ("ATTRIBUTES", "not-keyword"),
    ("AUTHORIZATION", "reserved-(can-be-function-or-type)"),
    ("AVG", "not-keyword"),
    ("BACKWARD", "non-reserved"),
    ("BASE64", "not-keyword"),
    ("BEFORE", "non-reserved"),
    ("BEGIN", "non-reserved"),
    ("BEGIN_FRAME", "not-keyword"),
    ("BEGIN_PARTITION", "not-keyword"),
    ("BERNOULLI", "non-reserved"),
    ("BETWEEN", "non-reserved-(cannot-be-function-or-type)"),
    ("BIGINT", "non-reserved-(cannot-be-function-or-type)"),
    ("BIGSERIAL", "non-reserved-(cannot-be-function-or-type)"),
    ("BINARY", "reserved-(can-be-function-or-type)"),
    ("BIT", "non-reserved-(cannot-be-function-or-type)"),
    ("BIT_LENGTH", "not-keyword"),
    ("BLOB", "not-keyword"),
    ("BLOCKED", "not-keyword"),
    ("BOM", "not-keyword"),
    ("BOOLEAN", "non-reserved-(cannot-be-function-or-type)"),
    ("BOOL", "non-reserved-(cannot-be-function-or-type)"),
    ("BOTH", "reserved"),
    ("BOX", "non-reserved-(cannot-be-function-or-type)"),
    ("BPCHAR", "non-reserved-(cannot-be-function-or-type)"),
    ("BREADTH", "not-keyword"),
    ("BY", "non-reserved"),
    ("BYTEA", "non-reserved-(cannot-be-function-or-type)"),
    ("C", "not-keyword"),
    ("CACHE", "non-reserved"),
    ("CALL", "non-reserved"),
    ("CALLED", "non-reserved"),
    ("CARDINALITY", "not-keyword"),
    ("CASCADE", "non-reserved"),
    ("CASCADED", "non-reserved"),
    ("CASE", "reserved"),
    ("CAST", "reserved"),
    ("CATALOG", "non-reserved"),
    ("CATALOG_NAME", "not-keyword"),
    ("CEIL", "not-keyword"),
    ("CEILING", "not-keyword"),
    ("CHAIN", "non-reserved"),
    ("CHAINING", "not-keyword"),
    ("CHAR", "non-reserved-(cannot-be-function-or-type)"),
    ("CHARACTER", "non-reserved-(cannot-be-function-or-type)"),
    ("CHARACTERISTICS", "non-reserved"),
    ("CHARACTERS", "not-keyword"),
    ("CHARACTER_LENGTH", "not-keyword"),
    ("CHARACTER_SET_CATALOG", "not-keyword"),
    ("CHARACTER_SET_NAME", "not-keyword"),
    ("CHARACTER_SET_SCHEMA", "not-keyword"),
    ("CHAR_LENGTH", "not-keyword"),
    ("CHECK", "reserved"),
    ("CHECKPOINT", "non-reserved"),
    ("CIDR", "non-reserved-(cannot-be-function-or-type)"),
    ("CIRCLE", "non-reserved-(cannot-be-function-or-type)"),
    ("CLASS", "non-reserved"),
    ("CLASSIFIER", "not-keyword"),
    ("CLASS_ORIGIN", "not-keyword"),
    ("CLOB", "not-keyword"),
    ("CLOSE", "non-reserved"),
    ("CLUSTER", "non-reserved"),
    ("COALESCE", "non-reserved-(cannot-be-function-or-type)"),
    ("COBOL", "not-keyword"),
    ("COLLATE", "reserved"),
    ("COLLATION", "non-reserved"),
    ("COLLATION_CATALOG", "not-keyword"),
    ("COLLATION_NAME", "not-keyword"),
    ("COLLATION_SCHEMA", "not-keyword"),
    ("COLLECT", "not-keyword"),
    ("COLUMN", "reserved"),
    ("COLUMNS", "non-reserved"),
    ("COLUMN_NAME", "not-keyword"),
    ("COMMAND_FUNCTION", "not-keyword"),
    ("COMMAND_FUNCTION_CODE", "not-keyword"),
    ("COMMENT", "non-reserved"),
    ("COMMENTS", "non-reserved"),
    ("COMMIT", "non-reserved"),
    ("COMMITTED", "non-reserved"),
    ("COMPRESSION", "non-reserved"),
    ("CONCURRENTLY", "reserved-(can-be-function-or-type)"),
    ("CONDITION", "not-keyword"),
    ("CONDITIONAL", "not-keyword"),
    ("CONDITION_NUMBER", "not-keyword"),
    ("CONFIGURATION", "non-reserved"),
    ("CONFLICT", "non-reserved"),
    ("CONNECT", "not-keyword"),
    ("CONNECTION", "non-reserved"),
    ("CONNECTION_NAME", "not-keyword"),
    ("CONSTRAINT", "reserved"),
    ("CONSTRAINTS", "non-reserved"),
    ("CONSTRAINT_CATALOG", "not-keyword"),
    ("CONSTRAINT_NAME", "not-keyword"),
    ("CONSTRAINT_SCHEMA", "not-keyword"),
    ("CONSTRUCTOR", "not-keyword"),
    ("CONTAINS", "not-keyword"),
    ("CONTENT", "non-reserved"),
    ("CONTINUE", "non-reserved"),
    ("CONTROL", "not-keyword"),
    ("CONVERSION", "non-reserved"),
    ("CONVERT", "not-keyword"),
    ("COPY", "non-reserved"),
    ("CORR", "not-keyword"),
    ("CORRESPONDING", "not-keyword"),
    ("COS", "not-keyword"),
    ("COSH", "not-keyword"),
    ("COST", "non-reserved"),
    ("COUNT", "not-keyword"),
    ("COVAR_POP", "not-keyword"),
    ("COVAR_SAMP", "not-keyword"),
    ("CREATE", "reserved"),
    ("CROSS", "reserved-(can-be-function-or-type)"),
    ("CSV", "non-reserved"),
    ("CUBE", "non-reserved"),
    ("CUME_DIST", "not-keyword"),
    ("CURRENT", "non-reserved"),
    ("CURRENT_CATALOG", "reserved"),
    ("CURRENT_DATE", "reserved"),
    ("CURRENT_DEFAULT_TRANSFORM_GROUP", "not-keyword"),
    ("CURRENT_PATH", "not-keyword"),
    ("CURRENT_ROLE", "reserved"),
    ("CURRENT_ROW", "not-keyword"),
    ("CURRENT_SCHEMA", "reserved-(can-be-function-or-type)"),
    ("CURRENT_TIME", "reserved"),
    ("CURRENT_TIMESTAMP", "reserved"),
    ("CURRENT_TRANSFORM_GROUP_FOR_TYPE", "not-keyword"),
    ("CURRENT_USER", "reserved"),
    ("CURSOR", "non-reserved"),
    ("CURSOR_NAME", "not-keyword"),
    ("CYCLE", "non-reserved"),
    ("DATA", "non-reserved"),
    ("DATABASE", "non-reserved"),
    ("DATALINK", "not-keyword"),
    ("DATE", "not-keyword"),
    ("DATERANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("DATETIME_INTERVAL_CODE", "not-keyword"),
    ("DATETIME_INTERVAL_PRECISION", "not-keyword"),
    ("DAY", "non-reserved"),
    ("DB", "not-keyword"),
    ("DEALLOCATE", "non-reserved"),
    ("DEC", "non-reserved-(cannot-be-function-or-type)"),
    ("DECFLOAT", "not-keyword"),
    ("DECIMAL", "non-reserved-(cannot-be-function-or-type)"),
    ("DECLARE", "non-reserved"),
    ("DEFAULT", "reserved"),
    ("DEFAULTS", "non-reserved"),
    ("DEFERRABLE", "reserved"),
    ("DEFERRED", "non-reserved"),
    ("DEFINE", "not-keyword"),
    ("DEFINED", "not-keyword"),
    ("DEFINER", "non-reserved"),
    ("DEGREE", "not-keyword"),
    ("DELETE", "non-reserved"),
    ("DELIMITER", "non-reserved"),
    ("DELIMITERS", "non-reserved"),
    ("DENSE_RANK", "not-keyword"),
    ("DEPENDS", "non-reserved"),
    ("DEPTH", "not-keyword"),
    ("DEREF", "not-keyword"),
    ("DERIVED", "not-keyword"),
    ("DESC", "reserved"),
    ("DESCRIBE", "not-keyword"),
    ("DESCRIPTOR", "not-keyword"),
    ("DETACH", "non-reserved"),
    ("DETERMINISTIC", "not-keyword"),
    ("DIAGNOSTICS", "not-keyword"),
    ("DICTIONARY", "non-reserved"),
    ("DISABLE", "non-reserved"),
    ("DISCARD", "non-reserved"),
    ("DISCONNECT", "not-keyword"),
    ("DISPATCH", "not-keyword"),
    ("DISTINCT", "reserved"),
    ("DLNEWCOPY", "not-keyword"),
    ("DLPREVIOUSCOPY", "not-keyword"),
    ("DLURLCOMPLETE", "not-keyword"),
    ("DLURLCOMPLETEONLY", "not-keyword"),
    ("DLURLCOMPLETEWRITE", "not-keyword"),
    ("DLURLPATH", "not-keyword"),
    ("DLURLPATHONLY", "not-keyword"),
    ("DLURLPATHWRITE", "not-keyword"),
    ("DLURLSCHEME", "not-keyword"),
    ("DLURLSERVER", "not-keyword"),
    ("DLVALUE", "not-keyword"),
    ("DO", "reserved"),
    ("DOCUMENT", "non-reserved"),
    ("DOMAIN", "non-reserved"),
    ("DOUBLE", "non-reserved"),
    ("DROP", "non-reserved"),
    ("DYNAMIC", "not-keyword"),
    ("DYNAMIC_FUNCTION", "not-keyword"),
    ("DYNAMIC_FUNCTION_CODE", "not-keyword"),
    ("EACH", "non-reserved"),
    ("ELEMENT", "not-keyword"),
    ("ELSE", "reserved"),
    ("EMPTY", "not-keyword"),
    ("ENABLE", "non-reserved"),
    ("ENCODING", "non-reserved"),
    ("ENCRYPTED", "non-reserved"),
    ("END", "reserved"),
    ("END-EXEC", "not-keyword"),
    ("END_FRAME", "not-keyword"),
    ("END_PARTITION", "not-keyword"),
    ("ENFORCED", "not-keyword"),
    ("ENUM", "non-reserved"),
    ("EQUALS", "not-keyword"),
    ("ERROR", "not-keyword"),
    ("ESCAPE", "non-reserved"),
    ("EVENT", "non-reserved"),
    ("EVERY", "not-keyword"),
    ("EXCEPT", "reserved"),
    ("EXCEPTION", "not-keyword"),
    ("EXCLUDE", "non-reserved"),
    ("EXCLUDING", "non-reserved"),
    ("EXCLUSIVE", "non-reserved"),
    ("EXEC", "not-keyword"),
    ("EXECUTE", "non-reserved"),
    ("EXISTS", "non-reserved-(cannot-be-function-or-type)"),
    ("EXP", "not-keyword"),
    ("EXPLAIN", "non-reserved"),
    ("EXPRESSION", "non-reserved"),
    ("EXTENSION", "non-reserved"),
    ("EXTERNAL", "non-reserved"),
    ("EXTRACT", "non-reserved-(cannot-be-function-or-type)"),
    ("FALSE", "reserved"),
    ("FAMILY", "non-reserved"),
    ("FETCH", "reserved"),
    ("FILE", "not-keyword"),
    ("FILTER", "non-reserved"),
    ("FINAL", "not-keyword"),
    ("FINALIZE", "non-reserved"),
    ("FINISH", "not-keyword"),
    ("FIRST", "non-reserved"),
    ("FIRST_VALUE", "not-keyword"),
    ("FLAG", "not-keyword"),
    ("FLOAT", "non-reserved-(cannot-be-function-or-type)"),
    ("FLOOR", "not-keyword"),
    ("FOLLOWING", "non-reserved"),
    ("FOR", "reserved"),
    ("FORCE", "non-reserved"),
    ("FOREIGN", "reserved"),
    ("FORMAT", "not-keyword"),
    ("FORTRAN", "not-keyword"),
    ("FORWARD", "non-reserved"),
    ("FOUND", "not-keyword"),
    ("FRAME_ROW", "not-keyword"),
    ("FREE", "not-keyword"),
    ("FREEZE", "reserved-(can-be-function-or-type)"),
    ("FROM", "reserved"),
    ("FS", "not-keyword"),
    ("FULFILL", "not-keyword"),
    ("FULL", "reserved-(can-be-function-or-type)"),
    ("FUNCTION", "non-reserved"),
    ("FUNCTIONS", "non-reserved"),
    ("FUSION", "not-keyword"),
    ("G", "not-keyword"),
    ("GENERAL", "not-keyword"),
    ("GENERATED", "non-reserved"),
    ("GET", "not-keyword"),
    ("GLOBAL", "non-reserved"),
    ("GO", "not-keyword"),
    ("GOTO", "not-keyword"),
    ("GRANT", "reserved"),
    ("GRANTED", "non-reserved"),
    ("GREATEST", "non-reserved-(cannot-be-function-or-type)"),
    ("GROUP", "reserved"),
    ("GROUPING", "non-reserved-(cannot-be-function-or-type)"),
    ("GROUPS", "non-reserved"),
    ("HANDLER", "non-reserved"),
    ("HAVING", "reserved"),
    ("HEADER", "non-reserved"),
    ("HEX", "not-keyword"),
    ("HIERARCHY", "not-keyword"),
    ("HOLD", "non-reserved"),
    ("HOUR", "non-reserved"),
    ("ID", "not-keyword"),
    ("IDENTITY", "non-reserved"),
    ("IF", "non-reserved"),
    ("IGNORE", "not-keyword"),
    ("ILIKE", "reserved-(can-be-function-or-type)"),
    ("IMMEDIATE", "non-reserved"),
    ("IMMEDIATELY", "not-keyword"),
    ("IMMUTABLE", "non-reserved"),
    ("IMPLEMENTATION", "not-keyword"),
    ("IMPLICIT", "non-reserved"),
    ("IMPORT", "non-reserved"),
    ("IN", "reserved"),
    ("INCLUDE", "non-reserved"),
    ("INCLUDING", "non-reserved"),
    ("INCREMENT", "non-reserved"),
    ("INDENT", "not-keyword"),
    ("INDEX", "non-reserved"),
    ("INDEXES", "non-reserved"),
    ("INET", "non-reserved-(cannot-be-function-or-type)"),
    ("INDICATOR", "not-keyword"),
    ("INHERIT", "non-reserved"),
    ("INHERITS", "non-reserved"),
    ("INITIAL", "not-keyword"),
    ("INITIALLY", "reserved"),
    ("INLINE", "non-reserved"),
    ("INNER", "reserved-(can-be-function-or-type)"),
    ("INOUT", "non-reserved-(cannot-be-function-or-type)"),
    ("INPUT", "non-reserved"),
    ("INSENSITIVE", "non-reserved"),
    ("INSERT", "non-reserved"),
    ("INSTANCE", "not-keyword"),
    ("INSTANTIABLE", "not-keyword"),
    ("INSTEAD", "non-reserved"),
    ("INT", "non-reserved-(cannot-be-function-or-type)"),
    ("INT2", "non-reserved-(cannot-be-function-or-type)"),
    ("INT4", "non-reserved-(cannot-be-function-or-type)"),
    ("INT4RANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("INT8", "non-reserved-(cannot-be-function-or-type)"),
    ("INT8RANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("INTEGER", "non-reserved-(cannot-be-function-or-type)"),
    ("INTEGRITY", "not-keyword"),
    ("INTERSECT", "reserved"),
    ("INTERSECTION", "not-keyword"),
    ("INTERVAL", "non-reserved-(cannot-be-function-or-type)"),
    ("INTO", "reserved"),
    ("INVOKER", "non-reserved"),
    ("IS", "reserved-(can-be-function-or-type)"),
    ("ISNULL", "reserved-(can-be-function-or-type)"),
    ("ISOLATION", "non-reserved"),
    ("JOIN", "reserved-(can-be-function-or-type)"),
    ("JSON", "not-keyword"),
    ("JSON_ARRAY", "not-keyword"),
    ("JSON_ARRAYAGG", "not-keyword"),
    ("JSON_EXISTS", "not-keyword"),
    ("JSON_OBJECT", "not-keyword"),
    ("JSON_OBJECTAGG", "not-keyword"),
    ("JSON_QUERY", "not-keyword"),
    ("JSON_TABLE", "not-keyword"),
    ("JSON_TABLE_PRIMITIVE", "not-keyword"),
    ("JSON_VALUE", "not-keyword"),
    ("JSONB", "non-reserved-(cannot-be-function-or-type)"),
    ("K", "not-keyword"),
    ("KEEP", "not-keyword"),
    ("KEY", "non-reserved"),
    ("KEYS", "not-keyword"),
    ("KEY_MEMBER", "not-keyword"),
    ("KEY_TYPE", "not-keyword"),
    ("LABEL", "non-reserved"),
    ("LAG", "not-keyword"),
    ("LANGUAGE", "non-reserved"),
    ("LARGE", "non-reserved"),
    ("LAST", "non-reserved"),
    ("LAST_VALUE", "not-keyword"),
    ("LATERAL", "reserved"),
    ("LEAD", "not-keyword"),
    ("LEADING", "reserved"),
    ("LEAKPROOF", "non-reserved"),
    ("LEAST", "non-reserved-(cannot-be-function-or-type)"),
    ("LEFT", "reserved-(can-be-function-or-type)"),
    ("LENGTH", "not-keyword"),
    ("LEVEL", "non-reserved"),
    ("LIBRARY", "not-keyword"),
    ("LIKE", "reserved-(can-be-function-or-type)"),
    ("LIKE_REGEX", "not-keyword"),
    ("LIMIT", "reserved"),
    ("LINE", "non-reserved-(cannot-be-function-or-type)"),
    ("LINK", "not-keyword"),
    ("LISTAGG", "not-keyword"),
    ("LISTEN", "non-reserved"),
    ("LN", "not-keyword"),
    ("LOAD", "non-reserved"),
    ("LOCAL", "non-reserved"),
    ("LOCALTIME", "reserved"),
    ("LOCALTIMESTAMP", "reserved"),
    ("LOCATION", "non-reserved"),
    ("LOCATOR", "not-keyword"),
    ("LOCK", "non-reserved"),
    ("LOCKED", "non-reserved"),
    ("LOG", "not-keyword"),
    ("LOG10", "not-keyword"),
    ("LOGGED", "non-reserved"),
    ("LOWER", "not-keyword"),
    ("LSEG", "non-reserved-(cannot-be-function-or-type)"),
    ("M", "not-keyword"),
    ("MACADDR", "non-reserved-(cannot-be-function-or-type)"),
    ("MACADDR8", "non-reserved-(cannot-be-function-or-type)"),
    ("MAP", "not-keyword"),
    ("MAPPING", "non-reserved"),
    ("MATCH", "non-reserved"),
    ("MATCHED", "non-reserved"),
    ("MATCHES", "not-keyword"),
    ("MATCH_NUMBER", "not-keyword"),
    ("MATCH_RECOGNIZE", "not-keyword"),
    ("MATERIALIZED", "non-reserved"),
    ("MAX", "not-keyword"),
    ("MAXVALUE", "non-reserved"),
    ("MEASURES", "not-keyword"),
    ("MEMBER", "not-keyword"),
    ("MERGE", "non-reserved"),
    ("MESSAGE_LENGTH", "not-keyword"),
    ("MESSAGE_OCTET_LENGTH", "not-keyword"),
    ("MESSAGE_TEXT", "not-keyword"),
    ("METHOD", "non-reserved"),
    ("MIN", "not-keyword"),
    ("MINUTE", "non-reserved"),
    ("MINVALUE", "non-reserved"),
    ("MOD", "not-keyword"),
    ("MODE", "non-reserved"),
    ("MODIFIES", "not-keyword"),
    ("MODULE", "not-keyword"),
    ("MONEY", "non-reserved-(cannot-be-function-or-type)"),
    ("MONTH", "non-reserved"),
    ("MORE", "not-keyword"),
    ("MOVE", "non-reserved"),
    ("MULTISET", "not-keyword"),
    ("MUMPS", "not-keyword"),
    ("NAME", "non-reserved"),
    ("NAMES", "non-reserved"),
    ("NAMESPACE", "not-keyword"),
    ("NATIONAL", "non-reserved-(cannot-be-function-or-type)"),
    ("NATURAL", "reserved-(can-be-function-or-type)"),
    ("NCHAR", "non-reserved-(cannot-be-function-or-type)"),
    ("NCLOB", "not-keyword"),
    ("NESTED", "not-keyword"),
    ("NESTING", "not-keyword"),
    ("NEW", "non-reserved"),
    ("NEXT", "non-reserved"),
    ("NFC", "non-reserved"),
    ("NFD", "non-reserved"),
    ("NFKC", "non-reserved"),
    ("NFKD", "non-reserved"),
    ("NIL", "not-keyword"),
    ("NO", "non-reserved"),
    ("NONE", "non-reserved-(cannot-be-function-or-type)"),
    ("NORMALIZE", "non-reserved-(cannot-be-function-or-type)"),
    ("NORMALIZED", "non-reserved"),
    ("NOT", "reserved"),
    ("NOTHING", "non-reserved"),
    ("NOTIFY", "non-reserved"),
    ("NOTNULL", "reserved-(can-be-function-or-type)"),
    ("NOWAIT", "non-reserved"),
    ("NTH_VALUE", "not-keyword"),
    ("NTILE", "not-keyword"),
    ("NULL", "reserved"),
    ("NULLABLE", "not-keyword"),
    ("NULLIF", "non-reserved-(cannot-be-function-or-type)"),
    ("NULLS", "non-reserved"),
    ("NUMBER", "not-keyword"),
    ("NUMERIC", "non-reserved-(cannot-be-function-or-type)"),
    ("NUMRANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("OBJECT", "non-reserved"),
    ("OCCURRENCES_REGEX", "not-keyword"),
    ("OCTETS", "not-keyword"),
    ("OCTET_LENGTH", "not-keyword"),
    ("OF", "non-reserved"),
    ("OFF", "non-reserved"),
    ("OFFSET", "reserved"),
    ("OIDS", "non-reserved"),
    ("OLD", "non-reserved"),
    ("OMIT", "not-keyword"),
    ("ON", "reserved"),
    ("ONE", "not-keyword"),
    ("ONLY", "reserved"),
    ("OPEN", "not-keyword"),
    ("OPERATOR", "non-reserved"),
    ("OPTION", "non-reserved"),
    ("OPTIONS", "non-reserved"),
    ("OR", "reserved"),
    ("ORDER", "reserved"),
    ("ORDERING", "not-keyword"),
    ("ORDINALITY", "non-reserved"),
    ("OTHERS", "non-reserved"),
    ("OUT", "non-reserved-(cannot-be-function-or-type)"),
    ("OUTER", "reserved-(can-be-function-or-type)"),
    ("OUTPUT", "not-keyword"),
    ("OVER", "non-reserved"),
    ("OVERFLOW", "not-keyword"),
    ("OVERLAPS", "reserved-(can-be-function-or-type)"),
    ("OVERLAY", "non-reserved-(cannot-be-function-or-type)"),
    ("OVERRIDING", "non-reserved"),
    ("OWNED", "non-reserved"),
    ("OWNER", "non-reserved"),
    ("P", "not-keyword"),
    ("PAD", "not-keyword"),
    ("PARALLEL", "non-reserved"),
    ("PARAMETER", "not-keyword"),
    ("PARAMETER_MODE", "not-keyword"),
    ("PARAMETER_NAME", "not-keyword"),
    ("PARAMETER_ORDINAL_POSITION", "not-keyword"),
    ("PARAMETER_SPECIFIC_CATALOG", "not-keyword"),
    ("PARAMETER_SPECIFIC_NAME", "not-keyword"),
    ("PARAMETER_SPECIFIC_SCHEMA", "not-keyword"),
    ("PARSER", "non-reserved"),
    ("PARTIAL", "non-reserved"),
    ("PARTITION", "non-reserved"),
    ("PASCAL", "not-keyword"),
    ("PASS", "not-keyword"),
    ("PASSING", "non-reserved"),
    ("PASSTHROUGH", "not-keyword"),
    ("PASSWORD", "non-reserved"),
    ("PAST", "not-keyword"),
    ("PATH", "non-reserved-(cannot-be-function-or-type)"),
    ("PATTERN", "not-keyword"),
    ("PER", "not-keyword"),
    ("PERCENT", "not-keyword"),
    ("PERCENTILE_CONT", "not-keyword"),
    ("PERCENTILE_DISC", "not-keyword"),
    ("PERCENT_RANK", "not-keyword"),
    ("PERIOD", "not-keyword"),
    ("PERMISSION", "not-keyword"),
    ("PERMISSIVE", "non-reserved"),
    ("PERMUTE", "not-keyword"),
    ("PG_LSN", "non-reserved-(cannot-be-function-or-type)"),
    ("PLACING", "reserved"),
    ("PLAN", "not-keyword"),
    ("PLANS", "non-reserved"),
    ("PLI", "not-keyword"),
    ("POINT", "non-reserved-(cannot-be-function-or-type)"),
    ("POLICY", "non-reserved"),
    ("POLYGON", "non-reserved-(cannot-be-function-or-type)"),
    ("PORTION", "not-keyword"),
    ("POSITION", "non-reserved-(cannot-be-function-or-type)"),
    ("POSITION_REGEX", "not-keyword"),
    ("POWER", "not-keyword"),
    ("PRECEDES", "not-keyword"),
    ("PRECEDING", "non-reserved"),
    ("PRECISION", "non-reserved-(cannot-be-function-or-type)"),
    ("PREPARE", "non-reserved"),
    ("PREPARED", "non-reserved"),
    ("PRESERVE", "non-reserved"),
    ("PRIMARY", "reserved"),
    ("PRIOR", "non-reserved"),
    ("PRIVATE", "not-keyword"),
    ("PRIVILEGES", "non-reserved"),
    ("PROCEDURAL", "non-reserved"),
    ("PROCEDURE", "non-reserved"),
    ("PROCEDURES", "non-reserved"),
    ("PROGRAM", "non-reserved"),
    ("PRUNE", "not-keyword"),
    ("PTF", "not-keyword"),
    ("PUBLIC", "not-keyword"),
    ("PUBLICATION", "non-reserved"),
    ("QUOTE", "non-reserved"),
    ("QUOTES", "not-keyword"),
    ("RANGE", "non-reserved"),
    ("RANK", "not-keyword"),
    ("READ", "non-reserved"),
    ("READS", "not-keyword"),
    ("REAL", "non-reserved-(cannot-be-function-or-type)"),
    ("REASSIGN", "non-reserved"),
    ("RECHECK", "non-reserved"),
    ("RECOVERY", "not-keyword"),
    ("RECURSIVE", "non-reserved"),
    ("REF", "non-reserved"),
    ("REFERENCES", "reserved"),
    ("REFERENCING", "non-reserved"),
    ("REFRESH", "non-reserved"),
    ("REGR_AVGX", "not-keyword"),
    ("REGR_AVGY", "not-keyword"),
    ("REGR_COUNT", "not-keyword"),
    ("REGR_INTERCEPT", "not-keyword"),
    ("REGR_R2", "not-keyword"),
    ("REGR_SLOPE", "not-keyword"),
    ("REGR_SXX", "not-keyword"),
    ("REGR_SXY", "not-keyword"),
    ("REGR_SYY", "not-keyword"),
    ("REINDEX", "non-reserved"),
    ("RELATIVE", "non-reserved"),
    ("RELEASE", "non-reserved"),
    ("RENAME", "non-reserved"),
    ("REPEATABLE", "non-reserved"),
    ("REPLACE", "non-reserved"),
    ("REPLICA", "non-reserved"),
    ("REQUIRING", "not-keyword"),
    ("RESET", "non-reserved"),
    ("RESPECT", "not-keyword"),
    ("RESTART", "non-reserved"),
    ("RESTORE", "not-keyword"),
    ("RESTRICT", "non-reserved"),
    ("RESTRICTIVE", "non-reserved"),
    ("RESULT", "not-keyword"),
    ("RETURN", "not-keyword"),
    ("RETURNED_CARDINALITY", "not-keyword"),
    ("RETURNED_LENGTH", "not-keyword"),
    ("RETURNED_OCTET_LENGTH", "not-keyword"),
    ("RETURNED_SQLSTATE", "not-keyword"),
    ("RETURNING", "reserved"),
    ("RETURNS", "non-reserved"),
    ("REVOKE", "non-reserved"),
    ("RIGHT", "reserved-(can-be-function-or-type)"),
    ("ROLE", "non-reserved"),
    ("ROLLBACK", "non-reserved"),
    ("ROLLUP", "non-reserved"),
    ("ROUTINE", "non-reserved"),
    ("ROUTINES", "non-reserved"),
    ("ROUTINE_CATALOG", "not-keyword"),
    ("ROUTINE_NAME", "not-keyword"),
    ("ROUTINE_SCHEMA", "not-keyword"),
    ("ROW", "non-reserved-(cannot-be-function-or-type)"),
    ("ROWS", "non-reserved"),
    ("ROW_COUNT", "not-keyword"),
    ("ROW_NUMBER", "not-keyword"),
    ("RULE", "non-reserved"),
    ("RUNNING", "not-keyword"),
    ("SAVEPOINT", "non-reserved"),
    ("SCALAR", "not-keyword"),
    ("SCALE", "not-keyword"),
    ("SCHEMA", "non-reserved"),
    ("SCHEMAS", "non-reserved"),
    ("SCHEMA_NAME", "not-keyword"),
    ("SCOPE", "not-keyword"),
    ("SCOPE_CATALOG", "not-keyword"),
    ("SCOPE_NAME", "not-keyword"),
    ("SCOPE_SCHEMA", "not-keyword"),
    ("SCROLL", "non-reserved"),
    ("SEARCH", "non-reserved"),
    ("SECOND", "non-reserved"),
    ("SECTION", "not-keyword"),
    ("SECURITY", "non-reserved"),
    ("SEEK", "not-keyword"),
    ("SELECT", "reserved"),
    ("SELECTIVE", "not-keyword"),
    ("SELF", "not-keyword"),
    ("SENSITIVE", "not-keyword"),
    ("SEQUENCE", "non-reserved"),
    ("SEQUENCES", "non-reserved"),
    ("SERIAL", "non-reserved-(cannot-be-function-or-type)"),
    ("SERIAL2", "non-reserved-(cannot-be-function-or-type)"),
    ("SERIAL4", "non-reserved-(cannot-be-function-or-type)"),
    ("SERIAL8", "non-reserved-(cannot-be-function-or-type)"),
    ("SERIALIZABLE", "non-reserved"),
    ("SERVER", "non-reserved"),
    ("SERVER_NAME", "not-keyword"),
    ("SESSION", "non-reserved"),
    ("SESSION_USER", "reserved"),
    ("SET", "non-reserved"),
    ("SETOF", "non-reserved-(cannot-be-function-or-type)"),
    ("SETS", "non-reserved"),
    ("SHARE", "non-reserved"),
    ("SHOW", "non-reserved"),
    ("SIMILAR", "reserved-(can-be-function-or-type)"),
    ("SIMPLE", "non-reserved"),
    ("SIN", "not-keyword"),
    ("SINH", "not-keyword"),
    ("SIZE", "not-keyword"),
    ("SKIP", "non-reserved"),
    ("SMALLINT", "non-reserved-(cannot-be-function-or-type)"),
    ("SMALLSERIAL", "non-reserved-(cannot-be-function-or-type)"),
    ("SNAPSHOT", "non-reserved"),
    ("SOME", "reserved"),
    ("SOURCE", "not-keyword"),
    ("SPACE", "not-keyword"),
    ("SPECIFIC", "not-keyword"),
    ("SPECIFICTYPE", "not-keyword"),
    ("SPECIFIC_NAME", "not-keyword"),
    ("SQL", "non-reserved"),
    ("SQLCODE", "not-keyword"),
    ("SQLERROR", "not-keyword"),
    ("SQLEXCEPTION", "not-keyword"),
    ("SQLSTATE", "not-keyword"),
    ("SQLWARNING", "not-keyword"),
    ("SQRT", "not-keyword"),
    ("STABLE", "non-reserved"),
    ("STANDALONE", "non-reserved"),
    ("START", "non-reserved"),
    ("STATE", "not-keyword"),
    ("STATEMENT", "non-reserved"),
    ("STATIC", "not-keyword"),
    ("STATISTICS", "non-reserved"),
    ("STDDEV_POP", "not-keyword"),
    ("STDDEV_SAMP", "not-keyword"),
    ("STDIN", "non-reserved"),
    ("STDOUT", "non-reserved"),
    ("STORAGE", "non-reserved"),
    ("STORED", "non-reserved"),
    ("STRICT", "non-reserved"),
    ("STRING", "not-keyword"),
    ("STRIP", "non-reserved"),
    ("STRUCTURE", "not-keyword"),
    ("STYLE", "not-keyword"),
    ("SUBCLASS_ORIGIN", "not-keyword"),
    ("SUBMULTISET", "not-keyword"),
    ("SUBSCRIPTION", "non-reserved"),
    ("SUBSET", "not-keyword"),
    ("SUBSTRING", "non-reserved-(cannot-be-function-or-type)"),
    ("SUBSTRING_REGEX", "not-keyword"),
    ("SUCCEEDS", "not-keyword"),
    ("SUM", "not-keyword"),
    ("SUPPORT", "non-reserved"),
    ("SYMMETRIC", "reserved"),
    ("SYSID", "non-reserved"),
    ("SYSTEM", "non-reserved"),
    ("SYSTEM_TIME", "not-keyword"),
    ("SYSTEM_USER", "not-keyword"),
    ("T", "not-keyword"),
    ("TABLE", "non-reserved"),
    ("TABLES", "non-reserved"),
    ("TABLESAMPLE", "reserved-(can-be-function-or-type)"),
    ("TABLESPACE", "non-reserved"),
    ("TABLE_NAME", "not-keyword"),
    ("TAN", "not-keyword"),
    ("TANH", "not-keyword"),
    ("TEMP", "non-reserved"),
    ("TEMPLATE", "non-reserved"),
    ("TEMPORARY", "non-reserved"),
    ("TEXT", "non-reserved"),
    ("THEN", "reserved"),
    ("THROUGH", "not-keyword"),
    ("TIES", "non-reserved"),
    ("TIME", "non-reserved-(cannot-be-function-or-type)"),
    ("TIMESTAMP", "non-reserved-(cannot-be-function-or-type)"),
    ("TIMEZONE_HOUR", "not-keyword"),
    ("TIMEZONE_MINUTE", "not-keyword"),
    ("TO", "reserved"),
    ("TOKEN", "not-keyword"),
    ("TOP_LEVEL_COUNT", "not-keyword"),
    ("TRAILING", "reserved"),
    ("TRANSACTION", "non-reserved"),
    ("TRANSACTIONS_COMMITTED", "not-keyword"),
    ("TRANSACTIONS_ROLLED_BACK", "not-keyword"),
    ("TRANSACTION_ACTIVE", "not-keyword"),
    ("TRANSFORM", "non-reserved"),
    ("TRANSFORMS", "not-keyword"),
    ("TRANSLATE", "not-keyword"),
    ("TRANSLATE_REGEX", "not-keyword"),
    ("TRANSLATION", "not-keyword"),
    ("TREAT", "non-reserved-(cannot-be-function-or-type)"),
    ("TRIGGER", "non-reserved"),
    ("TRIGGER_CATALOG", "not-keyword"),
    ("TRIGGER_NAME", "not-keyword"),
    ("TRIGGER_SCHEMA", "not-keyword"),
    ("TRIM", "non-reserved-(cannot-be-function-or-type)"),
    ("TRIM_ARRAY", "not-keyword"),
    ("TRUE", "reserved"),
    ("TRUNCATE", "non-reserved"),
    ("TRUSTED", "non-reserved"),
    ("TSQUERY", "non-reserved-(cannot-be-function-or-type)"),
    ("TSRANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("TSTZRANGE", "non-reserved-(cannot-be-function-or-type)"),
    ("TSVECTOR", "non-reserved-(cannot-be-function-or-type)"),
    ("TYPE", "non-reserved"),
    ("TYPES", "non-reserved"),
    ("UESCAPE", "non-reserved"),
    ("UNBOUNDED", "non-reserved"),
    ("UNCOMMITTED", "non-reserved"),
    ("UNCONDITIONAL", "not-keyword"),
    ("UNDER", "not-keyword"),
    ("UNENCRYPTED", "non-reserved"),
    ("UNION", "reserved"),
    ("UNIQUE", "reserved"),
    ("UNKNOWN", "non-reserved"),
    ("UNLINK", "not-keyword"),
    ("UNLISTEN", "non-reserved"),
    ("UNLOGGED", "non-reserved"),
    ("UNMATCHED", "not-keyword"),
    ("UNNAMED", "not-keyword"),
    ("UNNEST", "not-keyword"),
    ("UNTIL", "non-reserved"),
    ("UNTYPED", "not-keyword"),
    ("UPDATE", "non-reserved"),
    ("UPPER", "not-keyword"),
    ("URI", "not-keyword"),
    ("USAGE", "not-keyword"),
    ("USER", "reserved"),
    ("USER_DEFINED_TYPE_CATALOG", "not-keyword"),
    ("USER_DEFINED_TYPE_CODE", "not-keyword"),
    ("USER_DEFINED_TYPE_NAME", "not-keyword"),
    ("USER_DEFINED_TYPE_SCHEMA", "not-keyword"),
    ("USING", "reserved"),
    ("UTF16", "not-keyword"),
    ("UTF32", "not-keyword"),
    ("UTF8", "not-keyword"),
    ("UUID", "non-reserved-(cannot-be-function-or-type)"),
    ("VACUUM", "non-reserved"),
    ("VALID", "non-reserved"),
    ("VALIDATE", "non-reserved"),
    ("VALIDATOR", "non-reserved"),
    ("VALUE", "non-reserved"),
    ("VALUES", "non-reserved-(cannot-be-function-or-type)"),
    ("VALUE_OF", "not-keyword"),
    ("VARBINARY", "not-keyword"),
    ("VARCHAR", "non-reserved-(cannot-be-function-or-type)"),
    ("VARIADIC", "reserved"),
    ("VARYING", "non-reserved"),
    ("VAR_POP", "not-keyword"),
    ("VAR_SAMP", "not-keyword"),
    ("VERBOSE", "reserved-(can-be-function-or-type)"),
    ("VERSION", "non-reserved"),
    ("VERSIONING", "not-keyword"),
    ("VIEW", "non-reserved"),
    ("VIEWS", "non-reserved"),
    ("VOLATILE", "non-reserved"),
    ("WHEN", "reserved"),
    ("WHENEVER", "not-keyword"),
    ("WHERE", "reserved"),
    ("WHITESPACE", "non-reserved"),
    ("WIDTH_BUCKET", "not-keyword"),
    ("WINDOW", "reserved"),
    ("WITH", "reserved"),
    ("WITHIN", "non-reserved"),
    ("WITHOUT", "non-reserved"),
    ("WORK", "non-reserved"),
    ("WRAPPER", "non-reserved"),
    ("WRITE", "non-reserved"),
    ("XML", "non-reserved"),
    ("XMLAGG", "not-keyword"),
    ("XMLATTRIBUTES", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLBINARY", "not-keyword"),
    ("XMLCAST", "not-keyword"),
    ("XMLCOMMENT", "not-keyword"),
    ("XMLCONCAT", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLDECLARATION", "not-keyword"),
    ("XMLDOCUMENT", "not-keyword"),
    ("XMLELEMENT", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLEXISTS", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLFOREST", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLITERATE", "not-keyword"),
    ("XMLNAMESPACES", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLPARSE", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLPI", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLQUERY", "not-keyword"),
    ("XMLROOT", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLSCHEMA", "not-keyword"),
    ("XMLSERIALIZE", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLTABLE", "non-reserved-(cannot-be-function-or-type)"),
    ("XMLTEXT", "not-keyword"),
    ("XMLVALIDATE", "not-keyword"),
    ("YEAR", "non-reserved"),
    ("YES", "non-reserved"),
    ("ZONE", "non-reserved"),
];

const POSTGRES_NONDOCS_KEYWORDS: &[(&str, &str)] = &[
    ("ALLOW_CONNECTIONS", "non-reserved"),
    ("BREADTH", "non-reserved"),
    ("BUFFERS", "non-reserved"),
    ("BYPASSRLS", "non-reserved"),
    ("CONNECT", "reserved"),
    ("COSTS", "non-reserved"),
    ("CURRENT_USER", "non-reserved"),
    ("CREATEDB", "non-reserved"),
    ("CREATEROLE", "non-reserved"),
    ("DATE", "non-reserved"),
    ("DEPTH", "non-reserved"),
    ("DESCRIBE", "non-reserved"),
    ("DETERMINISTIC", "non-reserved"),
    ("DISABLE_PAGE_SKIPPING", "non-reserved"),
    ("EXECUTION", "not-keyword"),
    ("EXTENDED", "non-reserved"),
    ("FILE", "non-reserved"),
    ("FORCE_NOT_NULL", "non-reserved"),
    ("FORCE_NULL", "non-reserved"),
    ("FORCE_QUOTE", "non-reserved"),
    ("FORMAT", "non-reserved"),
    ("HASH", "non-reserved"),
    ("ICU", "non-reserved"),
    ("IGNORE", "non-reserved"),
    ("INDEX_CLEANUP", "non-reserved"),
    ("IS_TEMPLATE", "non-reserved"),
    ("JSON", "non-reserved"),
    ("LC_COLLATE", "non-reserved"),
    ("LC_CTYPE", "non-reserved"),
    ("LIBC", "non-reserved"),
    ("LIST", "non-reserved"),
    ("LOGIN", "non-reserved"),
    ("LOCALE", "non-reserved"),
    ("MAIN", "non-reserved"),
    ("MODULUS", "non-reserved"),
    ("NOBYPASSRLS", "non-reserved"),
    ("NOCREATEDB", "non-reserved"),
    ("NOCREATEROLE", "non-reserved"),
    ("NOINHERIT", "non-reserved"),
    ("NOLOGIN", "non-reserved"),
    ("NOREPLICATION", "non-reserved"),
    ("NOSUPERUSER", "non-reserved"),
    ("PLAIN", "non-reserved"),
    ("PROCESS_TOAST", "non-reserved"),
    ("PROVIDER", "non-reserved"),
    ("PUBLIC", "non-reserved"),
    ("REMAINDER", "non-reserved"),
    ("REPLICATION", "non-reserved"),
    ("RESPECT", "non-reserved"),
    ("RESTRICTED", "non-reserved"),
    ("SAFE", "non-reserved"),
    ("SETTINGS", "non-reserved"),
    ("SKIP_LOCKED", "non-reserved"),
    ("SUMMARY", "non-reserved"),
    ("SUPERUSER", "non-reserved"),
    ("TIMETZ", "non-reserved"),
    ("TIMESTAMPTZ", "non-reserved"),
    ("TIMING", "non-reserved"),
    ("UNSAFE", "non-reserved"),
    ("USAGE", "non-reserved"),
    ("WAL", "non-reserved"),
];

pub const POSTGRES_POSTGIS_DATATYPE_KEYWORDS: &[(&str, &str)] = &[
    ("POINT", "non-reserved"),
    ("LINESTRING", "non-reserved"),
    ("POLYGON", "non-reserved"),
    ("MULTIPOINT", "non-reserved"),
    ("MULTILINESTRING", "non-reserved"),
    ("MULTIPOLYGON", "non-reserved"),
    ("GEOMETRYCOLLECTION", "non-reserved"),
    ("POINTZ", "non-reserved"),
    ("LINESTRINGZ", "non-reserved"),
    ("POLYGONZ", "non-reserved"),
    ("MULTIPOINTZ", "non-reserved"),
    ("MULTILINESTRINGZ", "non-reserved"),
    ("MULTIPOLYGONZ", "non-reserved"),
    ("GEOMETRYCOLLECTIONZ", "non-reserved"),
    ("POINTM", "non-reserved"),
    ("LINESTRINGM", "non-reserved"),
    ("POLYGONM", "non-reserved"),
    ("MULTIPOINTM", "non-reserved"),
    ("MULTILINESTRINGM", "non-reserved"),
    ("MULTIPOLYGONM", "non-reserved"),
    ("GEOMETRYCOLLECTIONM", "non-reserved"),
    ("POINTZM", "non-reserved"),
    ("LINESTRINGZM", "non-reserved"),
    ("POLYGONZM", "non-reserved"),
    ("MULTIPOINTZM", "non-reserved"),
    ("MULTILINESTRINGZM", "non-reserved"),
    ("MULTIPOLYGONZM", "non-reserved"),
    ("GEOMETRYCOLLECTIONZM", "non-reserved"),
    ("CIRCULARSTRING", "non-reserved"),
    ("COMPOUNDCURVE", "non-reserved"),
    ("CURVEPOLYGON", "non-reserved"),
    ("MULTICURVE", "non-reserved"),
    ("MULTISURFACE", "non-reserved"),
    ("POLYHEDRALSURFACE", "non-reserved"),
    ("TRIANGLE", "non-reserved"),
    ("TIN", "non-reserved"),
];

const POSTGRES_POSTGIS_OTHER_KEYWORDS: &[(&str, &str)] =
    &[("GEOMETRY", "non-reserved"), ("GEOGRAPHY", "non-reserved"), ("EMPTY", "non-reserved")];

pub fn postgres_keywords() -> Vec<(&'static str, &'static str)> {
    priority_keyword_merge(&[
        POSTGRES_DOCS_KEYWORDS,
        POSTGRES_NONDOCS_KEYWORDS,
        POSTGRES_POSTGIS_DATATYPE_KEYWORDS,
        POSTGRES_POSTGIS_OTHER_KEYWORDS,
    ])
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::priority_keyword_merge;
    use crate::dialects::postgres_keywords::get_keywords;

    fn sorted<T: Ord>(mut xs: Vec<T>) -> Vec<T> {
        xs.sort();
        xs
    }

    #[test]
    fn test_priority_keyword_merge() {
        println!(
            "{}",
            crate::dialects::postgres_keywords::postgres_keywords()
                .iter()
                .map(|(k, v)| format!("{k} {v}"))
                .join("\n")
        );

        let kw_list_1 = &[("A", "not-keyword"), ("B", "non-reserved")];
        let kw_list_2 = &[("A", "reserved"), ("C", "non-reserved")];

        let result = priority_keyword_merge(&[kw_list_1, kw_list_2]);

        let expected_result = vec![("A", "reserved"), ("B", "non-reserved"), ("C", "non-reserved")];

        assert_eq!(sorted(result), sorted(expected_result));

        let result_2 = priority_keyword_merge(&[kw_list_2, kw_list_1]);

        let expected_result_2 =
            vec![("A", "not-keyword"), ("B", "non-reserved"), ("C", "non-reserved")];

        assert_eq!(sorted(result_2), sorted(expected_result_2));

        let kw_list_3 = &[("B", "reserved")];

        let result_3 = priority_keyword_merge(&[kw_list_2, kw_list_1, kw_list_3]);

        let expected_result_3 =
            vec![("A", "not-keyword"), ("B", "reserved"), ("C", "non-reserved")];

        assert_eq!(sorted(result_3), sorted(expected_result_3));

        let result_4 = priority_keyword_merge(&[kw_list_1]);

        let expected_result_4 = vec![("A", "not-keyword"), ("B", "non-reserved")];

        assert_eq!(sorted(result_4), sorted(expected_result_4));
    }

    #[test]
    fn test_get_keywords() {
        let kw_list = vec![
            ("A", "not-keyword"),
            ("B", "reserved"),
            ("C", "non-reserved"),
            ("D", "not-keyword"),
            ("E", "non-reserved-(cannot-be-function-or-type)"),
        ];

        let expected_result = vec!["A", "D"];
        assert_eq!(sorted(get_keywords(&kw_list, "not-keyword")), sorted(expected_result));

        let expected_result_2 = vec!["C", "E"];
        assert_eq!(sorted(get_keywords(&kw_list, "non-reserved")), sorted(expected_result_2));

        let expected_result_3 = vec!["B"];
        assert_eq!(sorted(get_keywords(&kw_list, "reserved")), sorted(expected_result_3));
    }
}