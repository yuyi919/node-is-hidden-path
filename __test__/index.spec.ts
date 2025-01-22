import test from 'ava'

import { isHiddenFile } from '../index'

test('sync function from native code', (t) => {
  t.is(isHiddenFile('D:/AlphaDiscLog.txt'), true)
  t.is(isHiddenFile('D:\\D.zip'), false)
  t.is(isHiddenFile('D:/workspace'), false)
})
