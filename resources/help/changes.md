# Changes

This file lists some of the things that are new between the versions of
Blightmud. It doesn't list all new features but will always list breaking
changes where you might need to take action.

---
# Changes in Blightmud v3.0
The `blight` module in lua has now been separated into multiple modules. You
will need to apply the following changes to your scripts.

## Blight module

- `blight:output(...)` => `blight.output(...)`
- `blight:terminal_dimension(...)` => `blight.terminal_dimension(...)`
- `blight:bind(...)` => `blight.bind(...)`
- `blight:unbind(...)` => `blight.unbind(...)`
- `blight:ui(...)` => `blight.ui(...)`
- `blight:status_height(...)` => `blight.status_height(...)`
- `blight:status_line(...)` => `blight.status_line(...)`
- `blight:version(...)` => `blight.version(...)`

## Triggers

- `blight:add_trigger(...)` => `trigger.add(...)`
- `blight:remove_trigger(id)` => `trigger.remove(id)`
- `blight:clear_triggers()` => `trigger.clear()`
- `blight:enable_trigger(id, bool)` => `trigger.get(id):set_enabled(bool)`
- `blight:gag` => `Line:gag(true)`

For more info about triggers and lines see `/help trigger` and `/help line`
There are now exists a group system for triggers for bulk trigger handling.

## Aliases

- `blight:add_alias(...)` => `alias.add(...)`
- `blight:remove_alias(id)` => `alias.remove(id)`
- `blight:clear_aliases()` => `alias.clear()`
- `blight:enable_alias(id, bool)` => `alias.get(id):set_enabled(bool)`

Just as with triggers aliases now have additional functionality through groups.
See `/help alias` for more information.

## Mud communication
- `blight:on_connect(...)` => `mud.on_connect(...)`
- `blight:on_disconnect(...)` => `mud.on_disconnect(...)`
- `blight:connect(...)` => `mud.connect(...)`
- `blight:send(...)` => `mud.send(...)`
- `blight:send_bytes(...)` => `mud.send_bytes(...)`
- `blight:user_input(...)` => `mud.input(...)`
- `blight:mud_output(...)` => `mud.output(...)`

## Logging
- `blight:start_log(...)` => `log.start(...)`
- `blight:stop_log(...)` => `log.stop(...)`

## Scripts
- `blight:load(...)` => `script.load(...)`
- `blight:reset()` => `script.reset()`

## Timers
- `blight:add_timer(...)` => `timer.add(...)`
- `blight:clear_timers()` => `timer.clear()`
- `blight:remove_timer(...)` => `timer.remove(...)`
- `blight:get_timer_ids()` => `timer.get_ids()`

## Storage
- `blight:store(...)` => `store.disk_write(...)`
- `blight:read(...)` => `store.disk_read(...)`
- `core:store(...)` => `store.session_write(...)`
- `core:read(...)` => `store.session_read(...)`

## Core
- `core:enable_protocol(...)` => `core.enable_protocol(...)`
- `core:disable_protocol(...)` => `core.disable_protocol(...)`
- `core:on_protocol_enabled(...)` => `core.on_protocol_enabled(...)`
- `core:subneg_recv(...)` => `core.subneg_recv(...)`
- `core:subneg_send(...)` => `core.subneg_send(...)`
- `core:exec(...)` => `core.exec(...)`

## TTS
- `tts:speak(...)` => `tts.speak(...)`
- `tts:speak_direct(...)` => `tts.speak_direct(...)`
- `tts:stop(...)` => `tts.stop(...)`
- `tts:enable(...)` => `tts.enable(...)`
- `tts:set_rate(...)` => `tts.set_rate(...)`
- `tts:change_rate(...)` => `tts.change_rate(...)`
- `tts:echo_keypresses(...)` => `tts.echo_keypresses(...)`
- `tts:step_back(...)` => `tts.step_back(...)`
- `tts:step_forward(...)` => `tts.step_forward(...)`
- `tts:scan_back(...)` => `tts.scan_back(...)`
- `tts:scan_forward(...)` => `tts.scan_forward(...)`
- `tts:scan_input_back(...)` => `tts.scan_input_back(...)`
- `tts:scan_input_forward(...)` => `tts.scan_input_forward(...)`
- `tts:step_begin(...)` => `tts.step_begin(...)`
- `tts:step_end(...)` => `tts.step_end(...)`

---
# Changes in Blightmud v2.0

## The GMCP module has been re-worked
All the gmcp related functionality now resides in the `gmcp` module.
It's already imported and ready to use. The following changes to your scripts will let you have a smooth transition.

`blight:on_gmcp_ready` is now referenced as `gmcp.on_ready`
`blight:register_gmcp` is now referenced as `gmcp.register`
`blight:add_gmcp_receiver` is now referenced as `gmcp.receive`
`blight:send_gmcp` is now referenced as `gmcp.send`
