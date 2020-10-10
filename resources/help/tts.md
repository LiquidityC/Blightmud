# Text to Speech

Blightmud has built in support for Text-to-speech (TTS).

You can enable TTS using `/tts on|off` or by providing the flag `--tts` on the commandline.
This will make everything printed spoken through TTS.

Make sure you disable your screen reader before you do this as blightmud and your screen
reader software sharing the same speech dispatcher isn't always a match made
in heaven.

If you want to use TTS just for notifications and other special information
you can interact with it through lua.

## Macros

`/tts on|off`          Enable or disable TTS
`/tts_rate <rate>`     Set the TTS rate

## Functions

***tts:speak(msg, interupt)***
Will speak the provided. If interupt is true, this message will interupt
possible messages that are waiting to be spoken.

##

***tts:enable(on)***
Toggle general TTS on or off. Where `on` is wither true or false.

##

***tts:enabled() -> enabled***
Returns if tts is enabled or not

##

***tts:set_rate(rate)***
Set the speech rate. Default is usually 0, max is 100 and min is -100. This can
vary on different operating systems.

##

***tts:change_rate(change)***
Increase or decrease the rate of speech

##

***tts:gag()***
Used from within a triggers callback function this will prevent the matched
line from being spoken through TTS.

See `/help triggers` for details about triggers.

##

***tts:stop()***
Stop all speach

## Bindings

By default `ctrl-s` is bound to stop current TTS and clear the queue.
You can rebind this as you please. See `/help bindings`