import { readFile } from 'node:fs/promises'

import { FileType } from './index.js'

const ft = new FileType(await readFile('./__test__/sample.jpg'))

console.log(ft.mime())
