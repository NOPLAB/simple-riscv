"use client";

import { ChangeEvent } from "react";
import init, { run, WasmComputer, WasmComputerResult } from "../../pkg";

const Wasm = (input: ChangeEvent<HTMLInputElement>) => {
  let reader = new FileReader();
  reader.onload = () => {
    if (reader.result instanceof ArrayBuffer) {
      let program = new Uint8Array(reader.result);
      init().then(() => {
        let computer = new WasmComputer();
        computer.load(program);
        while (true) {
          let result = computer.increment();
          if (result == WasmComputerResult.ECALL) {
            return;
          }
        }
      });
    }
  };

  if (input.target.files?.[0] != null) {
    reader.readAsArrayBuffer(input.target.files?.[0]);
  }
};

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <input type="file" onChange={Wasm}></input>
    </main>
  );
}
