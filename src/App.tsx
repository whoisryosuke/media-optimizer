import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet(fileName: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("opening file", fileName);
    if (name) await invoke("greet", { fileName });
  }

  const handleOpen = async () => {
    console.log("opening dialog...");
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg", "jpg"],
        },
      ],
    });

    console.log("found files maybe...", selected);

    if (selected === null) {
      // user cancelled the selection
    }

    if (Array.isArray(selected)) {
      // user selected multiple files
      setName(selected[0]);
      greet(selected[0]);
    }
  };

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <button onClick={handleOpen}>Compress file</button>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
