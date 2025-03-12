import { Bench } from 'tinybench'

import { isHiddenFile, isHiddenFileWin32, isHiddenFileWin322 } from '../index.js'

const b = new Bench()

b.add('Native isHiddenFileWin32', () => {
  !isHiddenFileWin32('D:\\D.zip')
  isHiddenFileWin32('D:\\AlphaDiscLog.txt')
  !isHiddenFileWin32('D:/workspace')
  !isHiddenFileWin32('D:/素材/')
  isHiddenFileWin32('C:/Documents and Settings/')
  isHiddenFileWin32('C:/System Volume Information')
  // isHiddenFileWin32('C:/hiberfil.sys')
  // isHiddenFileWin32('C:/DumpStack.log.tmp')
})

b.add('Native isHiddenFileWin32 2', () => {
  !isHiddenFileWin322('D:\\D.zip')
  isHiddenFileWin322('D:\\AlphaDiscLog.txt')
  !isHiddenFileWin322('D:/workspace/')
  !isHiddenFileWin322('D:/素材/')
  isHiddenFileWin322('C:/Documents and Settings')
  isHiddenFileWin322('C:/System Volume Information')
  // isHiddenFileWin322('C:/hiberfil.sys')
  // isHiddenFileWin322('C:/DumpStack.log.tmp')
})
b.add('Native isHiddenFile', () => {
  !isHiddenFile('D:\\D.zip')
  isHiddenFile('D:\\AlphaDiscLog.txt')
  !isHiddenFile('D:/workspace')
  !isHiddenFile('D:/素材/')
  isHiddenFile('C:/Documents and Settings/')
  isHiddenFile('C:/System Volume Information')
  // isHiddenFile('C:/hiberfil.sys')
  // isHiddenFile('C:/DumpStack.log.tmp')
})

// b.add('JavaScript a + 100', () => {
// })

await b.run()

console.log(b.table())
