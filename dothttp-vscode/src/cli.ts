import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";
import spawn from "cross-spawn";

export function resolveBinaryPath(): string | null {
  const config = vscode.workspace.getConfiguration("dothttp");
  const configured: string = config.get("binaryPath") ?? "";

  if (configured.trim() !== "") {
    return configured.trim();
  }

  // Fall back to PATH resolution
  const candidates = process.platform === "win32"
    ? ["dothttp.exe", "dothttp-cli.exe"]
    : ["dothttp", "dothttp-cli"];

  const pathDirs = (process.env.PATH ?? "").split(path.delimiter);
  for (const dir of pathDirs) {
    for (const name of candidates) {
      const full = path.join(dir, name);
      try {
        fs.accessSync(full, fs.constants.X_OK);
        return full;
      } catch {
        // not found here, keep looking
      }
    }
  }

  return null;
}

export interface CliResult {
  stdout: string;
  stderr: string;
}

export function runCliRequest(
  binaryPath: string,
  filePath: string,
  identifier: string
): Promise<CliResult> {
  return new Promise((resolve, reject) => {
    const config = vscode.workspace.getConfiguration("dothttp");
    const defaultEnv: string = config.get("defaultEnvironment") ?? "";

    const args = ["--file", filePath, "--request", identifier];
    if (defaultEnv.trim() !== "") {
      args.push("--env", defaultEnv.trim());
    }

    let stdout = "";
    let stderr = "";

    const child = spawn(binaryPath, args);

    child.stdout?.on("data", (chunk: Buffer) => {
      stdout += chunk.toString();
    });

    child.stderr?.on("data", (chunk: Buffer) => {
      stderr += chunk.toString();
    });

    child.on("close", (code: number | null) => {
      if (code === 0) {
        resolve({ stdout, stderr });
      } else {
        reject(new Error(stderr || `dothttp-cli exited with code ${code}`));
      }
    });

    child.on("error", (err: Error) => {
      reject(err);
    });
  });
}
