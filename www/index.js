import * as lotsaLib from "lotsa-wasm";

const client = new lotsaLib.LotsaClient();
setTimeout(() => client.send_message("WAT"), 1000);
