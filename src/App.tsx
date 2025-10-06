import React, { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

export default function App() {
  const [lastOut, setLastOut] = useState<string>("");

  const onBuild = async () => {
    try {
      const sel = await open({
        multiple: false,
        filters: [{ name: "PDF", extensions: ["pdf"] }]
      });
      if (!sel || Array.isArray(sel)) return;
      // Optional: customize packs root; default is ./packs
      const out = await invoke<string>("build_smoke_pack", {
        inputPdf: sel,
        packsRoot: null
      });
      setLastOut(out);
      alert(`Pack created:\n${out}`);
    } catch (e:any) {
      alert(`Failed: ${e.toString()}`);
    }
  };

  return (
    <div className="wrap">
      <h1>JLCatalouger</h1>
      <p>Smoke test: build a minimal Pack from a selected PDF (no parsing yet).</p>
      <button onClick={onBuild}>Build Smoke Pack</button>
      {lastOut && (
        <p style={{marginTop: 12}}>
          Last output: <code>{lastOut}</code>
        </p>
      )}
    </div>
  );
}
