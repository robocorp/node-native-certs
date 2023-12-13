const fs = require('fs');
const path = require('path');

const binariesDir = path.join(__dirname, 'binaries');
const osType = process.platform;
const cpuArch = process.arch;

let binaryName;
switch (osType) {
  case 'win32':
    binaryName = 'index-windows-latest.node';
    break;
  case 'linux':
    binaryName = 'index-ubuntu-latest.node';
    break;
  case 'darwin':
    if (cpuArch === 'arm' || cpuArch === 'arm64') {
      binaryName = 'index-macos-arm-latest.node';  
    } else {
      binaryName = 'index-macos-latest.node';
    }
    break;
  default:
    throw new Error(`Unsupported platform: ${osType}`);
}

const binaryPath = path.join(binariesDir, binaryName);
const targetPath = path.join(__dirname, 'index.node');

if (!fs.existsSync(binaryPath)) {
    console.log(`Binary not found: ${binaryPath}. Skipping setup.`);
    process.exit(0); // Exit gracefully
}

fs.renameSync(binaryPath, targetPath);
