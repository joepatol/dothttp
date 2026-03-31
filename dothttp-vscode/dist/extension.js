"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// node_modules/isexe/windows.js
var require_windows = __commonJS({
  "node_modules/isexe/windows.js"(exports2, module2) {
    module2.exports = isexe;
    isexe.sync = sync;
    var fs2 = require("fs");
    function checkPathExt(path2, options) {
      var pathext = options.pathExt !== void 0 ? options.pathExt : process.env.PATHEXT;
      if (!pathext) {
        return true;
      }
      pathext = pathext.split(";");
      if (pathext.indexOf("") !== -1) {
        return true;
      }
      for (var i = 0; i < pathext.length; i++) {
        var p = pathext[i].toLowerCase();
        if (p && path2.substr(-p.length).toLowerCase() === p) {
          return true;
        }
      }
      return false;
    }
    function checkStat(stat, path2, options) {
      if (!stat.isSymbolicLink() && !stat.isFile()) {
        return false;
      }
      return checkPathExt(path2, options);
    }
    function isexe(path2, options, cb) {
      fs2.stat(path2, function(er, stat) {
        cb(er, er ? false : checkStat(stat, path2, options));
      });
    }
    function sync(path2, options) {
      return checkStat(fs2.statSync(path2), path2, options);
    }
  }
});

// node_modules/isexe/mode.js
var require_mode = __commonJS({
  "node_modules/isexe/mode.js"(exports2, module2) {
    module2.exports = isexe;
    isexe.sync = sync;
    var fs2 = require("fs");
    function isexe(path2, options, cb) {
      fs2.stat(path2, function(er, stat) {
        cb(er, er ? false : checkStat(stat, options));
      });
    }
    function sync(path2, options) {
      return checkStat(fs2.statSync(path2), options);
    }
    function checkStat(stat, options) {
      return stat.isFile() && checkMode(stat, options);
    }
    function checkMode(stat, options) {
      var mod = stat.mode;
      var uid = stat.uid;
      var gid = stat.gid;
      var myUid = options.uid !== void 0 ? options.uid : process.getuid && process.getuid();
      var myGid = options.gid !== void 0 ? options.gid : process.getgid && process.getgid();
      var u = parseInt("100", 8);
      var g = parseInt("010", 8);
      var o = parseInt("001", 8);
      var ug = u | g;
      var ret = mod & o || mod & g && gid === myGid || mod & u && uid === myUid || mod & ug && myUid === 0;
      return ret;
    }
  }
});

// node_modules/isexe/index.js
var require_isexe = __commonJS({
  "node_modules/isexe/index.js"(exports2, module2) {
    var fs2 = require("fs");
    var core;
    if (process.platform === "win32" || global.TESTING_WINDOWS) {
      core = require_windows();
    } else {
      core = require_mode();
    }
    module2.exports = isexe;
    isexe.sync = sync;
    function isexe(path2, options, cb) {
      if (typeof options === "function") {
        cb = options;
        options = {};
      }
      if (!cb) {
        if (typeof Promise !== "function") {
          throw new TypeError("callback not provided");
        }
        return new Promise(function(resolve, reject) {
          isexe(path2, options || {}, function(er, is) {
            if (er) {
              reject(er);
            } else {
              resolve(is);
            }
          });
        });
      }
      core(path2, options || {}, function(er, is) {
        if (er) {
          if (er.code === "EACCES" || options && options.ignoreErrors) {
            er = null;
            is = false;
          }
        }
        cb(er, is);
      });
    }
    function sync(path2, options) {
      try {
        return core.sync(path2, options || {});
      } catch (er) {
        if (options && options.ignoreErrors || er.code === "EACCES") {
          return false;
        } else {
          throw er;
        }
      }
    }
  }
});

