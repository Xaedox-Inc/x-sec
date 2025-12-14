const fs = require('fs');

// Minimal valid PNG file (1x1 blue pixel)
const minimalPng = Buffer.from(
  'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==',
  'base64'
);

// Create all required icon files
['32x32.png', '128x128.png', '128x128@2x.png', 'icon.png', 'icon.ico', 'icon.icns'].forEach(name => {
  fs.writeFileSync(name, minimalPng);
  console.log(`Created ${name}`);
});
