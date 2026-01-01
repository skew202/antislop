const fs = require('fs');
const path = require('path');
const https = require('https');
const os = require('os');
const { execSync } = require('child_process');

const VERSION = "v0.2.0";
const BIN_DIR = path.join(__dirname, 'bin');
const BIN_NAME = os.platform() === 'win32' ? 'antislop.exe' : 'antislop';

// Check if binary already exists
const binPath = path.join(BIN_DIR, BIN_NAME);
if (fs.existsSync(binPath)) {
  process.exit(0);
}

const platform = os.platform();
const arch = os.arch();

let target = '';
if (platform === 'linux' && arch === 'x64') target = 'x86_64-unknown-linux-gnu';
else if (platform === 'linux' && arch === 'arm64') target = 'aarch64-unknown-linux-gnu';
else if (platform === 'darwin' && arch === 'x64') target = 'x86_64-apple-darwin';
else if (platform === 'darwin' && arch === 'arm64') target = 'aarch64-apple-darwin';
else if (platform === 'win32' && arch === 'x64') target = 'x86_64-pc-windows-msvc';
else {
  console.error(`Unsupported platform: ${platform} ${arch}`);
  console.error('Please install antislop via cargo: cargo install antislop');
  process.exit(1);
}

const ext = platform === 'win32' ? 'zip' : 'tar.gz';
const url = `https://github.com/skew202/antislop/releases/download/${VERSION}/antislop-${target}.${ext}`;
const dest = path.join(BIN_DIR, `download.${ext}`);

if (!fs.existsSync(BIN_DIR)) {
  fs.mkdirSync(BIN_DIR, { recursive: true });
}

console.log(`Downloading antislop ${VERSION} for ${target}...`);

downloadFile(url, dest, () => {
  console.log('Extracting...');
  try {
    if (platform === 'win32') {
      execSync(`powershell -Command "Expand-Archive -Path '${dest}' -DestinationPath '${BIN_DIR}' -Force"`);
    } else {
      execSync(`tar -xzf "${dest}" -C "${BIN_DIR}"`);
    }
    fs.unlinkSync(dest);
    console.log('Installed antislop successfully.');
  } catch (e) {
    console.error('Failed to extract:', e.message);
    process.exit(1);
  }
});

function downloadFile(url, dest, callback) {
  const file = fs.createWriteStream(dest);

  https.get(url, (response) => {
    if (response.statusCode === 302 || response.statusCode === 301) {
      https.get(response.headers.location, (res) => res.pipe(file));
    } else {
      response.pipe(file);
    }
    file.on('finish', () => {
      file.close();
      callback();
    });
  }).on('error', (err) => {
    fs.unlink(dest, () => {});
    console.error('Download failed:', err.message);
    process.exit(1);
  });
}
