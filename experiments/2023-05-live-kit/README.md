# Live Kit experiment

**Questions**
Is "LiveKit" actually just another plugin for Here-Now? As in, can you equally just have a "Zoom API" plugin for managing the rooms?
Probably not, because we want to make the video offering available to other plugins as a first class primitive (e.g. for a built-in "Loom"-like app for recording questions and sharing those questions with others on the team).

**Goals**
 - [ ] Can we get some simple LiveKit server up and running and connected to with Rust?
 - [ ] Can we get autocompletions for LiveKit rust client etc.

**Anti-Goals**
 - Do not set up a fork of Live Kit
 - Do not make it a fully functional app
 - Do not support features outside of listing rooms, joining rooms, etc.

## Notes

LiveKit custom deployment is pretty much universally recommended to use `helm`, so in order to make progress with a custom local deployment of LiveKit, we'd need to use something like tilt.dev to orchestrate. I think we can table this and just use [LiveKit cloud](http://cloud.livekit.io/) for the moment.
