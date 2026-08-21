import { useState } from 'react';
import { Music, Sparkles, Loader2, Copy } from 'lucide-react';

/**
 * Lightweight V.A.P. card generator for URL demos.
 * API keys must come from Vite env (never commit secrets).
 * VITE_YOUTUBE_API_KEY / VITE_SPOTIFY_TOKEN
 */
const VAPCardGenerator = () => {
  const [url, setUrl] = useState('');
  const [vapData, setVapData] = useState<unknown>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const youtubeKey = import.meta.env.VITE_YOUTUBE_API_KEY as string | undefined;
  const spotifyToken = import.meta.env.VITE_SPOTIFY_TOKEN as string | undefined;

  const analyzeUrl = async () => {
    if (!url) return;
    setIsLoading(true);
    setError('');

    try {
      let data;

      if (url.includes('spotify.com/track')) {
        const trackId = url.split('/track/')[1]?.split('?')[0];
        if (!spotifyToken) throw new Error('Set VITE_SPOTIFY_TOKEN in .env.local');
        const res = await fetch(`https://api.spotify.com/v1/tracks/${trackId}`, {
          headers: { Authorization: `Bearer ${spotifyToken}` },
        });
        if (!res.ok) throw new Error(`Spotify API ${res.status}`);
        await res.json();
        data = {
          VAP_VERSION: '3.1',
          IDENTITY: { ARTIST: 'Spotify', TITLE: trackId, SOURCE_DNA: 'spotify_stub' },
          PILLARS: {},
        };
      } else if (url.includes('youtube.com') || url.includes('youtu.be')) {
        if (!youtubeKey) throw new Error('Set VITE_YOUTUBE_API_KEY in .env.local');
        const videoId = url.includes('v=')
          ? url.split('v=')[1]?.split('&')[0]
          : url.split('/').pop();
        const res = await fetch(
          `https://www.googleapis.com/youtube/v3/videos?id=${videoId}&part=snippet&key=${youtubeKey}`,
        );
        const json = await res.json();
        const video = json.items?.[0]?.snippet;
        if (!video) throw new Error('Video not found');

        data = {
          VAP_VERSION: '3.1',
          IDENTITY: {
            ARTIST: video.channelTitle || 'Unknown',
            TITLE: video.title,
            ISRC: 'youtube',
            SOURCE_DNA: 'Golden_Set_Entry_YouTube',
          },
          PILLARS: {
            STRUCTURAL: {
              BPM_RAW: 140,
              BPM_PERCEIVED: 'double_time_drive',
              GROOVE_QUANTIZATION: 'machine_lock_85',
            },
            TONAL: { KEY: 'unknown', CHORD_COMPLEXITY: 'power_chord', DISSONANCE_RATING: 0.4 },
            TIMBRAL: { SPECTRAL_PHYSICS: 'mid_scooped_high_gain', SPECTRAL_SATURATION: 'gritty' },
            LINGUISTIC: {
              VOCAL_TEXTURE: { DELIVERY: 'vocal_hook' },
              SEMANTIC_CONTENT: { TOPIC: 'energetic' },
            },
            AFFECTIVE: { VALENCE: 0.6, AROUSAL: 0.9 },
            CONTEXTUAL: { SCENARIO_ENGINE: { MACRO: 'rave' } },
            PHOTOMETRIC: { PRIMARY_HEX: '#FF0000', PALETTE_TEMP: 'neon' },
            KINETIC: { TARGET_HR_ZONE: '130-160_bpm', MOTOR_RESPONSE: 'headbang' },
            GENEALOGICAL: { SUBCULTURE: 'electronic', AUTHENTICITY_SCORE: 0.95 },
          },
        };
      } else {
        throw new Error('Paste a Spotify track or YouTube URL');
      }

      setVapData(data);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#050505] text-zinc-200 p-8">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-4xl font-black mb-8 flex items-center gap-3">
          <Music className="text-red-500" /> V.A.P. Card Generator
        </h1>

        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Paste Spotify or YouTube URL..."
          className="w-full bg-black border border-red-900 rounded-3xl px-6 py-5 text-lg focus:outline-none"
        />

        <button
          type="button"
          onClick={analyzeUrl}
          disabled={isLoading || !url}
          className="mt-6 w-full py-5 bg-red-600 hover:bg-red-700 rounded-3xl font-black text-lg flex items-center justify-center gap-3 disabled:opacity-50"
        >
          {isLoading ? <Loader2 className="animate-spin" /> : <Sparkles />}
          GENERATE 9-PILLAR V.A.P. CARD
        </button>

        {vapData != null && (
          <div className="mt-12 bg-black border border-red-900/50 rounded-3xl p-8">
            <pre className="bg-zinc-950 p-6 text-xs overflow-auto max-h-96 rounded-2xl">
              {JSON.stringify(vapData, null, 2)}
            </pre>
            <button
              type="button"
              onClick={() => navigator.clipboard.writeText(JSON.stringify(vapData, null, 2))}
              className="mt-4 flex items-center gap-2 text-xs text-red-400"
            >
              <Copy size={14} /> Copy JSON
            </button>
          </div>
        )}

        {error && <p className="text-red-400 mt-4 text-center">{error}</p>}
      </div>
    </div>
  );
};

export default VAPCardGenerator;
