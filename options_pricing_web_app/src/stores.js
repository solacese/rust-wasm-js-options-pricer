import { writable } from "svelte/store";
import { createMachine } from "@xstate/fsm";
import { useMachine } from "./useMachine";

// UI state machines
export const connectMachine = writable(
  useMachine(
    createMachine({
      id: "connectMachine",
      initial: "disconnected",
      states: {
        disconnected: {
          on: {
            CONNECT_REQUEST: "connecting",
          },
        },
        connecting: {
          on: {
            UP_NOTICE: "connected",
            ERROR: "error",
          },
        },
        connected: {
          on: {
            DISCONNECTED: "disconnected",
          },
        },
        error: {
          on: {
            CONNECT_REQUEST: "connecting",
          },
        },
      },
    }),
  ),
);
