import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const source = root.endsWith('dashboard') ? root : path.join(root, 'dashboard');
const dist = path.join(source, 'dist');
const files = ['index.html', 'src/app.js', 'styles/main.css'];

for (const file of files) {
  const src = fs.readFileSync(path.join(source, file), 'utf8');
  const built = fs.readFileSync(path.join(dist, file), 'utf8');
  if (src !== built) throw new Error(`dist out of sync: ${file}`);
}
const app = fs.readFileSync(path.join(source, 'src/app.js'), 'utf8');
const required = [
  'const snapshot = new Date(',
  "document.getElementById('cooling-ratio')",
  "document.getElementById('snapshot-time')",
];
for (const token of required) {
  if (!app.includes(token)) throw new Error(`dashboard app missing: ${token}`);
}
console.log('dashboard dist and snapshot data verified');
