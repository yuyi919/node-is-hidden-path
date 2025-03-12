import test from 'ava'

import { isHiddenFile, isHiddenFileWin32, isHiddenFileWin322 } from '../index'

test('sync function from native code', (t) => {
  // console.log(
  //   !isHiddenFileWin322('D:\\D.zip'),
  //   isHiddenFileWin322('D:\\AlphaDiscLog.txt'),
  //   !isHiddenFileWin322('D:/workspace'),
  //   !isHiddenFileWin322('D:/素材/'),
  //   isHiddenFileWin322('C:/Documents and Settings/'),
  //   isHiddenFileWin322('C:/System Volume Information'),
  //   isHiddenFileWin322('C:/hiberfil.sys'),
  //   isHiddenFileWin322('C:/DumpStack.log.tmp'),
  // )
  // console.log(
  //   !isHiddenFileWin32('D:\\D.zip'),
  //   isHiddenFileWin32('D:\\AlphaDiscLog.txt'),
  //   !isHiddenFileWin32('D:/workspace'),
  //   !isHiddenFileWin32('D:/素材/'),
  //   isHiddenFileWin32('C:/Documents and Settings/'),
  //   isHiddenFileWin32('C:/System Volume Information'),
  //   isHiddenFileWin32('C:/hiberfil.sys'),
  //   isHiddenFileWin32('C:/DumpStack.log.tmp'),
  // )

  // console.log(
  //   !isHiddenFile('D:\\D.zip'),
  //   isHiddenFile('D:\\AlphaDiscLog.txt'),
  //   !isHiddenFile('D:/workspace'),
  //   !isHiddenFile('D:/素材/'),
  //   isHiddenFile('C:/Documents and Settings'),
  //   isHiddenFile('C:/System Volume Information'),
  //   isHiddenFile('C:/hiberfil.sys'),
  //   isHiddenFile('C:/DumpStack.log.tmp'),
  // )

  // t.is(isHiddenFile('D:/AlphaDiscLog.txt'), true)
  // t.is(isHiddenFile('D:\\D.zip'), false)
  // t.is(isHiddenFile('D:/workspace'), false)
  // t.is(isHiddenFile('D:\\D.zip'), false)
  t.is(isHiddenFile(process.cwd()), false)
})
