/** Supported media extensions for Open File / Open Folder. */

export const MEDIA_EXTENSIONS = [
  'mp3',
  'm4a',
  'aac',
  'flac',
  'ogg',
  'oga',
  'opus',
  'wav',
  'aiff',
  'aif',
  'mp4',
  'm4v',
  'caf',
  'webm',
] as const;

export const MEDIA_ACCEPT = MEDIA_EXTENSIONS.map((e) => `.${e}`).join(',');

export function isMediaFile(name: string): boolean {
  const ext = name.split('.').pop()?.toLowerCase() ?? '';
  return (MEDIA_EXTENSIONS as readonly string[]).includes(ext);
}

export function formatLabel(name: string): string {
  const ext = name.split('.').pop()?.toUpperCase() ?? '?';
  return ext;
}

export type PlaylistEntry = {
  id: string;
  name: string;
  title: string;
  artist: string;
  format: string;
  /** Browser object URL for playback */
  objectUrl: string;
  file?: File;
  durationSec?: number;
};

export type FileMenuId =
  | 'open_file'
  | 'open_many'
  | 'open_folder'
  | 'open_disc'
  | 'open_recent'
  | 'stream'
  | 'convert_export'
  | 'create_playlist'
  | 'save_playlist'
  | 'edit_playlist'
  | 'open_network'
  | 'save_quit'
  | 'quit';

export const FILE_MENU: { id: FileMenuId; label: string; shortcut?: string; sepAfter?: boolean }[] =
  [
    { id: 'open_file', label: 'Open File…', shortcut: 'Ctrl+O' },
    { id: 'open_many', label: 'Open Many Files…', shortcut: 'Ctrl+Shift+O' },
    { id: 'open_folder', label: 'Open Folder…', shortcut: 'Ctrl+F' },
    { id: 'open_disc', label: 'Open Disc…', sepAfter: true },
    { id: 'open_recent', label: 'Open Recent Media' },
    { id: 'stream', label: 'Stream…', shortcut: 'Ctrl+N' },
    { id: 'convert_export', label: 'Convert / Export…', sepAfter: true },
    { id: 'create_playlist', label: 'Create Playlist' },
    { id: 'save_playlist', label: 'Save Playlist…', shortcut: 'Ctrl+S' },
    { id: 'edit_playlist', label: 'Edit Playlist…', sepAfter: true },
    { id: 'open_network', label: 'Open Network Device…', sepAfter: true },
    { id: 'save_quit', label: 'Save & Quit', shortcut: 'Ctrl+Q' },
    { id: 'quit', label: 'Quit' },
  ];
