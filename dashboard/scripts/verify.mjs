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
console.log('dashboard dist verified');
