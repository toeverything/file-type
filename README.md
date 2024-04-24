# `@toeverything/file-type`

![https://github.com/toeverything/file-type/actions](https://github.com/toeverything/file-type/workflows/CI/badge.svg)

# Usage

```ts
import { readFile } from 'node:fs'

import { FileType } from '@toeverything/file-type'

const fileType = new FileType(await readFile('path/to/file.jpg'))
console.log(fileType.mime()) // 'application/image/jpeg'
console.log(fileType.ext()) // 'jpg'
```

# Performance

```txt
┌─────────┬──────────────────────┬─────────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name            │ ops/sec     │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼──────────────────────┼─────────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@toeverything/file-type' │ '1,505,155' │ 664.3832625987645  │ '±2.22%' │ 752578  │
│ 1       │ 'file-type'          │ '663,435'   │ 1507.3055818495407 │ '±0.36%' │ 331718  │
└─────────┴──────────────────────┴─────────────┴────────────────────┴──────────┴─────────┘
```
