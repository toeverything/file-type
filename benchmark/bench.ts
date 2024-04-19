import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { join } from 'node:path'

import { Bench } from 'tinybench'
import { fileTypeFromBuffer } from 'file-type'

import { FileType } from '../index.js'

const b = new Bench()

const FIXTURE = await readFile(join(fileURLToPath(import.meta.url), '..', '..', '__test__', 'sample.jpg'))

b.add('@napi-rs/file-type', () => {
  const ft = new FileType(FIXTURE)
  ft.extension()
}).add('file-type', async () => {
  await fileTypeFromBuffer(FIXTURE)
})

await b.warmup()
await b.run()
console.table(b.table())
