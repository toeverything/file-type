/* auto-generated by NAPI-RS */
/* eslint-disable */
export class FileType {
  constructor(bytes: Uint8Array)
  /**
   * Returns the common extension of the file format.
   *
   * Note: this information is never empty.
   *
   * # Examples
   *
   * Basic usage:
   *
   * ```
   * new FileType([...]).extension() // "wmv"
   *```
   */
  extension(): string
  /**
   * Returns the `Kind` of the file format.
   *
   * # Examples
   *
   * Basic usage:
   *
   * ```
   * new FileType([...]).kind() // Kind.Archive
   *```
   */
  kind(): Kind
  /**
   * Returns the common media type (formerly known as MIME type) of the file format as
   * defined in [IETF RFC 6838](https://tools.ietf.org/html/rfc6838).
   *
   * Note: some media types may not be defined in the
   * [IANA registry](https://www.iana.org/assignments/media-types/media-types.xhtml).
   *
   * # Examples
   *
   * Basic usage:
   *
   * ```
   * new FileType([...]).media_type() // "application/zstd"
   *```
   */
  mime(): string
  /**
   * Returns the full name of the file format.
   *
   * # Examples
   *
   * Basic usage:
   *
   * ```
   * new FileType([...]).name() // "MPEG-1/2 Audio Layer 3"
   *```
   */
  name(): string
}

/** A kind of file format. */
export const enum Kind {
  /** Files and directories stored in a single, possibly compressed, archive. */
  Archive = 0,
  /** Musics, sound effects, and spoken audio recordings. */
  Audio = 1,
  /** Compressed single files or streams. */
  Compressed = 2,
  /** Organized collections of data. */
  Database = 3,
  /** Visual information using graphics and spatial relationships. */
  Diagram = 4,
  /** Floppy disk images, optical disc images and virtual machine disks. */
  Disk = 5,
  /** Word processing and desktop publishing documents. */
  Document = 6,
  /** Electronic books. */
  Ebook = 7,
  /** Machine-executable code, virtual machine code and shared libraries. */
  Executable = 8,
  /** Typefaces used for displaying text on screen or in print. */
  Font = 9,
  /** Mathematical formulas. */
  Formula = 10,
  /** Collections of geospatial features, GPS tracks and other location-related files. */
  Geospatial = 11,
  /** Animated images, icons, cursors, raster graphics and vector graphics. */
  Image = 12,
  /** Data that provides information about other data. */
  Metadata = 13,
  /** 3D models, CAD drawings, and other types of files used for creating or displaying 3D images. */
  Model = 14,
  /** Data which do not fit in any of the other kinds. */
  Other = 15,
  /** Collections of files bundled together for software distribution. */
  Package = 16,
  /** Lists of audio or video files, organized in a specific order for sequential playback. */
  Playlist = 17,
  /** Slide shows. */
  Presentation = 18,
  /** Copies of a read-only memory chip of computers, cartridges, or other electronic devices. */
  Rom = 19,
  /** Data in tabular form. */
  Spreadsheet = 20,
  /** Subtitles and captions. */
  Subtitle = 21,
  /** Moving images, possibly with color and coordinated sound. */
  Video = 22
}

