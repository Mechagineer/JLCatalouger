import React from "react";

export default function App() {
  return (
    <div style={{ fontFamily: "system-ui, sans-serif", padding: 16 }}>
      <h1>JLCatalouger</h1>
      <p>Standalone, offline Reader/Compiler + Configurator.</p>
      <ol>
        <li>Import Catalog (PDF) → build Pack</li>
        <li>Open in Configurator → mutual gating + nearest-match</li>
      </ol>
      <p>See <code>research.md</code>, <code>plan.md</code>, and <code>techstack.md</code> for details.</p>
    </div>
  );
}
