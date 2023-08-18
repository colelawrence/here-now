// prettier no-semi
// deno-lint-ignore-file
// @ts-nocheck experimental

import * as iam from "./protocol/is/iam.v0.gen.ts"
import * as driver from "./protocol/is/driver.v0.gen.ts"
import { ch } from "./ch.ts"
// import * as service from "./protocol/is/service.v0.gen.ts"

async function test() {
  const ch0 = ch({ iam })
  const ch2 = ch({ iam })

  ch0.expect(({ iam: setup }) => {
    setup.IDENTIFY({
      title: "Discord Setup",
    })

    setup.OFFER({
      key: "setup-discord",
      title: "Setup Discord",
      known_params: [
        iam.UIItem.INPUT({
          key: "discord-client-id",
          label: "Application ID",
          type: iam.UIInputType.TEXT({
            examples: ["1132773161985908787", "1024467966416388126"],
            format_description: `Discord's "Application ID" or also called "Client ID".`,
          }),
        }),
      ],
    })
  })

  ch0.in.iam.ASK({
    offer_key: "setup-discord",
    channel: "2",
    given_params: [],
  })

  ch2.expect(({ iam: setup }) => {
    setup.IDENTIFY({
      title: "Discord Setup (in progress)",
    })

    setup.UI({
      key: "setup",
      items: [
        "Header",
        {
          INPUT: {
            key: "akwjklawdj",
            label: "client-id"
          }
        }
      ]
    })

    setup.RAISE({
      key: "need-client-id",
      related_input_keys: ["akwjklawdj"],
      ui_key: "setup"
      
    })

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