// node_modules/which/which.js
var require_which = __commonJS({
  "node_modules/which/which.js"(exports2, module2) {
    var isWindows = process.platform === "win32" || process.env.OSTYPE === "cygwin" || process.env.OSTYPE === "msys";
    var path2 = require("path");
    var COLON = isWindows ? ";" : ":";
    var isexe = require_isexe();
    var getNotFoundError = (cmd) => Object.assign(new Error(`not found: ${cmd}`), { code: "ENOENT" });
    var getPathInfo = (cmd, opt) => {
      const colon = opt.colon || COLON;
      const pathEnv = cmd.match(/\//) || isWindows && cmd.match(/\\/) ? [""] : [
        // windows always checks the cwd first
        ...isWindows ? [process.cwd()] : [],
        ...(opt.path || process.env.PATH || /* istanbul ignore next: very unusual */
        "").split(colon)
      ];
      const pathExtExe = isWindows ? opt.pathExt || process.env.PATHEXT || ".EXE;.CMD;.BAT;.COM" : "";
      const pathExt = isWindows ? pathExtExe.split(colon) : [""];
      if (isWindows) {
        if (cmd.indexOf(".") !== -1 && pathExt[0] !== "")
          pathExt.unshift("");
      }
      return {
        pathEnv,
        pathExt,
        pathExtExe
      };
    };
    var which = (cmd, opt, cb) => {
      if (typeof opt === "function") {
        cb = opt;
        opt = {};
      }
      if (!opt)
        opt = {};
      const { pathEnv, pathExt, pathExtExe } = getPathInfo(cmd, opt);
      const found = [];
      const step = (i) => new Promise((resolve, reject) => {
        if (i === pathEnv.length)
          return opt.all && found.length ? resolve(found) : reject(getNotFoundError(cmd));
        const ppRaw = pathEnv[i];
        const pathPart = /^".*"$/.test(ppRaw) ? ppRaw.slice(1, -1) : ppRaw;
        const pCmd = path2.join(pathPart, cmd);
        const p = !pathPart && /^\.[\\\/]/.test(cmd) ? cmd.slice(0, 2) + pCmd : pCmd;
        resolve(subStep(p, i, 0));
      });
      const subStep = (p, i, ii) => new Promise((resolve, reject) => {
        if (ii === pathExt.length)
          return resolve(step(i + 1));
        const ext = pathExt[ii];
        isexe(p + ext, { pathExt: pathExtExe }, (er, is) => {
          if (!er && is) {
            if (opt.all)
              found.push(p + ext);
            else
              return resolve(p + ext);
          }
          return resolve(subStep(p, i, ii + 1));
        });
      });
      return cb ? step(0).then((res) => cb(null, res), cb) : step(0);
    };
    var whichSync = (cmd, opt) => {
      opt = opt || {};
      const { pathEnv, pathExt, pathExtExe } = getPathInfo(cmd, opt);
      const found = [];
      for (let i = 0; i < pathEnv.length; i++) {
        const ppRaw = pathEnv[i];
        const pathPart = /^".*"$/.test(ppRaw) ? ppRaw.slice(1, -1) : ppRaw;
        const pCmd = path2.join(pathPart, cmd);
        const p = !pathPart && /^\.[\\\/]/.test(cmd) ? cmd.slice(0, 2) + pCmd : pCmd;
        for (let j = 0; j < pathExt.length; j++) {
          const cur = p + pathExt[j];
          try {
            const is = isexe.sync(cur, { pathExt: pathExtExe });
            if (is) {
              if (opt.all)
                found.push(cur);
              else
                return cur;
            }
          } catch (ex) {
          }
        }
      }
      if (opt.all && found.length)
        return found;
      if (opt.nothrow)
        return null;
      throw getNotFoundError(cmd);
    };
    module2.exports = which;
    which.sync = whichSync;
  }
});

// node_modules/path-key/index.js
var require_path_key = __commonJS({
  "node_modules/path-key/index.js"(exports2, module2) {
    "use strict";
    var pathKey = (options = {}) => {
      const environment = options.env || process.env;
      const platform = options.platform || process.platform;
      if (platform !== "win32") {
        return "PATH";
      }
      return Object.keys(environment).reverse().find((key) => key.toUpperCase() === "PATH") || "Path";
    };
    module2.exports = pathKey;
    module2.exports.default = pathKey;
  }
});

// node_modules/cross-spawn/lib/util/resolveCommand.js
var require_resolveCommand = __commonJS({
  "node_modules/cross-spawn/lib/util/resolveCommand.js"(exports2, module2) {
    "use strict";
    var path2 = require("path");
    var which = require_which();
    var getPathKey = require_path_key();
    function resolveCommandAttempt(parsed, withoutPathExt) {
      const env = parsed.options.env || process.env;
      const cwd = process.cwd();
      const hasCustomCwd = parsed.options.cwd != null;
      const shouldSwitchCwd = hasCustomCwd && process.chdir !== void 0 && !process.chdir.disabled;
      if (shouldSwitchCwd) {
        try {
          process.chdir(parsed.options.cwd);
        } catch (err) {
        }
      }
      let resolved;
      try {
        resolved = which.sync(parsed.command, {
          path: env[getPathKey({ env })],
          pathExt: withoutPathExt ? path2.delimiter : void 0
        });
      } catch (e) {
      } finally {
        if (shouldSwitchCwd) {
          process.chdir(cwd);
        }
      }
      if (resolved) {
        resolved = path2.resolve(hasCustomCwd ? parsed.options.cwd : "", resolved);
      }
      return resolved;
    }
    function resolveCommand(parsed) {
      return resolveCommandAttempt(parsed) || resolveCommandAttempt(parsed, true);
    }
    module2.exports = resolveCommand;
  }
});

