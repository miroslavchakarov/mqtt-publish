use futures::executor::block_on;
use paho_mqtt as mqtt;
use std::{env};

/////////////////////////////////////////////////////////////////////////////
//fn main() -> mqtt::Result<()> {
fn main()  {
    // Initialize the logger from the environment
    env_logger::init();

    // Let the user override the host, but note the "ssl://" protocol.
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "ssl://dev.mqtt.averato.com:8883".to_string());

    println!("Connecting to host: '{}'", host);

    // Run the client in an async block

    if let Err(err) = block_on(async {
        // Create a client & define connect options
        let cli = mqtt::CreateOptionsBuilder::new()
            .server_uri(&host)
            .client_id("raspberry")
            .max_buffered_messages(100)
            .create_client()?;

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .ssl_options(mqtt::SslOptions::new())
            .user_name("us")
            .password("...")
            .finalize();

        cli.connect(conn_opts).await?;

        let msg = mqtt::MessageBuilder::new()
            .topic("blackseachain-demo-vnd/rpi/vpos-client/msg")
            .payload("0.49&BGN")
            .qos(1)
            .finalize();

        cli.publish(msg).await?;
        cli.disconnect(None).await?;

        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
   // Ok(())
}