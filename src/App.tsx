import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2Icon } from "lucide-react";

import "./App.css";

function App() {
  const [loading, setLoading] = useState(false);
  return (
    <main className="justify-center mx-auto grid mt-32">
      <div className="w-full flex justify-center">
        <img width="100px" alt="qsv logo" src="/qsv-logo-icon.png" />
      </div>
      <h1 className="text-3xl my-4 font-bold">
        qsv Easy installer for Windows
      </h1>
      <p className="mb-4">
        Click the button to install the latest release of qsv and then qsv
        should be accessible from a new terminal.
      </p>
      <button
        disabled={loading}
        onClick={() => {
          setLoading(true);
          invoke("run_path_update")
            .then((message) => {
              setLoading(false);
              alert("Successfully installed qsv. Try opening a new terminal and run a qsv command! Version info: " + message);
            })
            .finally(() => {
              setLoading(false);
            });
        }}
        className="mx-auto w-full flex justify-center bg-blue-400"
      >
        {loading ? (
          <Loader2Icon className="animate-spin" />
        ) : (
          <p>Install qsv 🚀</p>
        )}
      </button>
      {loading && (
        <p className="mt-4 text-center">
          Downloading and installing qsv to PATH. This may take a few seconds...
        </p>
      )}
    </main>
  );
}

export default App;
