import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('IPC contract sử dụng camelCase arguments', () => {
  const source = readFileSync(new URL('./api.js', import.meta.url), 'utf8');
  assert.match(source, /invoke\('app_set_theme', \{ theme \}\)/);
  assert.match(source, /invoke\('app_set_typography', \{ fontFamily, fontScale \}\)/);
  assert.match(source, /invoke\('note_update', \{ path, content, expectedHash \}\)/);
});
