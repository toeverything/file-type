# `@napi-rs/file-type`

![https://github.com/Brooooooklyn/file-type/actions](https://github.com/Brooooooklyn/file-type/workflows/CI/badge.svg)

# Usage

```ts
import { readFile } from 'node:fs'

import { FileType } from '@napi-rs/file-type'

const fileType = new FileType(await readFile('path/to/file.jpg'))
console.log(fileType.mime()) // 'application/image/jpeg'
console.log(fileType.ext()) // 'jpg'
```
