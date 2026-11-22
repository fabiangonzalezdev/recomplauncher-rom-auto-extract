const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const root = path.join(__dirname, '..');
const plugin = fs.readFileSync(path.join(root, 'docs/examples/rom-auto-extract/app.js'), 'utf8');

test('plugin reports native support and explains an unsupported launcher', async () => {
  for (const [info, state] of [[{ features: ['rom.unpack'] }, 'ready'], [{ launcher: '0.90.0-beta.67' }, 'unsupported']]) {
    const status = { dataset: {} };
    await vm.runInNewContext(plugin, { document: { getElementById: () => status }, rl: { info: async () => info } });
    assert.equal(status.dataset.state, state);
    assert.ok(status.textContent.length > 20);
  }
});

test('archive progress stays visible across jobs and disappears on completion or error', () => {
  const source = fs.readFileSync(path.join(root, 'ui/plugins.js'), 'utf8');
  const progress = source.slice(source.indexOf('const romArchiveJobs = new Map();'));
  let listener, status;
  const context = {
    T: { event: { listen: (name, callback) => { assert.equal(name, 'rom-archive'); listener = callback; } } },
    t: value => value,
    document: {
      getElementById: () => status,
      createElement: () => ({ setAttribute() {}, remove() { status = undefined; } }),
      body: { append: node => { status = node; } },
    },
  };
  vm.runInNewContext(progress, context);
  const emit = (id, phase) => listener({ payload: { id, phase } });
  emit('game-one', 'extracting');
  assert.equal(status.textContent, 'Extracting ROM…');
  emit('game-one', 'selecting');
  assert.equal(status.textContent, 'Selecting extracted ROM…');
  emit('game-two', 'extracting');
  emit('game-one', 'done');
  assert.equal(status.textContent, 'Extracting ROM…');
  emit('game-two', 'error');
  assert.equal(status, undefined);
});
