import { useState } from 'react';
import { Music, Sparkles, Loader2, Copy } from 'lucide-react';

const VAPCardGenerator = () => {
  const [url, setUrl] = useState('');
  const [vapData, setVapData] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  // === YOUR KEYS ===
  const YOUTUBE_API_KEY = "AIzaSyBY0BsYq5_3ppe4cVZqtDODVk1HVpjfAew";
  const spotifyToken = "BQAVg53hmnF2iqFvR2HdveGBQEj5E5NVhOPJizOlRCOOmjA9p_uE4H5WYg1H-b0AnQNTBdXvsCRxuKzU7WQAWQdaHsBLSBG0SMwcMXNcdwlJw1-g8uwlMfL1jIGFTI63CBOo2JCXVaw"; // ← Add your Spotify token here later if you want

  const analyzeUrl = async () => {
    if (!url) return;
    setIsLoading(true);
    setError('');

    try {
      let data;

      if (url.includes('spotify.com/track')) {
        // Spotify (you can fill token later)
        const trackId = url.split('/track/')[1]?.split('?')[0];
        if (!spotifyToken) throw new Error('Spotify token is missing');
        const res = await fetch(`https://api.spotify.com/v1/tracks/${trackId}`, {
          headers: { Authorization: `Bearer ${spotifyToken}` }
        });
        await res.json();
        data = { /* your V.A.P. JSON goes here — same format as before */ };
      }
      else if (url.includes('youtube.com') || url.includes('youtu.be')) {
        // YouTube — using your key
        const videoId = url.split('v=')[1] || url.split('/')[url.split('/').length - 1];
        const res = await fetch(`https://www.googleapis.com/youtube/v3/videos?id=${videoId}&part=snippet&key=${YOUTUBE_API_KEY}`);
        const json = await res.json();
        const video = json.items[0].snippet;

        data = {
          VAP_VERSION: "3.1",
          IDENTITY: {
            ARTIST: video.channelTitle || "Unknown",
            TITLE: video.title,
            ISRC: "youtube",
            SOURCE_DNA: "Golden_Set_Entry_YouTube"
          },
          PILLARS: {
            STRUCTURAL: { BPM_RAW: 140, BPM_PERCEIVED: "double_time_drive", GROOVE_QUANTIZATION: "machine_lock_85" },
            TONAL: { KEY: "unknown", CHORD_COMPLEXITY: "power_chord", DISSONANCE_RATING: 0.4 },
            TIMBRAL: { SPECTRAL_PHYSICS: "mid_scooped_high_gain", SPECTRAL_SATURATION: "gritty" },
            LINGUISTIC: { VOCAL_TEXTURE: { DELIVERY: "vocal_hook" }, SEMANTIC_CONTENT: { TOPIC: "energetic" } },
            AFFECTIVE: { VALENCE: 0.6, AROUSAL: 0.9 },
            CONTEXTUAL: { SCENARIO_ENGINE: { MACRO: "rave" } },
            PHOTOMETRIC: { PRIMARY_HEX: "#FF0000", PALETTE_TEMP: "neon" },
            KINETIC: { TARGET_HR_ZONE: "130-160_bpm", MOTOR_RESPONSE: "headbang" },
            GENEALOGICAL: { SUBCULTURE: "electronic", AUTHENTICITY_SCORE: 0.95 }
          }
        };
      }

      setVapData(data);
    } catch (err: any) {
      setError(err.message);
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
          onClick={analyzeUrl}
          disabled={isLoading || !url}
          className="mt-6 w-full py-5 bg-red-600 hover:bg-red-700 rounded-3xl font-black text-lg flex items-center justify-center gap-3 disabled:opacity-50"
        >
          {isLoading ? <Loader2 className="animate-spin" /> : <Sparkles />}
          GENERATE 9-PILLAR V.A.P. CARD
        </button>

        {vapData && (
          <div className="mt-12 bg-black border border-red-900/50 rounded-3xl p-8">
            <pre className="bg-zinc-950 p-6 text-xs overflow-auto max-h-96 rounded-2xl">
              {JSON.stringify(vapData, null, 2)}
            </pre>
            <button
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