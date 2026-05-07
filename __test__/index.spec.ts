import test from 'ava'
import path from 'path'

import { isHiddenFile, isHiddenFileWin32, isHiddenFileWin322 } from '../index.js'

test('sync function from native code', (t) => {
  t.is(isHiddenFile(process.cwd()), false)
  t.is(isHiddenFileWin32(process.cwd()), false)
  t.is(isHiddenFileWin322(process.cwd()), false)

  if (process.platform !== 'win32') {
    t.is(isHiddenFile('.hidden_file'), true)
    t.is(isHiddenFile('visible_file'), false)
    t.is(isHiddenFile(path.join(process.cwd(), '.gitignore')), true)
    t.is(isHiddenFile(path.join(process.cwd(), 'package.json')), false)
  }
})