// node_modules/cross-spawn/lib/util/escape.js
var require_escape = __commonJS({
  "node_modules/cross-spawn/lib/util/escape.js"(exports2, module2) {
    "use strict";
    var metaCharsRegExp = /([()\][%!^"`<>&|;, *?])/g;
    function escapeCommand(arg) {
      arg = arg.replace(metaCharsRegExp, "^$1");
      return arg;
    }
    function escapeArgument(arg, doubleEscapeMetaChars) {
      arg = `${arg}`;
      arg = arg.replace(/(?=(\\+?)?)\1"/g, '$1$1\\"');
      arg = arg.replace(/(?=(\\+?)?)\1$/, "$1$1");
      arg = `"${arg}"`;
      arg = arg.replace(metaCharsRegExp, "^$1");
      if (doubleEscapeMetaChars) {
        arg = arg.replace(metaCharsRegExp, "^$1");
      }
      return arg;
    }
    module2.exports.command = escapeCommand;
    module2.exports.argument = escapeArgument;
  }
});

// node_modules/shebang-regex/index.js
var require_shebang_regex = __commonJS({
  "node_modules/shebang-regex/index.js"(exports2, module2) {
    "use strict";
    module2.exports = /^#!(.*)/;
  }
});

// node_modules/shebang-command/index.js
var require_shebang_command = __commonJS({
  "node_modules/shebang-command/index.js"(exports2, module2) {
    "use strict";
    var shebangRegex = require_shebang_regex();
    module2.exports = (string = "") => {
      const match = string.match(shebangRegex);
      if (!match) {
        return null;
      }
      const [path2, argument] = match[0].replace(/#! ?/, "").split(" ");
      const binary = path2.split("/").pop();
      if (binary === "env") {
        return argument;
      }
      return argument ? `${binary} ${argument}` : binary;
    };
  }
});

// node_modules/cross-spawn/lib/util/readShebang.js
var require_readShebang = __commonJS({
  "node_modules/cross-spawn/lib/util/readShebang.js"(exports2, module2) {
    "use strict";
    var fs2 = require("fs");
    var shebangCommand = require_shebang_command();
    function readShebang(command) {
      const size = 150;
      const buffer = Buffer.alloc(size);
      let fd;
      try {
        fd = fs2.openSync(command, "r");
        fs2.readSync(fd, buffer, 0, size, 0);
        fs2.closeSync(fd);
      } catch (e) {
      }
      return shebangCommand(buffer.toString());
    }
    module2.exports = readShebang;
  }
});

// node_modules/cross-spawn/lib/parse.js
var require_parse = __commonJS({
  "node_modules/cross-spawn/lib/parse.js"(exports2, module2) {
    "use strict";
    var path2 = require("path");
    var resolveCommand = require_resolveCommand();
    var escape = require_escape();
    var readShebang = require_readShebang();
    var isWin = process.platform === "win32";
    var isExecutableRegExp = /\.(?:com|exe)$/i;
    var isCmdShimRegExp = /node_modules[\\/].bin[\\/][^\\/]+\.cmd$/i;
    function detectShebang(parsed) {
      parsed.file = resolveCommand(parsed);
      const shebang = parsed.file && readShebang(parsed.file);
      if (shebang) {
        parsed.args.unshift(parsed.file);
        parsed.command = shebang;
        return resolveCommand(parsed);
      }
      return parsed.file;
    }
    function parseNonShell(parsed) {
      if (!isWin) {
        return parsed;
      }
      const commandFile = detectShebang(parsed);
      const needsShell = !isExecutableRegExp.test(commandFile);
      if (parsed.options.forceShell || needsShell) {
        const needsDoubleEscapeMetaChars = isCmdShimRegExp.test(commandFile);
        parsed.command = path2.normalize(parsed.command);
        parsed.command = escape.command(parsed.command);
        parsed.args = parsed.args.map((arg) => escape.argument(arg, needsDoubleEscapeMetaChars));
        const shellCommand = [parsed.command].concat(parsed.args).join(" ");
        parsed.args = ["/d", "/s", "/c", `"${shellCommand}"`];
        parsed.command = process.env.comspec || "cmd.exe";
        parsed.options.windowsVerbatimArguments = true;
      }
      return parsed;
    }
    function parse(command, args, options) {
      if (args && !Array.isArray(args)) {
        options = args;
        args = null;
      }
      args = args ? args.slice(0) : [];
      options = Object.assign({}, options);
      const parsed = {
        command,
        args,
        options,
        file: void 0,
        original: {
          command,
          args
        }
      };
      return options.shell ? parsed : parseNonShell(parsed);
    }
    module2.exports = parse;
  }
});

// node_modules/cross-spawn/lib/enoent.js
var require_enoent = __commonJS({
  "node_modules/cross-spawn/lib/enoent.js"(exports2, module2) {
    "use strict";
    var isWin = process.platform === "win32";
    function notFoundError(original, syscall) {
      return Object.assign(new Error(`${syscall} ${original.command} ENOENT`), {
        code: "ENOENT",
        errno: "ENOENT",
        syscall: `${syscall} ${original.command}`,
        path: original.command,
        spawnargs: original.args
      });
    }
    function hookChildProcess(cp, parsed) {
      if (!isWin) {
        return;
      }
      const originalEmit = cp.emit;
      cp.emit = function(name, arg1) {
        if (name === "exit") {
          const err = verifyENOENT(arg1, parsed);
          if (err) {
            return originalEmit.call(cp, "error", err);
          }
        }
        return originalEmit.apply(cp, arguments);
      };
    }
    function verifyENOENT(status, parsed) {
      if (isWin && status === 1 && !parsed.file) {
        return notFoundError(parsed.original, "spawn");
      }
      return null;
    }
    function verifyENOENTSync(status, parsed) {
      if (isWin && status === 1 && !parsed.file) {
        return notFoundError(parsed.original, "spawnSync");
      }
      return null;
    }
    module2.exports = {
      hookChildProcess,
      verifyENOENT,
      verifyENOENTSync,
      notFoundError
    };
  }
});

// node_modules/cross-spawn/index.js
var require_cross_spawn = __commonJS({
  "node_modules/cross-spawn/index.js"(exports2, module2) {
    "use strict";
    var cp = require("child_process");
    var parse = require_parse();
    var enoent = require_enoent();
    function spawn2(command, args, options) {
      const parsed = parse(command, args, options);
      const spawned = cp.spawn(parsed.command, parsed.args, parsed.options);
      enoent.hookChildProcess(spawned, parsed);
      return spawned;
    }
    function spawnSync(command, args, options) {
      const parsed = parse(command, args, options);
      const result = cp.spawnSync(parsed.command, parsed.args, parsed.options);
      result.error = result.error || enoent.verifyENOENTSync(result.status, parsed);
      return result;
    }
    module2.exports = spawn2;
    module2.exports.spawn = spawn2;
    module2.exports.sync = spawnSync;
    module2.exports._parse = parse;
    module2.exports._enoent = enoent;
  }
});

// src/extension.ts
var extension_exports = {};
__export(extension_exports, {
  activate: () => activate,
  deactivate: () => deactivate
});
module.exports = __toCommonJS(extension_exports);
var vscode4 = __toESM(require("vscode"));

// src/cli.ts
var vscode = __toESM(require("vscode"));
var path = __toESM(require("path"));
var fs = __toESM(require("fs"));
var import_cross_spawn = __toESM(require_cross_spawn());
function resolveBinaryPath() {
  const config = vscode.workspace.getConfiguration("dothttp");
  const configured = config.get("binaryPath") ?? "";
  if (configured.trim() !== "") {
    return configured.trim();
  }
  const candidates = process.platform === "win32" ? ["dothttp.exe", "dothttp-cli.exe"] : ["dothttp", "dothttp-cli"];
  const pathDirs = (process.env.PATH ?? "").split(path.delimiter);
  for (const dir of pathDirs) {
    for (const name of candidates) {
      const full = path.join(dir, name);
      try {
        fs.accessSync(full, fs.constants.X_OK);
        return full;
      } catch {
      }
    }
  }
  return null;
}
function runCliRequest(binaryPath, filePath, identifier) {
  return new Promise((resolve, reject) => {
    const config = vscode.workspace.getConfiguration("dothttp");
    const defaultEnv = config.get("defaultEnvironment") ?? "";
    const args = ["--file", filePath, "--request", identifier];
    if (defaultEnv.trim() !== "") {
      args.push("--env", defaultEnv.trim());
    }
    let stdout = "";
    let stderr = "";
    const child = (0, import_cross_spawn.default)(binaryPath, args);
    child.stdout?.on("data", (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr?.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("close", (code) => {
      if (code === 0) {
        resolve({ stdout, stderr });
      } else {
        reject(new Error(stderr || `dothttp-cli exited with code ${code}`));
      }
    });
    child.on("error", (err) => {
      reject(err);
    });
  });
}

// src/responsePanel.ts
var vscode2 = __toESM(require("vscode"));
var MAX_BODY_BYTES = 1 * 1024 * 1024;
function parseCliOutput(output) {
  const lines = output.split(/\r?\n/);
  let statusLine = "";
  const headers = [];
  let bodyLines = [];
  let state = "pre";
  for (const line of lines) {
    if (state === "pre") {
      if (line.trim().startsWith("\u2500\u2500\u2500")) {
        state = "status";
      }
      continue;
    }
    if (state === "status") {
      statusLine = line.trim();
      state = "headers";
      continue;
    }
    if (state === "headers") {
      if (line.trim() === "") {
        state = "body";
        continue;
      }
      const colonIdx = line.indexOf(":");
      if (colonIdx > 0) {
        headers.push([line.slice(0, colonIdx).trim(), line.slice(colonIdx + 1).trim()]);
      }
      continue;
    }
    if (state === "body") {
      bodyLines.push(line);
    }
  }
  let body = bodyLines.join("\n").trimEnd();
  let truncated = false;
  const bodyBytes = Buffer.byteLength(body, "utf8");
  if (bodyBytes > MAX_BODY_BYTES) {
    body = Buffer.from(body, "utf8").subarray(0, MAX_BODY_BYTES).toString("utf8");
    truncated = true;
  }
  try {
    const parsed = JSON.parse(body);
    body = JSON.stringify(parsed, null, 2);
  } catch {
  }
  return { statusLine, headers, body, truncated };
}
function escapeHtml(text) {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}
function statusColor(statusLine) {
  const match = statusLine.match(/\d+/);
  if (!match)
    return "#cccccc";
  const code = parseInt(match[0], 10);
  if (code >= 200 && code < 300)
    return "#4ec9b0";
  if (code >= 300 && code < 400)
    return "#dcdcaa";
  if (code >= 400)
    return "#f48771";
  return "#cccccc";
}
function buildHtml(response) {
  const color = statusColor(response.statusLine);
  const headerRows = response.headers.map(([k, v]) => `<tr><td class="hk">${escapeHtml(k)}</td><td>${escapeHtml(v)}</td></tr>`).join("\n");
  const truncationNotice = response.truncated ? `<p class="truncation">Response body truncated at 1 MB.</p>` : "";
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<style>
  body { font-family: var(--vscode-editor-font-family, monospace); font-size: 13px; padding: 12px; color: var(--vscode-editor-foreground); background: var(--vscode-editor-background); }
  .status { font-size: 1.2em; font-weight: bold; color: ${color}; margin-bottom: 12px; }
  table { border-collapse: collapse; margin-bottom: 12px; width: 100%; }
  td { padding: 2px 8px; vertical-align: top; }
  .hk { color: var(--vscode-symbolIcon-fieldForeground, #9cdcfe); font-weight: bold; white-space: nowrap; }
  pre { background: var(--vscode-textBlockQuote-background); padding: 10px; border-radius: 4px; overflow-x: auto; white-space: pre-wrap; word-break: break-word; }
  .truncation { color: var(--vscode-editorWarning-foreground, #cca700); font-style: italic; }
  h3 { margin: 8px 0 4px; color: var(--vscode-descriptionForeground); font-size: 0.85em; text-transform: uppercase; letter-spacing: 0.05em; }
</style>
</head>
<body>
<div class="status">${escapeHtml(response.statusLine)}</div>
${response.headers.length > 0 ? `<h3>Headers</h3><table>${headerRows}</table>` : ""}
${response.body ? `<h3>Body</h3>${truncationNotice}<pre>${escapeHtml(response.body)}</pre>` : ""}
</body>
</html>`;
}
var ResponsePanel = class _ResponsePanel {
  constructor(panel) {
    this.panel = panel;
    panel.onDidDispose(() => {
      _ResponsePanel.current = void 0;
    });
  }
  static show(output) {
    if (_ResponsePanel.current) {
      _ResponsePanel.current.panel.reveal(vscode2.ViewColumn.Beside);
    } else {
      const panel = vscode2.window.createWebviewPanel(
        "dothttpResponse",
        "dothttp Response",
        vscode2.ViewColumn.Beside,
        { enableScripts: false }
      );
      _ResponsePanel.current = new _ResponsePanel(panel);
    }
    const parsed = parseCliOutput(output);
    _ResponsePanel.current.panel.webview.html = buildHtml(parsed);
  }
};

// src/codeLensProvider.ts
var vscode3 = __toESM(require("vscode"));
var SEPARATOR_RE = /^###\s*(.*)$/;
var REQUEST_LINE_RE = /^(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE)\s+(\S+)/;
function findRequests(document) {
  const entries = [];
  const lineCount = document.lineCount;
  let i = 0;
  while (i < lineCount) {
    const text = document.lineAt(i).text;
    const sepMatch = SEPARATOR_RE.exec(text);
    if (sepMatch !== null) {
      const name = sepMatch[1].trim();
      if (name !== "") {
        entries.push({ line: i, identifier: name });
      } else {
        const reqLine = findNextRequestLine(document, i + 1);
        if (reqLine !== null) {
          entries.push({ line: i, identifier: reqLine.identifier });
        }
      }
      i++;
      continue;
    }
    const reqMatch = REQUEST_LINE_RE.exec(text);
    if (reqMatch !== null) {
      const identifier = `${reqMatch[1]} ${reqMatch[2]}`;
      const alreadyCovered = entries.some((e) => {
        return e.identifier === identifier && e.line < i;
      });
      if (!alreadyCovered) {
        entries.push({ line: i, identifier });
      }
    }
    i++;
  }
  return entries;
}
function findNextRequestLine(document, fromLine) {
  for (let i = fromLine; i < document.lineCount; i++) {
    const text = document.lineAt(i).text;
    const match = REQUEST_LINE_RE.exec(text);
    if (match) {
      return { identifier: `${match[1]} ${match[2]}` };
    }
    if (SEPARATOR_RE.test(text)) {
      break;
    }
  }
  return null;
}
var DothttpCodeLensProvider = class {
  constructor(context) {
    this.context = context;
    this.changeEmitter = new vscode3.EventEmitter();
    this.onDidChangeCodeLenses = this.changeEmitter.event;
    vscode3.workspace.onDidChangeTextDocument(
      () => this.changeEmitter.fire(),
      null,
      context.subscriptions
    );
  }
  provideCodeLenses(document) {
    const requests = findRequests(document);
    return requests.map(({ line, identifier }) => {
      const range = new vscode3.Range(line, 0, line, 0);
      return new vscode3.CodeLens(range, {
        title: "\u25B6 Run",
        command: "dothttp.runRequest",
        arguments: [document.uri.fsPath, identifier]
      });
    });
  }
};

// src/extension.ts
function activate(context) {
  const binary = resolveBinaryPath();
  if (binary === null) {
    vscode4.window.showWarningMessage(
      "dothttp: CLI binary not found. Configure the path in settings.",
      "Open Settings"
    ).then((choice) => {
      if (choice === "Open Settings") {
        vscode4.commands.executeCommand(
          "workbench.action.openSettings",
          "dothttp.binaryPath"
        );
      }
    });
  }
  const codeLensProvider = new DothttpCodeLensProvider(context);
  context.subscriptions.push(
    vscode4.languages.registerCodeLensProvider({ language: "dothttp" }, codeLensProvider)
  );
  const runRequestCmd = vscode4.commands.registerCommand(
    "dothttp.runRequest",
    async (filePath, identifier) => {
      if (!filePath || !identifier) {
        vscode4.window.showErrorMessage(
          "dothttp.runRequest requires a file path and request identifier."
        );
        return;
      }
      const bin = resolveBinaryPath();
      if (bin === null) {
        vscode4.window.showErrorMessage(
          "dothttp: CLI binary not found. Configure the path in settings.",
          "Open Settings"
        ).then((choice) => {
          if (choice === "Open Settings") {
            vscode4.commands.executeCommand(
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
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        vscode4.window.showErrorMessage(`dothttp error: ${message}`);
      }
    }
  );
  context.subscriptions.push(runRequestCmd);
}
function deactivate() {
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  activate,
  deactivate
});
