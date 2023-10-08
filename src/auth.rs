use crate::infrastructure::MySqlGateway;

struct Auth<G> {
    credentials_gateway: G,
    credentials_table_name: String,
    session_gateway: Option<G>,
    session_table_name: String,
    use_jwt_toke: bool,
}
