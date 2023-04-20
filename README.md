# Here Now

Tentative plans for "Here Now".

I started working on my own light-weight version of a virtual office space like Tandem.chat, Remotion, or Tulip which is built on slint-ui with Rust and only implements the basic waving features.
I'm working on this for fun, and because I genuinely think I'd have more conversations with friends and collaborators if it was easier to connect, see who's online, and wave for help or to start a conversation.
With Tandem.chat (the platform I have the most experience with), I think it was a challenge to convince people to keep yet another Electron app open on their computer, so I'm going with technology that uses the minimum amount of resources possible while being cross-platform.

## Technology

**Front-end:**
 * Slint-UI: Cross-platform desktop front-end with an innovative "Slint markup language"
    * As they are constantly improving slint, we'll probably talk with them some and contribute features like https://github.com/slint-ui/slint/pull/2557
 * Rust for desktop app state management and binding to Slint-UI
 * (Considering) GStreamer for
    * Rendering incoming streamed videos and audio
    * Streaming desktop application windows for screen share

**Back-end**
 * Axum: Rust web server framework for general web serving and websockets
 * Considering: [LiveKit](https://github.com/livekit/livekit) for live-streaming infra (leveraging their Rust SDK)

**Tooling**
 * TypeScript/Deno/Bun for code generation and general purpose repo maintenance
 * Git for version control
 * Github for issue management

Anti-tech:
 * No JS/HTML/CSS for front-end.
   * For now, I just don't want to add complexity of three additional languages for the front-end as I think it discourages additional contributors
   * I believe framing content in web-views will add complexity to distribution and add to the weight of the running application (where I want it to be super low-resources even if you're in a shared room).

## Who can help the most

 * GStreamer knowledgeable advisors / enthusiasts
 * WebRTC video implementors for advice
 * Anyone interested in designing and building UI interfaces in Figma & slint-ui (I can help teach you slint markup language, it's awesome!)
 
Are you interested in playing with slint-ui or very basic webservices for direct notifications?
Please open an issue introducing yourself or email me at cole@colelawrence.com

## Why open source?

I think people who have experienced Tandem.chat compared to Slack/Discord have a clearer understanding of the importance of this kind of tool for engineering teams.
But, I think the market is very difficult to penetrate since this kind of application is very difficult to explain the nuances of. So, let's just make it in public, share ideas, and make this kind of application for ourselves.

