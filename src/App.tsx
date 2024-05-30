import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Button, Slider, Text, Title } from "@mantine/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [quality, setQuality] = useState(50);

  async function greet(fileName: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("opening file", fileName);
    if (name) await invoke("greet", { fileName, quality });
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

  const handleQuality = (value: number) => {
    setQuality(value);
  };

  return (
    <div className="container">
      <Title>Media Optimizer</Title>
      <Text>Select a file to compress it</Text>

      <Slider
        value={quality}
        onChange={handleQuality}
        marks={[
          { value: 20, label: "20%" },
          { value: 50, label: "50%" },
          { value: 80, label: "80%" },
        ]}
      />

      <Button onClick={handleOpen}>Compress file</Button>

      <Text>{greetMsg}</Text>
    </div>
  );
}

export default App;
