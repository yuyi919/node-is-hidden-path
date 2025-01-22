import { Bench } from 'tinybench'

import { isHiddenFile } from '../index.js'


const b = new Bench()


b.add('Native isHiddenFileApi', () => {
  isHiddenFile('D:\\AlphaDiscLog.txt')
})

// b.add('JavaScript a + 100', () => {
// })

await b.run()

console.table(b.table())
