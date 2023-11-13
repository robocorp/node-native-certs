const fs = require('fs');
const path = require('path');

const binariesDir = path.join(__dirname, 'binaries');
const osType = process.platform;

let binaryName;
switch (osType) {
  case 'win32':
    binaryName = 'index-windows-latest.node';
    break;
  case 'linux':
    binaryName = 'index-ubuntu-latest.node';
    break;
  case 'darwin':
    binaryName = 'index-macos-latest.node';
    break;
  default:
    throw new Error(`Unsupported platform: ${osType}`);
}

const binaryPath = path.join(binariesDir, binaryName);
const targetPath = path.join(__dirname, 'index.node');

fs.renameSync(binaryPath, targetPath);
