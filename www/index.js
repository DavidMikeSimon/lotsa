import * as lotsaLib from "lotsa-wasm";

const client = new lotsaLib.LotsaClient();
setTimeout(() => client.handle_message("WAT"), 1000);
