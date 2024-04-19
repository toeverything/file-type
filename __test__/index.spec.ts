import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import test from 'ava'

import { FileType, Kind } from '../index.js'

const dirname = join(fileURLToPath(import.meta.url), '..')

const JPG = readFileSync(join(dirname, 'sample.jpg'))

test('Common file type test', (t) => {
  const fileType = new FileType(JPG)
  t.is(fileType.extension(), 'jpg')
  t.is(fileType.mime(), 'image/jpeg')
  t.is(fileType.name(), 'Joint Photographic Experts Group')
  t.is(fileType.kind(), Kind.Image)
})
