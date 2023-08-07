// prettier no-semi
// deno-lint-ignore-file
// @ ts-nocheck experimental

import * as iam from "./protocol/is/iam.v0.gen.ts"
import * as driver from "./protocol/is/driver.v0.gen.js"
// import * as service from "./protocol/is/service.v0.gen.ts"

async function test() {
  await ch0.expect(({ iam, driver }) => {
    // this is essentially a long-lived process
    iam.IDENTIFY({
      title: "Here Now App",
      // protocols: ["protocol//iam.v0", "protocol//driver.v0"],
    })
    iam.UI({
      key: "dev",
      order: 1,
      title: "Developer Settings",
      items: [
        {
          INPUT: {
            key: "dev_mode",
            label: "Enable Dev Mode",
            type: { TEXT: {} },
          },
        },
      ],
    })
    driver.DECLARE_SERVICE({
      key: "public-http",
      title: "",
      // protocols: [
      //   "protocol//configuration",
      //   "protocol//http-server",
      //   "protocol//driver",
      // ],
    })
  })

  ch0.in.driver.CREATE_SERVICE({
    service_key: "public-http",
    channel: "1",
  })

  const ch1 = {
    in: {
      service: service.In.factory(
        console.log.bind("[1] %c-> Service", "color: blue"),
      ),
    },
    expect: {
      service: service.Out.factory(async (value) =>
        console.log("[1] %cService ->", "color: yellow", value),
      ),
    },
  }

  ch1.expect.iam.IDENTIFY({
    title: "",
  })

  ch1.expect.IDENTIFY({})

  announce(
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
  )

  announce(
    {
      IDENTIFY: {
        key: "discord-bot",
        links: [
          {
            label: "Here Now Server",
            driver_id: "here-now-app",
            shared: [
              { item_id: "section/input//public/base_url", optional: false },
            ],
          },
        ],
      },
    },
    {
      CONFIG_SECTION_UI: {
        key: "discord",
        items: [
          {
            INPUT: {
              key: "client_id",
              type: { TEXT: { placeholder: "" } },
              label: "Hello",
              help: "Help text",
            },
          },
          { INPUT: { key: "client_secret", label: "Secret from ..." } },
          { INPUT: { key: "bot_token", label: "Looks kinda like..." } },
        ],
      },
    },
  )

  operator({})

  announce({
    RAISE: {
      // multiple keys so external applications can resolve it?
      // keys: ["error-102738902903878389"],
      key: "",
      resolved_by: [
        {
          label:
            "Need to specify a public url in order to give other applications a public URL.",
          type: { SECTION_INPUTS: ["section/input:public/"] },
        },
      ],
    },
  })
}

const ch0 = {
  in: {
    /** driver.v0 */
    driver: driver.In.factory(async (value) =>
      console.log("[0] %c-> [driver]", "color: blue", value),
    ),
    /** iam */
    iam: iam.In.factory(async (value) =>
      console.log("[0] %c-> [iam]", "color: blue", value),
    ),
  },
  expect(
    fn: (protos: {
      driver: driver.Out.ApplyFns
      iam: iam.Out.ApplyFns
    }) => void,
  ) {
    fn({
      iam: iam.Out.factory((value) =>
        console.log("[0] %c[iam] ->", "color: yellow", value),
      ),
      driver: driver.Out.factory((value) =>
        console.log("[0] %c[driver] ->", "color: yellow", value),
      ),
    })
  },
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
