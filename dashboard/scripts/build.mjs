import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const source = root.endsWith('dashboard') ? root : path.join(root, 'dashboard');
const dist = path.join(source, 'dist');
fs.rmSync(dist, { recursive: true, force: true });
fs.mkdirSync(dist, { recursive: true });
for (const dir of ['src', 'styles']) fs.cpSync(path.join(source, dir), path.join(dist, dir), { recursive: true });
const html = fs.readFileSync(path.join(source, 'index.html'), 'utf8');
fs.writeFileSync(path.join(dist, 'index.html'), html);
console.log('dashboard built:', dist);
