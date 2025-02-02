use crate::dht::bbdht::dynamodb::api::item::read::get_item_by_address;
use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;

pub fn agent_exists(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    agent_id: &Address,
) -> BbDhtResult<bool> {
    tracer(&log_context, "agent_exists");

    // agent only exists in the space if the space exists
    Ok(if table_exists(log_context, client, table_name)? {
        get_item_by_address(log_context, client, table_name, agent_id)?.is_some()
    } else {
        false
    })
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::read::agent_exists;
    use crate::dht::bbdht::dynamodb::api::agent::write::touch_agent;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;

    #[test]
    fn agent_exists_test() {
        let log_context = "agent_exists";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let agent_id = agent_id_fresh();

        // agent not exists if space not exists
        match agent_exists(&log_context, &local_client, &table_name, &agent_id) {
            Ok(false) => {
                tracer(&log_context, "👌");
            }
            Ok(true) => {
                panic!("apparently agent exists before the space does");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // agent not exists if not join space
        match agent_exists(&log_context, &local_client, &table_name, &agent_id) {
            Ok(false) => {
                tracer(&log_context, "👌");
            }
            Ok(true) => {
                panic!("agent exists before join");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        // join
        assert!(touch_agent(&log_context, &local_client, &table_name, &agent_id).is_ok());

        // agent exists now
        match agent_exists(&log_context, &local_client, &table_name, &agent_id) {
            Ok(false) => {
                panic!("agent not exists after join");
            }
            Ok(true) => {
                tracer(&log_context, "👌");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

}
