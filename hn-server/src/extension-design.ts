// @ts-nocheck

namespace app {
  announce(
    {
      IDENTIFY: {
        key: "here-now-app",
        protocol: "protocol//driver.v0"
      },
    },
    {
      CONFIG_SECTION_UI: {
        key: "public",
        order: 0,
        items: [
          {
            INPUT: {
              key: "base_url",
              label: "Where will people connect to your public server?",
              type: { TEXT: { default: "http://127.0.0.1:9000" } },
            },
          },
        ],
      },
    },
    {
      CONFIG_SECTION_UI: {
        key: "public_internal",
        order: 1,
        items: [
          {
            INPUT: {
              key: "bind_address",
              label: "The bind address for the public server",
              type: { TEXT: { default: "0.0.0.0:9000" } },
            },
          },
        ],
      },
    },
    {
      CONFIG_SECTION_UI: {
        key: "config",
        order: 2,
        items: [
          {
            INPUT: {
              key: "bind_address",
              label: "The bind address for the config server",
              type: { TEXT: { default: "0.0.0.0:3000" } },
            },
            INPUT: {
              key: "base_url",
              label: "The base url to use for the config server",
              type: { TEXT: { default: "http://127.0.0.1:3000" } },
            },
            INPUT: {
              key: "jaeger_trace_collector_endpoint",
              label: "Where to send trace data when using Jaeger",
              type: { TEXT: { default: "http://localhost:14268/api/traces" } },
            },
          },
        ],
      },
    },
    {
      // resource:public_url
      IDENTIFY_RESOURCE: {
        key: "public_url",
        type_id: "protocol//http-server",
      },
    },
    {
      RESOURCE_UI: {
        key: "public_url",
      },
    },
  );

  announce(
    {
      IDENTIFY: {
        key: "discord-bot",
        links: [
          {
            label: "Here Now Server",
            driver_id: "here-now-app",
            shared: [{ item_id: "section/input//public/base_url", optional: false }],
          },
        ],
      },
    },
    {
      CONFIG_SECTION_UI: {
        key: "discord",
        items: [
          { INPUT: { key: "client_id", type: { TEXT: { placeholder: "" } }, label: "Hello", help: "Help text" } },
          { INPUT: { key: "client_secret", label: "Secret from ..." } },
          { INPUT: { key: "bot_token", label: "Looks kinda like..." } },
        ],
      },
    },
  );

  operator({});

  announce({
    RAISE: {
      // multiple keys so external applications can resolve it?
      // keys: ["error-102738902903878389"],
      key: "",
      resolved_by: [
        {
          label: "Need to specify a public url in order to give other applications a public URL.",
          type: { SECTION_INPUTS: ["section/input:public/"] },
        },
      ],
    },
  });
}

function describe(...args: any[]) {}
function operator(...args: any[]) {}
function inp(...args: any[]) {}
/**
 * should it be implied that every item is part of some "active conversation" id?
 * So each item can describe the context it refers to / is responding to, rather than
 * expecting to always speak from a global stand point?
 *
 * Similarly, these conversation contexts could be traced / spanned, right?
 */
function announce(...args: any[]) {}
