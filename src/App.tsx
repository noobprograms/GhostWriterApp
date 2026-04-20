import "./App.css";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Progress } from "@/components/ui/progress";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core"
export default function App() {
  const [file, setFile] = useState<File | null>(null);
  const [script, setScript] = useState("");
  const [style, setStyle] = useState("ghost");
  const [progress, setProgress] = useState(0);
  const [loading, setLoading] = useState(false);
  const [filePath, setFilePath] = useState<string | null>(null);

  const handlePickFile = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Media",
          extensions: ["mp4", "mov", "png", "jpg", "jpeg"],
        },
      ],
    });

    if (typeof selected === "string") {
      setFilePath(selected);
    }
  };
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files?.[0]) {
      setFile(e.target.files[0]);
    }
  };

  const handleGenerate = async () => {
  if (!filePath) {
    alert("Select a file first")
    return
  }

  setLoading(true)
  setProgress(10)

  try {
    const outputPath = filePath.replace(
      /\.(mp4|mov|png|jpg|jpeg)$/i,
      "_output.mp4"
    )
    console.log("Invoking video processing with:", {
      input: filePath,
      script,
      style,
      output: outputPath,
    })
    await invoke("process_video", {
      input: filePath,
      script,
      output: outputPath,
    })

    setProgress(100)
    alert("Done! Output saved at:\n" + outputPath)
  } catch (err) {
    console.error(err)
    alert("Something went wrong")
  } finally {
    setLoading(false)
  }
}

  return (
    <div className="min-h-screen bg-black text-white flex items-center justify-center p-6">
      <Card className="w-full max-w-2xl bg-neutral-900 border-neutral-800">
        <CardContent className="space-y-6 p-6">
          {/* Title */}
          <h1 className="text-2xl font-semibold tracking-tight">
            GhostText Studio
          </h1>

          {/* Upload */}
          <div className="space-y-2">
            <Button onClick={handlePickFile}>
              {filePath ? "Change File" : "Select File"}
            </Button>

            {filePath && (
              <p className="text-xs text-neutral-400 truncate">{filePath}</p>
            )}
          </div>

          {/* Script */}
          <div className="space-y-2">
            <label className="text-sm text-neutral-400">
              Script (with timing)
            </label>
            <Textarea
              placeholder={`[2.0] I shouldn't be here...\n[5.0] but I had to come.`}
              value={script}
              onChange={(e) => setScript(e.target.value)}
              className="bg-neutral-800 border-neutral-700 min-h-[120px]"
            />
          </div>

          {/* Style */}
          <div className="space-y-2">
            <label className="text-sm text-neutral-400">Style</label>
            <Select value={style} onValueChange={setStyle}>
              <SelectTrigger className="bg-neutral-800 border-neutral-700">
                <SelectValue placeholder="Select style" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ghost">Ghost</SelectItem>
                <SelectItem value="clean">Clean</SelectItem>
                <SelectItem value="horror">Horror</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* Progress */}
          {loading && (
            <Progress value={progress} className="h-2 bg-neutral-800" />
          )}

          {/* Button */}
          <Button
            onClick={handleGenerate}
            className="w-full bg-white text-black hover:bg-neutral-200"
            disabled={loading}
          >
            {loading ? "Processing..." : "Generate Video"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
