#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub struct FileType(file_format::FileFormat);

/// A kind of file format.
#[napi]
pub enum Kind {
  /// Files and directories stored in a single, possibly compressed, archive.
  Archive,
  /// Musics, sound effects, and spoken audio recordings.
  Audio,
  /// Compressed single files or streams.
  Compressed,
  /// Organized collections of data.
  Database,
  /// Visual information using graphics and spatial relationships.
  Diagram,
  /// Floppy disk images, optical disc images and virtual machine disks.
  Disk,
  /// Word processing and desktop publishing documents.
  Document,
  /// Electronic books.
  Ebook,
  /// Machine-executable code, virtual machine code and shared libraries.
  Executable,
  /// Typefaces used for displaying text on screen or in print.
  Font,
  /// Mathematical formulas.
  Formula,
  /// Collections of geospatial features, GPS tracks and other location-related files.
  Geospatial,
  /// Animated images, icons, cursors, raster graphics and vector graphics.
  Image,
  /// Data that provides information about other data.
  Metadata,
  /// 3D models, CAD drawings, and other types of files used for creating or displaying 3D images.
  Model,
  /// Data which do not fit in any of the other kinds.
  Other,
  /// Collections of files bundled together for software distribution.
  Package,
  /// Lists of audio or video files, organized in a specific order for sequential playback.
  Playlist,
  /// Slide shows.
  Presentation,
  /// Copies of a read-only memory chip of computers, cartridges, or other electronic devices.
  Rom,
  /// Data in tabular form.
  Spreadsheet,
  /// Subtitles and captions.
  Subtitle,
  /// Moving images, possibly with color and coordinated sound.
  Video,
}

impl From<file_format::Kind> for Kind {
  fn from(value: file_format::Kind) -> Self {
    match value {
      file_format::Kind::Archive => Self::Archive,
      file_format::Kind::Audio => Self::Audio,
      file_format::Kind::Compressed => Self::Compressed,
      file_format::Kind::Database => Self::Database,
      file_format::Kind::Diagram => Self::Diagram,
      file_format::Kind::Disk => Self::Disk,
      file_format::Kind::Document => Self::Document,
      file_format::Kind::Ebook => Self::Ebook,
      file_format::Kind::Executable => Self::Executable,
      file_format::Kind::Font => Self::Font,
      file_format::Kind::Formula => Self::Formula,
      file_format::Kind::Geospatial => Self::Geospatial,
      file_format::Kind::Image => Self::Image,
      file_format::Kind::Metadata => Self::Metadata,
      file_format::Kind::Model => Self::Model,
      file_format::Kind::Other => Self::Other,
      file_format::Kind::Package => Self::Package,
      file_format::Kind::Playlist => Self::Playlist,
      file_format::Kind::Presentation => Self::Presentation,
      file_format::Kind::Rom => Self::Rom,
      file_format::Kind::Spreadsheet => Self::Spreadsheet,
      file_format::Kind::Subtitle => Self::Subtitle,
      file_format::Kind::Video => Self::Video,
    }
  }
}

#[napi]
impl FileType {
  #[napi(constructor)]
  pub fn new(bytes: &[u8]) -> Self {
    Self(file_format::FileFormat::from_bytes(bytes))
  }

  #[napi]
  /// Returns the common extension of the file format.
  ///
  /// Note: this information is never empty.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// new FileType([...]).extension() // "wmv"
  ///```
  pub fn extension(&self) -> &str {
    self.0.extension()
  }

  #[napi]
  /// Returns the `Kind` of the file format.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// new FileType([...]).kind() // Kind.Archive
  ///```
  pub fn kind(&self) -> Kind {
    self.0.kind().into()
  }

  #[napi]
  /// Returns the common media type (formerly known as MIME type) of the file format as
  /// defined in [IETF RFC 6838](https://tools.ietf.org/html/rfc6838).
  ///
  /// Note: some media types may not be defined in the
  /// [IANA registry](https://www.iana.org/assignments/media-types/media-types.xhtml).
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// new FileType([...]).media_type() // "application/zstd"
  ///```
  pub fn mime(&self) -> &str {
    self.0.media_type()
  }

  #[napi]
  /// Returns the full name of the file format.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// new FileType([...]).name() // "MPEG-1/2 Audio Layer 3"
  ///```
  pub fn name(&self) -> &str {
    self.0.name()
  }
}
