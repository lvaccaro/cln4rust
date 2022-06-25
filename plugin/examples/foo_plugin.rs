extern crate clightningrpc_plugin;

use clightningrpc_plugin::types::LogLevel;
use clightningrpc_plugin::{commands::RPCCommand, plugin::Plugin};
use serde_json::{json, Value};

#[derive(Clone)]
struct PluginState(());

/// HelloRPC is used to register the RPC method
#[derive(Clone)]
struct HelloRPC {}

/// Implementation of the RPC method
impl RPCCommand<PluginState> for HelloRPC {
    fn call<'c>(&self, plugin: &mut Plugin<PluginState>, _request: &'c Value) -> Value {
        plugin.log(LogLevel::Debug, "call the custom rpc method from rust");
        json!({
            "language": "Hello from rust"
        })
    }
}

#[derive(Clone)]
struct OnChannelOpened {}

impl RPCCommand<PluginState> for OnChannelOpened {
    fn call_void<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) {
        _plugin.log(LogLevel::Debug, "A new channel was opened!");
    }
}

fn main() {
    let mut plugin = Plugin::<PluginState>::new(PluginState(()), true)
        .add_rpc_method(
            "hello",
            "",
            "show how is possible add a method",
            HelloRPC {},
        )
        .add_opt(
            "foo",
            "flag",
            None,
            "An example of command line option",
            false,
        )
        .register_notification("channel_opened", OnChannelOpened {})
        .clone();
    plugin.start();
}
