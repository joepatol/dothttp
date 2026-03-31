import * as vscode from "vscode";
import { resolveBinaryPath, runCliRequest } from "./cli";
import { ResponsePanel } from "./responsePanel";
import { DothttpCodeLensProvider } from "./codeLensProvider";

export function activate(context: vscode.ExtensionContext): void {
  // Check binary on activation
  const binary = resolveBinaryPath();
  if (binary === null) {
    vscode.window
      .showWarningMessage(
        "dothttp: CLI binary not found. Configure the path in settings.",
        "Open Settings"
      )
      .then((choice) => {
        if (choice === "Open Settings") {
          vscode.commands.executeCommand(
            "workbench.action.openSettings",
            "dothttp.binaryPath"
          );
        }
      });
  }

  // Register CodeLens provider
  const codeLensProvider = new DothttpCodeLensProvider(context);
  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider({ language: "dothttp" }, codeLensProvider)
  );

  // Register runRequest command
  const runRequestCmd = vscode.commands.registerCommand(
    "dothttp.runRequest",
    async (filePath: string | undefined, identifier: string | undefined) => {
      if (!filePath || !identifier) {
        vscode.window.showErrorMessage(
          "dothttp.runRequest requires a file path and request identifier."
        );
        return;
      }

      const bin = resolveBinaryPath();
      if (bin === null) {
        vscode.window
          .showErrorMessage(
            "dothttp: CLI binary not found. Configure the path in settings.",
            "Open Settings"
          )
          .then((choice) => {
            if (choice === "Open Settings") {
              vscode.commands.executeCommand(
                "workbench.action.openSettings",
                "dothttp.binaryPath"
              );
            }
          });
        return;
      }

      try {
        const { stdout } = await runCliRequest(bin, filePath, identifier);
        ResponsePanel.show(stdout);
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        vscode.window.showErrorMessage(`dothttp error: ${message}`);
      }
    }
  );

  context.subscriptions.push(runRequestCmd);
}

export function deactivate(): void {
  // nothing to clean up beyond subscriptions
}
