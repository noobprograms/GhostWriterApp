import "./App.css";
import { useState, useEffect } from "react";
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
import { Switch } from "./components/ui/switch";
import { Label } from "./components/ui/label";
export default function App() {
  const [script, setScript] = useState("");
  const [style, setStyle] = useState("ghost");
  const [progress, setProgress] = useState(0);
  const [loading, setLoading] = useState(false);
  const [filePath, setFilePath] = useState<string | null>(null);
  const [shouldDrawGlow, setShouldDrawGlow] = useState(false);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
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

  useEffect(() => {
    return () => {
      if (previewUrl) URL.revokeObjectURL(previewUrl)
    }
  }, [previewUrl])

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
    // invoke backend which now returns base64-encoded video bytes
    const base64: string = await invoke("process_video", {
      input: filePath,
      script,
      output: outputPath,
      shouldDrawGlow: shouldDrawGlow,
    }) as string

    try {
      const binaryString = atob(base64)
      const len = binaryString.length
      const bytes = new Uint8Array(len)
      for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i)
      }
      const blob = new Blob([bytes], { type: "video/mp4" })
      const url = URL.createObjectURL(blob)
      if (previewUrl) URL.revokeObjectURL(previewUrl)
      setPreviewUrl(url)
      setProgress(100)
    } catch (decodeErr) {
      console.error("Failed to decode returned video:", decodeErr)
      alert("Done! (but preview failed)")
    }
  } catch (err) {
    console.error(err)
    alert("Something went wrong")
  } finally {
    setLoading(false)
  }
}

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-white flex items-center justify-center p-6">
      <Card className="w-full max-w-3xl bg-transparent border-0 shadow-xl">
        <CardContent className="space-y-6 p-8 bg-neutral-900/60 backdrop-blur rounded-xl border border-neutral-800">
          {/* Title */}
          <h1 className="text-3xl font-bold text-white tracking-tight">
            GhostText Studio
          </h1>

          {/* Upload */}
          <div className="flex flex-col sm:flex-row sm:items-center gap-4">
            <Button onClick={handlePickFile} className="flex-shrink-0">
              {filePath ? "Change File" : "Select File"}
            </Button>
            <div className="flex-1 min-w-0">
              {filePath ? (
                <p className="text-sm text-neutral-300 truncate">{filePath}</p>
              ) : (
                <p className="text-sm text-neutral-500">No file selected</p>
              )}
            </div>
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
              className="bg-neutral-800 border-neutral-700 min-h-[120px] text-white"
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
          {/* Glow Effect */}
          <div className="space-y-2">
            <Label htmlFor="glow-switch" className="text-white">Toggle Glow Effect</Label>
            <Switch
              id="glow-switch"
              checked={shouldDrawGlow}
              onCheckedChange={setShouldDrawGlow}
            />
          </div>

          {/* Progress */}
          {loading && (
            <Progress value={progress} className="h-2 bg-emerald-500/80" />
          )}

          {/* Button */}
          <Button
            onClick={handleGenerate}
            className="w-full bg-emerald-500 text-black hover:bg-emerald-400"
            disabled={loading}
          >
            {loading ? "Processing..." : "Generate Video"}
          </Button>

          {/* Preview */}
          {previewUrl && (
            <div className="mt-4">
              <h2 className="text-sm text-neutral-300 mb-2">Preview</h2>
              <div className="w-full bg-black rounded-md overflow-hidden">
                <video src={previewUrl} controls className="w-full h-auto" />
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
