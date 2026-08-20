import { o as __toESM } from "../_runtime.mjs";
import { o as require_jsx_runtime, s as require_react } from "../_libs/@radix-ui/react-collection+[...].mjs";
import { _ as Link } from "../_libs/@tanstack/react-router+[...].mjs";
import { S as Activity, _ as Heart, a as SkipBack, b as Eye, c as Repeat, d as Pause, f as Music2, g as Hexagon, h as Library, i as SkipForward, l as Repeat1, m as MapPin, o as Shuffle, p as MessageCircle, r as Trash2, s as Settings2, t as Waves, u as Play, v as GitBranch, x as AudioLines, y as FolderOpen } from "../_libs/lucide-react.mjs";
import { a as hashHue, i as formatTime, l as signOut, n as authClient, o as hexToRgb, r as cn, s as rgbToHex, t as Button, u as uid } from "./client-DvfM4FyA.mjs";
import { t as Drawer } from "../_libs/vaul.mjs";
import { n as create, t as persist } from "../_libs/zustand.mjs";
import { i as SliderTrack, n as SliderRange, r as SliderThumb, t as Slider$1 } from "../_libs/@radix-ui/react-slider+[...].mjs";
import { n as SwitchThumb, t as Switch$1 } from "../_libs/radix-ui__react-switch.mjs";
//#region node_modules/.nitro/vite/services/ssr/assets/routes-CKkxQbI_.js
var import_react = /* @__PURE__ */ __toESM(require_react());
var import_jsx_runtime = require_jsx_runtime();
function BottomSheet({ open, onOpenChange, title, eyebrow, children, height = "tall" }) {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Drawer.Root, {
		open,
		onOpenChange,
		children: /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(Drawer.Portal, { children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Drawer.Overlay, { className: "fixed inset-0 z-40 bg-black/55" }), /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(Drawer.Content, {
			className: cn("fixed inset-x-0 bottom-0 z-50 mx-auto flex max-w-lg flex-col rounded-t-3xl bg-surface text-fg shadow-sheet outline-none", height === "tall" ? "h-[min(86dvh,720px)]" : "h-[min(64dvh,560px)]"),
			children: [
				/* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
					className: "flex justify-center pt-3",
					children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", { className: "h-1 w-12 rounded-full bg-white/18" })
				}),
				/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
					className: "px-5 pb-2 pt-3",
					children: [eyebrow ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
						className: "text-[11px] font-medium uppercase tracking-[0.18em] text-accent",
						children: eyebrow
					}) : null, /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Drawer.Title, {
						className: "font-display text-xl font-semibold tracking-tight text-fg",
						children: title
					})]
				}),
				/* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
					className: "min-h-0 flex-1 overflow-y-auto px-5 pb-[max(1.5rem,env(safe-area-inset-bottom))]",
					children
				})
			]
		})] })
	});
}
var FFT = 1024;
function clamp01(n) {
	return n < 0 ? 0 : n > 1 ? 1 : n;
}
var VibeAudioEngine = class {
	ctx = null;
	master = null;
	analyser = null;
	mediaEl = null;
	mediaSrc = null;
	demo = null;
	freq = new Uint8Array(FFT / 2);
	time = new Uint8Array(FFT);
	volume = .72;
	lastBeatAt = 0;
	beatPulse = 0;
	prevBass = 0;
	simPhase = 0;
	bpm = 128;
	playing = false;
	mode = "idle";
	onEnded = null;
	onTime = null;
	timeTimer = null;
	isPlaying() {
		return this.playing;
	}
	getMediaElement() {
		return this.mediaEl;
	}
	setBpm(bpm) {
		this.bpm = Math.max(60, Math.min(200, bpm));
	}
	setVolume(v) {
		this.volume = clamp01(v);
		if (this.master && this.ctx) this.master.gain.setTargetAtTime(this.volume, this.ctx.currentTime, .04);
		if (this.mediaEl) this.mediaEl.volume = this.volume;
	}
	setEndedHandler(fn) {
		this.onEnded = fn;
	}
	setTimeHandler(fn) {
		this.onTime = fn;
	}
	/** Call synchronously from a click/tap so the AudioContext unlocks. */
	unlock() {
		if (typeof window === "undefined") return;
		if (this.ctx && this.ctx.state !== "closed") {
			if (this.ctx.state === "suspended") this.ctx.resume();
			return;
		}
		const ctx = new (window.AudioContext || window.webkitAudioContext)();
		const master = ctx.createGain();
		master.gain.value = this.volume;
		const analyser = ctx.createAnalyser();
		analyser.fftSize = FFT;
		analyser.smoothingTimeConstant = .72;
		analyser.minDecibels = -92;
		analyser.maxDecibels = -18;
		master.connect(analyser);
		analyser.connect(ctx.destination);
		this.ctx = ctx;
		this.master = master;
		this.analyser = analyser;
		this.freq = new Uint8Array(analyser.frequencyBinCount);
		this.time = new Uint8Array(analyser.fftSize);
		this.ensureMediaElement();
		if (ctx.state === "suspended") ctx.resume();
	}
	async ensure() {
		this.unlock();
		if (!this.ctx) throw new Error("Audio is not available in this browser.");
		if (this.ctx.state === "suspended") await Promise.race([this.ctx.resume(), new Promise((resolve) => window.setTimeout(resolve, 400))]);
		return this.ctx;
	}
	ensureMediaElement() {
		if (this.mediaEl) return this.mediaEl;
		const el = document.createElement("audio");
		el.preload = "auto";
		el.crossOrigin = "anonymous";
		el.setAttribute("playsinline", "true");
		el.volume = this.volume;
		el.addEventListener("ended", () => this.onEnded?.());
		el.addEventListener("timeupdate", () => {
			if (this.mode === "file") this.onTime?.(el.currentTime, el.duration || 0);
		});
		this.mediaEl = el;
		return el;
	}
	connectMedia() {
		if (!this.ctx || !this.master || !this.mediaEl) return;
		if (this.mediaSrc) return;
		try {
			this.mediaSrc = this.ctx.createMediaElementSource(this.mediaEl);
			this.mediaSrc.connect(this.master);
		} catch {}
	}
	async playDemo(synth) {
		const ctx = await this.ensure();
		this.stopDemo();
		this.ensureMediaElement().pause();
		this.bpm = synth.bpm;
		this.mode = "demo";
		this.playing = true;
		this.demo = startDemoSynth(ctx, this.master, synth);
		this.startTimeClock();
		this.publishMediaSession(true);
	}
	async playFile(url) {
		await this.ensure();
		this.stopDemo();
		const el = this.ensureMediaElement();
		this.connectMedia();
		if (el.src !== url) el.src = url;
		this.mode = "file";
		this.playing = true;
		el.volume = this.volume;
		try {
			await el.play();
		} catch {
			this.playing = false;
			throw new Error("Playback was blocked. Tap play again.");
		}
		this.startTimeClock();
		this.publishMediaSession(true);
	}
	async togglePause() {
		if (this.mode === "file" && this.mediaEl) {
			if (this.playing) {
				this.mediaEl.pause();
				this.playing = false;
				this.publishMediaSession(false);
			} else {
				await this.ensure();
				await this.mediaEl.play();
				this.playing = true;
				this.publishMediaSession(true);
			}
			return this.playing;
		}
		if (this.mode === "demo") {
			if (this.playing) {
				this.demo?.stop();
				this.demo = null;
				this.playing = false;
				this.publishMediaSession(false);
			}
			return this.playing;
		}
		return false;
	}
	pause() {
		if (this.mode === "file" && this.mediaEl) this.mediaEl.pause();
		this.stopDemo();
		this.playing = false;
		this.publishMediaSession(false);
	}
	seek(seconds) {
		if (this.mode === "file" && this.mediaEl) this.mediaEl.currentTime = Math.max(0, seconds);
	}
	currentTime() {
		if (this.mode === "file" && this.mediaEl) return this.mediaEl.currentTime;
		if (this.mode === "demo" && this.ctx) return this.ctx.currentTime;
		return 0;
	}
	duration() {
		if (this.mode === "file" && this.mediaEl && Number.isFinite(this.mediaEl.duration)) return this.mediaEl.duration;
		return 0;
	}
	stop() {
		this.pause();
		this.mode = "idle";
		if (this.mediaEl) {
			this.mediaEl.removeAttribute("src");
			this.mediaEl.load();
		}
	}
	stopDemo() {
		this.demo?.stop();
		this.demo = null;
	}
	startTimeClock() {
		if (this.timeTimer != null) window.clearInterval(this.timeTimer);
		this.timeTimer = window.setInterval(() => {
			if (!this.playing) return;
			if (this.mode === "file" && this.mediaEl) this.onTime?.(this.mediaEl.currentTime, this.mediaEl.duration || 0);
			else if (this.mode === "demo") this.onTime?.(this.currentTime() % 3600, 0);
		}, 250);
	}
	sample() {
		const now = typeof performance !== "undefined" ? performance.now() / 1e3 : 0;
		this.simPhase = now;
		const beatLen = 60 / this.bpm;
		if (this.analyser && this.playing) {
			this.analyser.getByteFrequencyData(this.freq);
			this.analyser.getByteTimeDomainData(this.time);
		} else fillSimulated(this.freq, this.time, now, this.bpm, this.playing);
		const bass = bandMean(this.freq, 0, 6);
		const mid = bandMean(this.freq, 8, 28);
		const treble = bandMean(this.freq, 40, 90);
		let rms = 0;
		for (let i = 0; i < this.time.length; i++) {
			const v = (this.time[i] - 128) / 128;
			rms += v * v;
		}
		rms = Math.sqrt(rms / this.time.length);
		const energy = clamp01(bass * .55 + mid * .3 + rms * .8);
		const flux = Math.max(0, bass - this.prevBass);
		this.prevBass = bass * .65 + this.prevBass * .35;
		const minGap = beatLen * .46;
		let beat = false;
		if (this.playing && flux > .12 && bass > .34 && now - this.lastBeatAt > minGap) {
			beat = true;
			this.lastBeatAt = now;
			this.beatPulse = 1;
		} else {
			const kickPhase = now % beatLen / beatLen;
			if (this.playing && kickPhase < .06 && now - this.lastBeatAt > minGap) {
				beat = true;
				this.lastBeatAt = now;
				this.beatPulse = Math.max(this.beatPulse, .85);
			}
		}
		this.beatPulse *= .86;
		return {
			freq: this.freq,
			time: this.time,
			bass,
			mid,
			treble,
			rms,
			energy,
			beat,
			beatPulse: this.beatPulse,
			timeSec: now
		};
	}
	publishMediaSession(playing) {
		if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
		try {
			navigator.mediaSession.playbackState = playing ? "playing" : "paused";
		} catch {}
	}
};
function bandMean(data, from, to) {
	const end = Math.min(data.length - 1, to);
	const start = Math.min(from, end);
	let s = 0;
	let n = 0;
	for (let i = start; i <= end; i++) {
		s += data[i];
		n++;
	}
	return n ? s / n / 255 : 0;
}
function fillSimulated(freq, time, t, bpm, playing) {
	const beatLen = 60 / bpm;
	const phase = t % beatLen / beatLen;
	const kick = playing ? Math.exp(-Math.pow(phase * 7.2, 2)) : .12;
	const hat = playing ? Math.pow(Math.max(0, Math.sin(t * Math.PI * (bpm / 60) * 4)), 8) * .45 : .05;
	const bass = playing ? .35 + .35 * Math.sin(t * Math.PI * (bpm / 60) * .5) + kick * .5 : .12;
	for (let i = 0; i < freq.length; i++) {
		const bin = i / freq.length;
		const envelope = Math.exp(-bin * 5.4) * bass + Math.exp(-Math.pow(bin - .18, 2) * 40) * .35 + Math.exp(-Math.pow(bin - .55, 2) * 28) * hat;
		const wobble = .5 + .5 * Math.sin(t * 2.2 + i * .17);
		freq[i] = Math.max(0, Math.min(255, envelope * wobble * 255 * (playing ? 1 : .35)));
	}
	for (let i = 0; i < time.length; i++) {
		const x = i / time.length;
		const wave = Math.sin(x * Math.PI * 8 + t * 6) * (.18 + kick * .45) + Math.sin(x * Math.PI * 20 + t * 3.2) * .08;
		time[i] = Math.max(0, Math.min(255, 128 + wave * 120 * (playing ? 1 : .25)));
	}
}
function startDemoSynth(ctx, dest, synth) {
	const stepSec = 60 / synth.bpm / 4;
	let step = 0;
	let next = ctx.currentTime + .06;
	let stopped = false;
	let timer = 0;
	const filter = ctx.createBiquadFilter();
	filter.type = "lowpass";
	filter.frequency.value = 1400 + synth.brightness * 2200;
	filter.Q.value = .7;
	filter.connect(dest);
	const padGain = ctx.createGain();
	padGain.gain.value = .045 + synth.drive * .02;
	padGain.connect(filter);
	const roots = synth.minor ? [
		1,
		1,
		6 / 5,
		3 / 2,
		1,
		2 / 3,
		3 / 2,
		1
	] : [
		1,
		1,
		5 / 4,
		3 / 2,
		1,
		2 / 3,
		3 / 2,
		1
	];
	const padOsc = [];
	const ratios = synth.minor ? [
		1,
		6 / 5,
		3 / 2
	] : [
		1,
		5 / 4,
		3 / 2
	];
	for (const r of ratios) {
		const o = ctx.createOscillator();
		o.type = "sine";
		o.frequency.value = synth.rootHz * r * .5;
		o.connect(padGain);
		o.start();
		padOsc.push(o);
	}
	const kick = (time, accent) => {
		const o = ctx.createOscillator();
		const g = ctx.createGain();
		o.type = "sine";
		o.frequency.setValueAtTime(150 + synth.drive * 20, time);
		o.frequency.exponentialRampToValueAtTime(38, time + .12);
		g.gain.setValueAtTime(1e-4, time);
		g.gain.exponentialRampToValueAtTime(.85 * accent * synth.drive, time + .008);
		g.gain.exponentialRampToValueAtTime(1e-4, time + .22);
		o.connect(g);
		g.connect(dest);
		o.start(time);
		o.stop(time + .25);
	};
	const hat = (time, open, gain) => {
		const buffer = ctx.createBuffer(1, Math.floor(ctx.sampleRate * .08), ctx.sampleRate);
		const data = buffer.getChannelData(0);
		for (let i = 0; i < data.length; i++) data[i] = Math.random() * 2 - 1;
		const src = ctx.createBufferSource();
		src.buffer = buffer;
		const bp = ctx.createBiquadFilter();
		bp.type = "highpass";
		bp.frequency.value = open ? 6e3 : 8e3;
		const g = ctx.createGain();
		g.gain.setValueAtTime(gain, time);
		g.gain.exponentialRampToValueAtTime(1e-4, time + (open ? .12 : .04));
		src.connect(bp);
		bp.connect(g);
		g.connect(dest);
		src.start(time);
		src.stop(time + .14);
	};
	const bass = (time, freq, len) => {
		const o = ctx.createOscillator();
		const g = ctx.createGain();
		const f = ctx.createBiquadFilter();
		o.type = "triangle";
		o.frequency.setValueAtTime(freq, time);
		f.type = "lowpass";
		f.frequency.setValueAtTime(380 + synth.brightness * 240, time);
		g.gain.setValueAtTime(1e-4, time);
		g.gain.exponentialRampToValueAtTime(.22 * synth.drive, time + .02);
		g.gain.exponentialRampToValueAtTime(1e-4, time + len);
		o.connect(f);
		f.connect(g);
		g.connect(dest);
		o.start(time);
		o.stop(time + len + .02);
	};
	const blip = (time, freq) => {
		const o = ctx.createOscillator();
		const g = ctx.createGain();
		o.type = "sine";
		o.frequency.setValueAtTime(freq, time);
		g.gain.setValueAtTime(1e-4, time);
		g.gain.exponentialRampToValueAtTime(.05 * synth.brightness, time + .01);
		g.gain.exponentialRampToValueAtTime(1e-4, time + .16);
		o.connect(g);
		g.connect(filter);
		o.start(time);
		o.stop(time + .18);
	};
	const schedule = () => {
		if (stopped) return;
		const horizon = ctx.currentTime + .12;
		while (next < horizon) {
			const s = step % 16;
			const bar = Math.floor(step / 4) % 8;
			const accent = s === 0 || s === 8 ? 1 : s % 4 === 0 ? .78 : .55;
			if (s % 4 === 0) kick(next, accent);
			if (s === 4 || s === 12) hat(next, true, .045 * synth.hatDensity);
			if (synth.hatDensity > .3 && s % 2 === 0) hat(next, false, (s % 4 === 2 ? .04 : .022) * synth.hatDensity);
			if (s === 0 || s === 6 || s === 10 || s === 14) {
				const r = roots[bar] ?? 1;
				bass(next, synth.rootHz * .5 * r, stepSec * 2.2);
			}
			if (s % 4 === 2 && synth.brightness > .4) {
				const arp = [
					1,
					3 / 2,
					2,
					9 / 4
				][bar % 4] ?? 1;
				blip(next, synth.rootHz * arp);
			}
			next += stepSec;
			step += 1;
		}
		timer = window.setTimeout(schedule, 25);
	};
	schedule();
	return { stop() {
		stopped = true;
		window.clearTimeout(timer);
		for (const o of padOsc) try {
			o.stop();
			o.disconnect();
		} catch {}
		try {
			padGain.disconnect();
			filter.disconnect();
		} catch {}
	} };
}
var engine = new VibeAudioEngine();
var DB_NAME = "vibe-audio-player";
var STORE = "tracks";
var VERSION = 1;
function openDb() {
	return new Promise((resolve, reject) => {
		const req = indexedDB.open(DB_NAME, VERSION);
		req.onupgradeneeded = () => {
			const db = req.result;
			if (!db.objectStoreNames.contains(STORE)) db.createObjectStore(STORE, { keyPath: "id" });
		};
		req.onsuccess = () => resolve(req.result);
		req.onerror = () => reject(req.error);
	});
}
async function listStoredTracks() {
	try {
		const db = await openDb();
		return await new Promise((resolve, reject) => {
			const req = db.transaction(STORE, "readonly").objectStore(STORE).getAll();
			req.onsuccess = () => resolve(req.result ?? []);
			req.onerror = () => reject(req.error);
		});
	} catch {
		return [];
	}
}
async function putStoredTrack(track) {
	const db = await openDb();
	await new Promise((resolve, reject) => {
		const tx = db.transaction(STORE, "readwrite");
		tx.oncomplete = () => resolve();
		tx.onerror = () => reject(tx.error);
		tx.objectStore(STORE).put(track);
	});
}
async function deleteStoredTrack(id) {
	const db = await openDb();
	await new Promise((resolve, reject) => {
		const tx = db.transaction(STORE, "readwrite");
		tx.oncomplete = () => resolve();
		tx.onerror = () => reject(tx.error);
		tx.objectStore(STORE).delete(id);
	});
}
var VASP_VERSION = "3.69";
var PILLAR_KEYS = [
	"STRUCTURAL",
	"TONAL",
	"TIMBRAL",
	"LINGUISTIC",
	"AFFECTIVE",
	"CONTEXTUAL",
	"PHOTOMETRIC",
	"KINETIC",
	"GENEALOGICAL"
];
var PILLAR_META = {
	STRUCTURAL: {
		key: "STRUCTURAL",
		label: "Structural",
		archetype: "The Skeleton",
		purpose: "Tempo, time signature, rhythmic behavior, arrangement, and percussive DNA"
	},
	TONAL: {
		key: "TONAL",
		label: "Tonal",
		archetype: "The Flesh",
		purpose: "Key, harmony, melody, dissonance, pitch, and tuning"
	},
	TIMBRAL: {
		key: "TIMBRAL",
		label: "Timbral",
		archetype: "The Skin",
		purpose: "Spectral balance, fidelity, spatial character, production aesthetic, and texture"
	},
	LINGUISTIC: {
		key: "LINGUISTIC",
		label: "Linguistic",
		archetype: "The Voice",
		purpose: "Lyrics, semantic content, vocal style, language, and explicit-content classification"
	},
	AFFECTIVE: {
		key: "AFFECTIVE",
		label: "Affective",
		archetype: "The Heart",
		purpose: "Valence, arousal, dominance, emotional complexity, and tension movement"
	},
	CONTEXTUAL: {
		key: "CONTEXTUAL",
		label: "Contextual",
		archetype: "The Scene",
		purpose: "Scenario, setting, activity, intent, time of day, weather, and environmental match"
	},
	PHOTOMETRIC: {
		key: "PHOTOMETRIC",
		label: "Photometric",
		archetype: "The Eye",
		purpose: "Color palette, visual texture, brightness, lighting behavior, and synchronization output"
	},
	KINETIC: {
		key: "KINETIC",
		label: "Kinetic",
		archetype: "The Body",
		purpose: "Entrainment, movement response, energy expenditure, and physical activity metadata"
	},
	GENEALOGICAL: {
		key: "GENEALOGICAL",
		label: "Genealogical",
		archetype: "The Roots",
		purpose: "Era, sampling lineage, genre tree, cultural context, and tribe alignment"
	}
};
function known(value) {
	return {
		value,
		status: "known"
	};
}
function pending(value) {
	return {
		value,
		status: "pending"
	};
}
function unknown(value) {
	return {
		value,
		status: "unknown"
	};
}
function profile(title, artist, pillars) {
	return {
		VAP_VERSION: VASP_VERSION,
		IDENTITY: {
			TITLE: title,
			ARTIST: artist
		},
		PILLARS: pillars
	};
}
var NIGHT_DRIVE = {
	id: "demo-night-drive",
	kind: "demo",
	title: "Night Drive Protocol",
	artist: "Aurphyx Demo",
	duration: null,
	synth: {
		bpm: 128,
		rootHz: 110,
		minor: true,
		drive: .86,
		brightness: .62,
		hatDensity: 1
	},
	vasp: profile("Night Drive Protocol", "Aurphyx Demo", {
		STRUCTURAL: {
			bpm: known(128),
			timeSignature: known("4/4"),
			groove: known("Machine-lock groove"),
			kickPulse: known("Strong kick pulse"),
			arrangement: known("Two-bar loop, driving eighths")
		},
		TONAL: {
			key: known("A Minor"),
			mode: known("Natural minor"),
			dissonance: known("Moderate dissonance"),
			contour: known("Dark melodic contour"),
			tuning: known("A440 equal temperament")
		},
		TIMBRAL: {
			spectral: known("Bright and airy"),
			fidelity: known("Hi-fi"),
			stereo: known("Wide stereo image"),
			texture: known("Glassy electronic texture"),
			production: known("Night-drive synth, tight low end")
		},
		LINGUISTIC: {
			lyrics: known("Instrumental"),
			language: known("None"),
			vocalStyle: known("No lyrics"),
			contentTier: known("Clean content tier")
		},
		AFFECTIVE: {
			valence: known("Low-to-neutral valence"),
			arousal: known("High arousal"),
			dominance: known("Focused control"),
			mood: known("Focused, nocturnal, energetic"),
			tension: known("Forward-drive tension")
		},
		CONTEXTUAL: {
			scenario: known("Night drive"),
			setting: known("City lights / rain atmosphere"),
			activity: known("Solo listening"),
			timeOfDay: known("Late night"),
			atmosphere: known("Wet asphalt, sodium-to-neon")
		},
		PHOTOMETRIC: {
			primaryHex: known("#4B0082"),
			secondaryHex: known("#008080"),
			temperature: known("cool"),
			brightness: known(.62),
			fade: known("Smooth visual fades"),
			lightBehavior: known("Indigo and teal light behavior")
		},
		KINETIC: {
			movementEnergy: known("High movement energy"),
			entrainment: known("Strong beat entrainment"),
			metScore: known(6),
			response: known("Head-nod and forward-drive response")
		},
		GENEALOGICAL: {
			genre: known("Electronic"),
			lineage: known("Synthwave-inspired"),
			era: known("Contemporary"),
			tribe: known("Digital nocturnal"),
			aesthetic: known("Digital nocturnal aesthetic")
		}
	})
};
var DEMO_TRACKS = [
	NIGHT_DRIVE,
	{
		id: "demo-orbital-lattice",
		kind: "demo",
		title: "Orbital Lattice",
		artist: "Aurphyx Demo",
		duration: null,
		synth: {
			bpm: 96,
			rootHz: 146.83,
			minor: true,
			drive: .55,
			brightness: .42,
			hatDensity: .5
		},
		vasp: profile("Orbital Lattice", "Aurphyx Demo", {
			STRUCTURAL: {
				bpm: known(96),
				timeSignature: known("4/4"),
				groove: known("Slow-orbit pulse"),
				kickPulse: known("Rounded kick, wide decay"),
				arrangement: known("Long pads over sparse percussion")
			},
			TONAL: {
				key: known("D Minor"),
				mode: known("Dorian color"),
				dissonance: known("Low dissonance"),
				contour: known("Ascending lattice intervals"),
				tuning: known("A440 equal temperament")
			},
			TIMBRAL: {
				spectral: known("Dark, sub-heavy"),
				fidelity: known("Hi-fi, filtered"),
				stereo: known("Deep stereo field"),
				texture: known("Velvet drone + glass chime"),
				production: known("Orbital pad, distant bells")
			},
			LINGUISTIC: {
				lyrics: known("Instrumental"),
				language: known("None"),
				vocalStyle: known("No lyrics"),
				contentTier: known("Clean content tier")
			},
			AFFECTIVE: {
				valence: known("Neutral-positive valence"),
				arousal: known("Mid-low arousal"),
				dominance: known("Suspended"),
				mood: known("Contemplative, weightless"),
				tension: known("Slow harmonic bloom")
			},
			CONTEXTUAL: {
				scenario: known("Deep space drift"),
				setting: known("Interior of a dark observatory"),
				activity: known("Solo listening"),
				timeOfDay: known("After midnight"),
				atmosphere: known("Vacuum hush, distant stars")
			},
			PHOTOMETRIC: {
				primaryHex: known("#1B1464"),
				secondaryHex: known("#5B8DEF"),
				temperature: known("cool"),
				brightness: known(.44),
				fade: known("Long crossfades"),
				lightBehavior: known("Deep indigo with ice-blue rims")
			},
			KINETIC: {
				movementEnergy: known("Low-mid movement energy"),
				entrainment: known("Breath-paced pulse"),
				metScore: known(3),
				response: known("Stillness with slow sway")
			},
			GENEALOGICAL: {
				genre: known("Ambient electronic"),
				lineage: known("Orbital / IDM adjacent"),
				era: known("Contemporary"),
				tribe: known("Lattice listeners"),
				aesthetic: known("Fractal quiet")
			}
		})
	},
	{
		id: "demo-forward-current",
		kind: "demo",
		title: "Forward Current",
		artist: "Aurphyx Demo",
		duration: null,
		synth: {
			bpm: 140,
			rootHz: 185,
			minor: true,
			drive: .92,
			brightness: .78,
			hatDensity: 1
		},
		vasp: profile("Forward Current", "Aurphyx Demo", {
			STRUCTURAL: {
				bpm: known(140),
				timeSignature: known("4/4"),
				groove: known("High-energy lock"),
				kickPulse: known("Hard four-on-the-floor"),
				arrangement: known("Peak-time loop, rolling bass")
			},
			TONAL: {
				key: known("F# Minor"),
				mode: known("Natural minor"),
				dissonance: known("Bright tension"),
				contour: known("Rising current"),
				tuning: known("A440 equal temperament")
			},
			TIMBRAL: {
				spectral: known("Crisp highs, present mids"),
				fidelity: known("Hi-fi, slightly overdriven"),
				stereo: known("Wide, kinetic"),
				texture: known("Electric current, metallic hats"),
				production: known("Club-leaning electronic")
			},
			LINGUISTIC: {
				lyrics: known("Instrumental"),
				language: known("None"),
				vocalStyle: known("No lyrics"),
				contentTier: known("Clean content tier")
			},
			AFFECTIVE: {
				valence: known("Neutral-high valence"),
				arousal: known("Very high arousal"),
				dominance: known("Assertive"),
				mood: known("Urgent, kinetic, lucid"),
				tension: known("Relentless forward pressure")
			},
			CONTEXTUAL: {
				scenario: known("Rain circuit"),
				setting: known("Elevated freeway in weather"),
				activity: known("Motion listening"),
				timeOfDay: known("Blue hour into night"),
				atmosphere: known("Wet glass, sodium flares")
			},
			PHOTOMETRIC: {
				primaryHex: known("#0D7377"),
				secondaryHex: known("#7C6CFF"),
				temperature: known("cool"),
				brightness: known(.74),
				fade: known("Fast visual cuts"),
				lightBehavior: known("Teal core, violet highlights")
			},
			KINETIC: {
				movementEnergy: known("Very high movement energy"),
				entrainment: known("Body-lock beat"),
				metScore: known(7),
				response: known("Stride and pulse")
			},
			GENEALOGICAL: {
				genre: known("Electronic"),
				lineage: known("Techno-adjacent synthwave"),
				era: known("Contemporary"),
				tribe: known("Night circuit"),
				aesthetic: known("Rain-slick current")
			}
		})
	}
];
function hslToHex(h, s, l) {
	const sat = s / 100;
	const lig = l / 100;
	const k = (n) => (n + h / 30) % 12;
	const a = sat * Math.min(lig, 1 - lig);
	const f = (n) => lig - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
	return rgbToHex(255 * f(0), 255 * f(8), 255 * f(4));
}
function makeFileVasp(title, artist = "Local file") {
	const hue = hashHue(title);
	const primary = hslToHex((hue + 260) % 360, 72, 28);
	const secondary = hslToHex((hue + 175) % 360, 64, 38);
	return profile(title, artist, {
		STRUCTURAL: {
			bpm: pending(null),
			timeSignature: unknown("4/4"),
			groove: pending("Pending analysis"),
			kickPulse: pending("Pending analysis"),
			arrangement: unknown("Unknown arrangement")
		},
		TONAL: {
			key: pending("Pending"),
			mode: unknown("Unknown"),
			dissonance: pending("Pending"),
			contour: unknown("Unknown"),
			tuning: known("Source file")
		},
		TIMBRAL: {
			spectral: pending("Pending"),
			fidelity: known("Source file"),
			stereo: unknown("Unknown"),
			texture: pending("Pending"),
			production: known("Local playback")
		},
		LINGUISTIC: {
			lyrics: unknown("Unknown"),
			language: unknown("Unknown"),
			vocalStyle: unknown("Unknown"),
			contentTier: unknown("Unknown")
		},
		AFFECTIVE: {
			valence: pending("Pending"),
			arousal: pending("Pending"),
			dominance: unknown("Unknown"),
			mood: pending("Listening"),
			tension: pending("Pending")
		},
		CONTEXTUAL: {
			scenario: known("Local playback"),
			setting: known("On device"),
			activity: known("Solo listening"),
			timeOfDay: unknown("Unknown"),
			atmosphere: known("Player session")
		},
		PHOTOMETRIC: {
			primaryHex: known(primary),
			secondaryHex: known(secondary),
			temperature: known("cool"),
			brightness: known(.6),
			fade: known("Smooth visual fades"),
			lightBehavior: known("File-derived palette")
		},
		KINETIC: {
			movementEnergy: pending("Pending"),
			entrainment: pending("Pending"),
			metScore: pending(null),
			response: unknown("Unknown")
		},
		GENEALOGICAL: {
			genre: unknown("Unknown"),
			lineage: known("Local library"),
			era: unknown("Unknown"),
			tribe: known("Personal collection"),
			aesthetic: known("Source material")
		}
	});
}
function trackFromFile(file) {
	const title = file.name.replace(/\.[^.]+$/, "").replace(/[_-]+/g, " ").trim() || "Untitled";
	return {
		id: uid("file"),
		kind: "file",
		title,
		artist: "Local file",
		duration: null,
		objectUrl: URL.createObjectURL(file),
		mime: file.type || "audio/*",
		size: file.size,
		vasp: makeFileVasp(title)
	};
}
function mappingFromProfile(profile) {
	const p = profile.PILLARS;
	const bpm = p.STRUCTURAL.bpm.value ?? 120;
	const kick = (p.STRUCTURAL.kickPulse.value ?? "").toLowerCase();
	const pulseStrength = kick.includes("strong") || kick.includes("hard") ? 1 : kick.includes("round") ? .55 : .75;
	const key = (p.TONAL.key.value ?? "").toLowerCase();
	const geometrySides = key.includes("minor") ? 6 : key.includes("major") ? 4 : 5;
	const dissonance = (p.TONAL.dissonance.value ?? "").toLowerCase();
	const harmonicShift = dissonance.includes("high") || dissonance.includes("bright") ? .18 : dissonance.includes("moderate") ? .1 : .04;
	const spectral = (p.TIMBRAL.spectral.value ?? "").toLowerCase();
	const particleDensity = spectral.includes("bright") || spectral.includes("crisp") ? 1 : spectral.includes("dark") ? .55 : .8;
	const texture = (p.TIMBRAL.texture.value ?? "").toLowerCase();
	const blur = texture.includes("velvet") || texture.includes("drone") ? .55 : texture.includes("glassy") ? .22 : .3;
	const grain = texture.includes("metallic") || texture.includes("over") ? .18 : .08;
	const arousal = (p.AFFECTIVE.arousal.value ?? "").toLowerCase();
	const movementEnergy = arousal.includes("very high") ? 1 : arousal.includes("high") ? .82 : arousal.includes("mid") ? .45 : .32;
	const contrast = arousal.includes("high") ? .85 : .55;
	const scene = (p.CONTEXTUAL.scenario.value ?? "Night drive").toString();
	const atmosphere = (p.CONTEXTUAL.atmosphere.value ?? "").toLowerCase();
	const rain = atmosphere.includes("rain") || atmosphere.includes("wet") || scene.toLowerCase().includes("rain") || scene.toLowerCase().includes("drive");
	const met = p.KINETIC.metScore.value ?? 5;
	const impactScale = Math.min(1.2, .45 + met / 10);
	const accentWarmth = (p.GENEALOGICAL.lineage.value ?? "").toLowerCase().includes("synthwave") ? .22 : .08;
	const temp = p.PHOTOMETRIC.temperature.value ?? "cool";
	return {
		primary: p.PHOTOMETRIC.primaryHex.value ?? "#4B0082",
		secondary: p.PHOTOMETRIC.secondaryHex.value ?? "#008080",
		temperature: temp,
		brightness: p.PHOTOMETRIC.brightness.value ?? .6,
		fade: (p.PHOTOMETRIC.fade.value ?? "").toLowerCase().includes("fast") ? .35 : .18,
		bpm,
		pulseStrength,
		geometrySides,
		harmonicShift,
		particleDensity,
		blur,
		grain,
		edgeSharpness: texture.includes("glassy") || spectral.includes("crisp") ? .85 : .5,
		contrast,
		movementEnergy,
		scene,
		rain,
		impactScale,
		accentWarmth
	};
}
function flattenPillar(profile, key) {
	const pillar = profile.PILLARS[key];
	const labels = {
		bpm: "BPM",
		timeSignature: "Time signature",
		groove: "Groove",
		kickPulse: "Kick pulse",
		arrangement: "Arrangement",
		key: "Key",
		mode: "Mode",
		dissonance: "Dissonance",
		contour: "Contour",
		tuning: "Tuning",
		spectral: "Spectral balance",
		fidelity: "Fidelity",
		stereo: "Stereo image",
		texture: "Texture",
		production: "Production",
		lyrics: "Lyrics",
		language: "Language",
		vocalStyle: "Vocal style",
		contentTier: "Content tier",
		valence: "Valence",
		arousal: "Arousal",
		dominance: "Dominance",
		mood: "Mood",
		tension: "Tension",
		scenario: "Scenario",
		setting: "Setting",
		activity: "Activity",
		timeOfDay: "Time of day",
		atmosphere: "Atmosphere",
		primaryHex: "Primary hex",
		secondaryHex: "Secondary hex",
		temperature: "Palette temperature",
		brightness: "Brightness",
		fade: "Fades",
		lightBehavior: "Light behavior",
		movementEnergy: "Movement energy",
		entrainment: "Entrainment",
		metScore: "MET score",
		response: "Body response",
		genre: "Genre",
		lineage: "Lineage",
		era: "Era",
		tribe: "Tribe",
		aesthetic: "Aesthetic"
	};
	return Object.entries(pillar).map(([k, field]) => ({
		label: labels[k] ?? k,
		value: field.value == null ? "—" : String(field.value),
		status: field.status
	}));
}
var DEFAULT_SETTINGS = {
	colorIntensity: .82,
	motionIntensity: .78,
	particles: true,
	spectrum: true,
	beatPulse: true,
	reducedMotion: false,
	readableType: false
};
function orderIds(library, shuffle, currentId) {
	const ids = library.map((t) => t.id);
	if (!shuffle) return ids;
	const rest = ids.filter((id) => id !== currentId);
	for (let i = rest.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		const a = rest[i];
		rest[i] = rest[j];
		rest[j] = a;
	}
	return [currentId, ...rest];
}
function publishMetadata(track) {
	if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
	try {
		navigator.mediaSession.metadata = new MediaMetadata({
			title: track.title,
			artist: track.artist,
			album: "Vibe Audio Player"
		});
	} catch {}
}
var usePlayer = create()(persist((set, get) => ({
	hydrated: false,
	library: DEMO_TRACKS,
	currentId: NIGHT_DRIVE.id,
	playing: false,
	currentTime: 0,
	duration: 0,
	volume: .72,
	shuffle: false,
	repeat: "all",
	sheet: "none",
	activePillar: "PHOTOMETRIC",
	settings: DEFAULT_SETTINGS,
	mapping: mappingFromProfile(NIGHT_DRIVE.vasp),
	dropActive: false,
	notice: null,
	current: () => get().library.find((t) => t.id === get().currentId) ?? get().library[0] ?? NIGHT_DRIVE,
	async hydrate() {
		if (get().hydrated) return;
		engine.setVolume(get().volume);
		engine.setEndedHandler(() => get().onEngineEnded());
		engine.setTimeHandler((t, d) => get().onEngineTime(t, d));
		try {
			const stored = await listStoredTracks();
			if (stored.length) {
				const restored = stored.map((row) => ({
					id: row.id,
					kind: "file",
					title: row.title,
					artist: row.artist,
					duration: null,
					objectUrl: URL.createObjectURL(row.blob),
					mime: row.mime,
					size: row.size,
					vasp: makeFileVasp(row.title, row.artist)
				}));
				set({ library: [...DEMO_TRACKS, ...restored] });
			}
		} catch {}
		const track = get().current();
		set({
			hydrated: true,
			mapping: mappingFromProfile(track.vasp)
		});
		engine.setBpm(track.vasp.PILLARS.STRUCTURAL.bpm.value ?? 120);
		if (typeof navigator !== "undefined" && "mediaSession" in navigator) try {
			navigator.mediaSession.setActionHandler("play", () => void get().togglePlay());
			navigator.mediaSession.setActionHandler("pause", () => void get().togglePlay());
			navigator.mediaSession.setActionHandler("nexttrack", () => void get().next());
			navigator.mediaSession.setActionHandler("previoustrack", () => void get().prev());
		} catch {}
	},
	async playTrack(id) {
		const track = get().library.find((t) => t.id === id);
		if (!track) return;
		engine.setBpm(track.vasp.PILLARS.STRUCTURAL.bpm.value ?? 120);
		set({
			currentId: id,
			mapping: mappingFromProfile(track.vasp),
			currentTime: 0,
			duration: track.duration ?? 0
		});
		publishMetadata(track);
		try {
			if (track.kind === "demo" && track.synth) await engine.playDemo(track.synth);
			else if (track.objectUrl) {
				await engine.playFile(track.objectUrl);
				set({ duration: engine.duration() || track.duration || 0 });
			}
			set({
				playing: true,
				notice: null
			});
		} catch (err) {
			set({
				playing: false,
				notice: err instanceof Error ? err.message : "Could not play this track"
			});
		}
	},
	async togglePlay() {
		const { playing, currentId, current } = get();
		const track = current();
		if (!playing) {
			await get().playTrack(currentId);
			return;
		}
		if (track.kind === "file") {
			set({ playing: await engine.togglePause() });
			return;
		}
		engine.pause();
		set({ playing: false });
	},
	async next() {
		const { library, currentId, shuffle, repeat, playing } = get();
		if (repeat === "one" && playing) {
			await get().playTrack(currentId);
			return;
		}
		const ids = orderIds(library, shuffle, currentId);
		const i = ids.indexOf(currentId);
		const nextId = ids[(i + 1) % ids.length];
		if (!nextId) return;
		if (repeat === "off" && i === ids.length - 1) {
			engine.pause();
			set({
				playing: false,
				currentTime: 0
			});
			return;
		}
		await get().playTrack(nextId);
	},
	async prev() {
		const { library, currentId, shuffle, currentTime } = get();
		if (currentTime > 3) {
			engine.seek(0);
			set({ currentTime: 0 });
			return;
		}
		const ids = orderIds(library, shuffle, currentId);
		const prevId = ids[(ids.indexOf(currentId) - 1 + ids.length) % ids.length];
		if (prevId) await get().playTrack(prevId);
	},
	seek(t) {
		engine.seek(t);
		set({ currentTime: t });
	},
	setVolume(v) {
		engine.setVolume(v);
		set({ volume: v });
	},
	setSheet(s) {
		set({ sheet: s });
	},
	setPillar(p) {
		set({
			activePillar: p,
			sheet: "vasp"
		});
	},
	setSetting(key, value) {
		set({ settings: {
			...get().settings,
			[key]: value
		} });
	},
	resetSettings() {
		const demo = NIGHT_DRIVE;
		set({
			settings: DEFAULT_SETTINGS,
			currentId: demo.id,
			mapping: mappingFromProfile(demo.vasp)
		});
	},
	async importFiles(files) {
		const list = Array.from(files).filter((f) => f.type.startsWith("audio/") || /\.(mp3|wav|flac|ogg|m4a|aac|opus|webm)$/i.test(f.name));
		if (!list.length) {
			set({ notice: "No audio files found in that selection." });
			return;
		}
		const added = [];
		for (const file of list) {
			const track = trackFromFile(file);
			added.push(track);
			try {
				await putStoredTrack({
					id: track.id,
					title: track.title,
					artist: track.artist,
					mime: track.mime ?? file.type,
					size: file.size,
					addedAt: Date.now(),
					blob: file
				});
			} catch {}
		}
		set({
			library: [...get().library, ...added],
			sheet: "library",
			notice: `Added ${added.length} track${added.length === 1 ? "" : "s"} to your library.`
		});
		if (!get().playing) await get().playTrack(added[0].id);
	},
	async removeTrack(id) {
		const track = get().library.find((t) => t.id === id);
		if (!track || track.kind === "demo") return;
		if (track.objectUrl) URL.revokeObjectURL(track.objectUrl);
		try {
			await deleteStoredTrack(id);
		} catch {}
		const library = get().library.filter((t) => t.id !== id);
		const nextCurrent = get().currentId === id ? library[0]?.id ?? NIGHT_DRIVE.id : get().currentId;
		const wasCurrent = get().currentId === id;
		set({ library });
		if (wasCurrent) {
			engine.pause();
			set({
				playing: false,
				currentId: nextCurrent
			});
			set({ mapping: mappingFromProfile((library.find((t) => t.id === nextCurrent) ?? NIGHT_DRIVE).vasp) });
		}
	},
	setShuffle(v) {
		set({ shuffle: v });
	},
	setRepeat(v) {
		set({ repeat: v });
	},
	setDropActive(v) {
		set({ dropActive: v });
	},
	setNotice(v) {
		set({ notice: v });
	},
	onEngineEnded() {
		get().next();
	},
	onEngineTime(t, d) {
		set({
			currentTime: t,
			duration: d || get().duration
		});
	}
}), {
	name: "vibe-player-settings",
	partialize: (s) => ({
		volume: s.volume,
		settings: s.settings,
		shuffle: s.shuffle,
		repeat: s.repeat
	})
}));
function LibrarySheet() {
	const sheet = usePlayer((s) => s.sheet);
	const setSheet = usePlayer((s) => s.setSheet);
	const library = usePlayer((s) => s.library);
	const currentId = usePlayer((s) => s.currentId);
	const playTrack = usePlayer((s) => s.playTrack);
	const importFiles = usePlayer((s) => s.importFiles);
	const removeTrack = usePlayer((s) => s.removeTrack);
	const inputRef = (0, import_react.useRef)(null);
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(BottomSheet, {
		open: sheet === "library",
		onOpenChange: (o) => setSheet(o ? "library" : "none"),
		eyebrow: "Local library",
		title: "Vibe Audio Player",
		children: [
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("input", {
				ref: inputRef,
				type: "file",
				accept: "audio/*,.mp3,.wav,.flac,.ogg,.m4a,.aac,.opus,.webm",
				multiple: true,
				className: "hidden",
				onChange: (e) => {
					if (e.target.files?.length) importFiles(e.target.files);
					e.target.value = "";
				}
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
				className: "mb-4 max-w-prose text-pretty text-sm leading-relaxed text-muted",
				children: "Play built-in VASP demos or open audio from this device. Files stay on your phone — nothing is uploaded."
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)(Button, {
				type: "button",
				variant: "teal",
				className: "mb-5 w-full",
				onClick: () => inputRef.current?.click(),
				children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)(FolderOpen, {}), "Open local audio"]
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("ul", {
				className: "flex flex-col gap-2",
				children: library.map((track) => {
					const active = track.id === currentId;
					return /* @__PURE__ */ (0, import_jsx_runtime.jsx)("li", { children: /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: cn("flex items-center gap-3 rounded-2xl p-2.5 shadow-border", active ? "bg-white/8" : "bg-white/3"),
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("button", {
							type: "button",
							onClick: () => {
								engine.unlock();
								playTrack(track.id);
							},
							className: "flex min-w-0 flex-1 items-center gap-3 text-left",
							children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
								className: cn("grid size-11 shrink-0 place-items-center rounded-xl", active ? "bg-accent/20 text-accent" : "bg-white/6 text-muted"),
								children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Music2, { className: "size-4" })
							}), /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("span", {
								className: "min-w-0",
								children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
									className: "block truncate font-medium text-fg",
									children: track.title
								}), /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("span", {
									className: "block truncate text-xs text-muted",
									children: [
										track.artist,
										track.kind === "demo" ? " · Demo" : "",
										track.duration ? ` · ${formatTime(track.duration)}` : ""
									]
								})]
							})]
						}), track.kind === "file" ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("button", {
							type: "button",
							"aria-label": `Remove ${track.title}`,
							className: "grid size-11 place-items-center rounded-full text-muted hover:bg-white/6 hover:text-fg",
							onClick: () => void removeTrack(track.id),
							children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Trash2, { className: "size-4" })
						}) : null]
					}) }, track.id);
				})
			})
		]
	});
}
var Slider = import_react.forwardRef(({ className, ...props }, ref) => /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(Slider$1, {
	ref,
	className: cn("relative flex w-full touch-none select-none items-center", className),
	...props,
	children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)(SliderTrack, {
		className: "relative h-1.5 w-full grow overflow-hidden rounded-full bg-white/10",
		children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(SliderRange, { className: "absolute h-full bg-accent" })
	}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(SliderThumb, { className: "block size-4 rounded-full bg-fg shadow-glow-teal ring-2 ring-bg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent" })]
}));
Slider.displayName = Slider$1.displayName;
function NowPlayingCard() {
	const track = usePlayer((s) => s.current());
	const playing = usePlayer((s) => s.playing);
	const currentTime = usePlayer((s) => s.currentTime);
	const duration = usePlayer((s) => s.duration);
	const togglePlay = usePlayer((s) => s.togglePlay);
	const next = usePlayer((s) => s.next);
	const prev = usePlayer((s) => s.prev);
	const seek = usePlayer((s) => s.seek);
	const setSheet = usePlayer((s) => s.setSheet);
	const shuffle = usePlayer((s) => s.shuffle);
	const setShuffle = usePlayer((s) => s.setShuffle);
	const repeat = usePlayer((s) => s.repeat);
	const setRepeat = usePlayer((s) => s.setRepeat);
	const mapping = usePlayer((s) => s.mapping);
	const p = track.vasp.PILLARS;
	const bpm = p.STRUCTURAL.bpm.value;
	const key = p.TONAL.key.value;
	const mood = p.AFFECTIVE.mood.value;
	const scene = p.CONTEXTUAL.scenario.value;
	const isDemo = track.kind === "demo";
	const canSeek = !isDemo && duration > 0;
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("section", {
		className: "pointer-events-auto mx-auto w-full max-w-lg rounded-[28px] bg-surface/75 p-3 shadow-sheet backdrop-blur-xl",
		children: [
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
				className: "rounded-2xl bg-white/4 px-3 py-3",
				children: [
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "mb-2 flex items-start justify-between gap-3",
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
							className: "min-w-0",
							children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
								className: "truncate font-display text-lg font-semibold leading-tight tracking-tight text-fg",
								children: track.title
							}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
								className: "truncate text-sm text-muted",
								children: track.artist
							})]
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
							className: "shrink-0 rounded-full bg-accent/15 px-2.5 py-1 text-[10px] font-medium uppercase tracking-[0.14em] text-accent",
							children: scene ?? "Scene"
						})]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "mb-3 flex flex-wrap gap-1.5",
						children: [
							bpm ? /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(Chip, { children: [String(bpm), " BPM"] }) : null,
							key ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Chip, { children: key }) : null,
							mood ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Chip, { children: mood }) : null
						]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "mb-1 flex items-center gap-3",
						children: [
							/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
								className: "w-8 text-[11px] tabular-nums text-muted",
								children: isDemo ? "LIVE" : formatTime(currentTime)
							}),
							canSeek ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Slider, {
								min: 0,
								max: duration,
								step: .1,
								value: [currentTime],
								onValueChange: ([v]) => seek(v ?? 0),
								"aria-label": "Seek"
							}) : /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
								className: "relative h-1.5 w-full overflow-hidden rounded-full bg-white/10",
								children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", { className: cn("absolute inset-y-0 left-0 rounded-full bg-accent", playing ? "w-2/3 opacity-90" : "w-1/4 opacity-50") })
							}),
							/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
								className: "w-8 text-right text-[11px] tabular-nums text-muted",
								children: isDemo ? "∞" : formatTime(duration)
							})
						]
					})
				]
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
				className: "mt-2 flex items-center justify-between px-1",
				children: [
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(IconBtn, {
						label: shuffle ? "Shuffle on" : "Shuffle off",
						onClick: () => setShuffle(!shuffle),
						active: shuffle,
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Shuffle, { className: "size-4" })
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(IconBtn, {
						label: "Previous",
						onClick: () => void prev(),
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(SkipBack, { className: "size-5" })
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Button, {
						type: "button",
						size: "play",
						variant: "teal",
						"aria-label": playing ? "Pause" : "Play",
						"data-testid": "play-toggle",
						onClick: () => {
							engine.unlock();
							togglePlay();
						},
						className: "shadow-glow-teal",
						children: playing ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Pause, { className: "size-7 fill-current" }) : /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Play, { className: "ml-0.5 size-7 fill-current" })
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(IconBtn, {
						label: "Next",
						onClick: () => void next(),
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(SkipForward, { className: "size-5" })
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(IconBtn, {
						label: `Repeat ${repeat}`,
						onClick: () => setRepeat(repeat === "off" ? "all" : repeat === "all" ? "one" : "off"),
						active: repeat !== "off",
						children: repeat === "one" ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Repeat1, { className: "size-4" }) : /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Repeat, { className: "size-4" })
					})
				]
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
				className: "mt-2 grid grid-cols-3 gap-1.5",
				children: [
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(NavBtn, {
						onClick: () => setSheet("library"),
						icon: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Library, { className: "size-4" }),
						label: "Library"
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(NavBtn, {
						onClick: () => setSheet("vasp"),
						icon: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Hexagon, { className: "size-4" }),
						label: "VASP"
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(NavBtn, {
						onClick: () => setSheet("settings"),
						icon: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Settings2, { className: "size-4" }),
						label: "Scene"
					})
				]
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("p", {
				className: "sr-only",
				children: [
					"Active mapping palette ",
					mapping.primary,
					" ",
					mapping.secondary
				]
			})
		]
	});
}
function Chip({ children }) {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
		className: "rounded-full bg-white/6 px-2 py-0.5 text-[11px] font-medium text-fg/90",
		children
	});
}
function IconBtn({ children, label, onClick, active }) {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsx)("button", {
		type: "button",
		"aria-label": label,
		onClick,
		className: cn("grid size-11 place-items-center rounded-full text-muted transition-colors hover:bg-white/6 hover:text-fg", active && "text-accent"),
		children
	});
}
function NavBtn({ onClick, icon, label }) {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("button", {
		type: "button",
		onClick,
		className: "flex h-11 items-center justify-center gap-1.5 rounded-2xl bg-white/4 text-xs font-medium text-fg shadow-border hover:bg-white/8",
		children: [icon, label]
	});
}
/**
* Current user + loading state. Same behavior in live preview and when deployed:
*   - Auth enabled (default) -> the real signed-in user; `user` is `null` while
*                            the session resolves (`isPending: true`) and when
*                            signed out (`isPending: false`). Session comes from
*                            Better Auth `useSession()` → `/api/auth/get-session`
*                            (cookie when deployed; bearer in live preview).
*   - Auth disabled (`VITE_AUTH_ENABLED=false`) -> `DEV_USER`, never pending.
*
* Protect a route by waiting out `isPending` before acting on `user` —
* redirecting on `user: null` alone bounces signed-in visitors to sign-in on
* every hard reload:
*
*   import { RedirectToSignIn } from "@/lib/auth/gates";
*   const { user, isPending } = useCurrentUserState();
*   if (isPending) return null;              // still resolving — don't redirect yet
*   if (!user) return <RedirectToSignIn />;  // definitely signed out
*
* `authEnabled` is a module-level constant fixed at load, so the guarded hook
* call keeps a stable hook order across every render of a given component.
*/
function useCurrentUserState() {
	const { data, isPending } = authClient.useSession();
	const user = data?.user;
	return {
		user: user ? {
			id: user.id,
			displayName: user.name ?? null,
			primaryEmail: user.email ?? null,
			profileImageUrl: user.image ?? null,
			isDevFallback: false
		} : null,
		isPending
	};
}
/**
* Convenience view of `useCurrentUserState().user` for display (e.g.
* `user?.displayName ?? "Guest"`). NOTE: `null` means *loading OR signed out* —
* for redirects/guards use `useCurrentUserState()` and check `isPending`.
*/
function useCurrentUser() {
	return useCurrentUserState().user;
}
/**
* Minimal signed-in identity chip + sign-out. Restyle freely (see the
* `design-ui` skill). Sign-out is only shown when auth is enabled (the
* disabled-auth dev user has nothing to sign out of).
*/
function UserButton() {
	const user = useCurrentUser();
	if (!user) return null;
	const label = user.displayName ?? user.primaryEmail ?? "Account";
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
		className: "flex items-center gap-2",
		children: [
			user.profileImageUrl ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("img", {
				src: user.profileImageUrl,
				alt: "",
				className: "h-8 w-8 rounded-full object-cover"
			}) : /* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
				className: "grid h-8 w-8 place-items-center rounded-full bg-black/10 text-sm font-medium dark:bg-white/20",
				children: label.charAt(0).toUpperCase()
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
				className: "text-sm font-medium",
				children: label
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("button", {
				type: "button",
				onClick: () => void signOut(),
				className: "cursor-pointer text-sm underline-offset-4 opacity-70 hover:underline",
				children: "Sign out"
			})
		]
	});
}
var Switch = import_react.forwardRef(({ className, ...props }, ref) => /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch$1, {
	className: cn("peer inline-flex h-7 w-12 shrink-0 cursor-pointer items-center rounded-full shadow-border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-40 data-[state=checked]:bg-accent data-[state=unchecked]:bg-white/10", className),
	...props,
	ref,
	children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(SwitchThumb, { className: "pointer-events-none block size-5 translate-x-1 rounded-full bg-fg shadow-sm transition-transform data-[state=checked]:translate-x-6 data-[state=unchecked]:translate-x-1" })
}));
Switch.displayName = Switch$1.displayName;
function Row({ label, hint, children }) {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
		className: "flex items-center justify-between gap-4 py-3",
		children: [/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
			className: "min-w-0",
			children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
				className: "text-sm font-medium text-fg",
				children: label
			}), hint ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
				className: "text-pretty text-xs leading-relaxed text-muted",
				children: hint
			}) : null]
		}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
			className: "shrink-0",
			children
		})]
	});
}
function AccountSlot() {
	const { user, isPending } = useCurrentUserState();
	if (isPending) return /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", { className: "h-11 animate-pulse rounded-full bg-white/8" });
	if (user) return /* @__PURE__ */ (0, import_jsx_runtime.jsx)(UserButton, {});
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(import_jsx_runtime.Fragment, { children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
		className: "mb-3 text-pretty text-sm leading-relaxed text-muted",
		children: "Optional. Sign-in is not required to play local audio."
	}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Button, {
		asChild: true,
		variant: "secondary",
		className: "w-full",
		children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Link, {
			to: "/login",
			children: "Sign in"
		})
	})] });
}
function SettingsSheet() {
	const sheet = usePlayer((s) => s.sheet);
	const setSheet = usePlayer((s) => s.setSheet);
	const settings = usePlayer((s) => s.settings);
	const setSetting = usePlayer((s) => s.setSetting);
	const resetSettings = usePlayer((s) => s.resetSettings);
	const volume = usePlayer((s) => s.volume);
	const setVolume = usePlayer((s) => s.setVolume);
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(BottomSheet, {
		open: sheet === "settings",
		onOpenChange: (o) => setSheet(o ? "settings" : "none"),
		eyebrow: "Scene",
		title: "Visualizer settings",
		height: "mid",
		children: [
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
				className: "divide-y divide-white/8",
				children: [
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "py-3",
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "mb-2 text-sm font-medium text-fg",
							children: "Output level"
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Slider, {
							min: 0,
							max: 1,
							step: .01,
							value: [volume],
							onValueChange: ([v]) => setVolume(v ?? 0),
							"aria-label": "Volume"
						})]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "py-3",
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "mb-2 text-sm font-medium text-fg",
							children: "Color intensity"
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Slider, {
							min: .2,
							max: 1,
							step: .01,
							value: [settings.colorIntensity],
							onValueChange: ([v]) => setSetting("colorIntensity", v ?? .8),
							"aria-label": "Color intensity"
						})]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "py-3",
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "mb-2 text-sm font-medium text-fg",
							children: "Motion intensity"
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Slider, {
							min: 0,
							max: 1,
							step: .01,
							value: [settings.motionIntensity],
							onValueChange: ([v]) => setSetting("motionIntensity", v ?? .8),
							"aria-label": "Motion intensity"
						})]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Row, {
						label: "Particle field",
						hint: "Soft reactive dust around the orb",
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch, {
							checked: settings.particles,
							onCheckedChange: (v) => setSetting("particles", v),
							"aria-label": "Particle effects"
						})
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Row, {
						label: "Spectrum rings",
						hint: "Frequency bars around the core",
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch, {
							checked: settings.spectrum,
							onCheckedChange: (v) => setSetting("spectrum", v),
							"aria-label": "Spectrum bars"
						})
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Row, {
						label: "Beat pulse",
						hint: "Kick-locked orb scale",
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch, {
							checked: settings.beatPulse,
							onCheckedChange: (v) => setSetting("beatPulse", v),
							"aria-label": "Beat pulse"
						})
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Row, {
						label: "Reduced motion",
						hint: "Calmer scene, less rotation",
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch, {
							checked: settings.reducedMotion,
							onCheckedChange: (v) => setSetting("reducedMotion", v),
							"aria-label": "Reduced motion"
						})
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Row, {
						label: "Readable type",
						hint: "Larger, more spaced labels",
						children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Switch, {
							checked: settings.readableType,
							onCheckedChange: (v) => setSetting("readableType", v),
							"aria-label": "Readable type"
						})
					})
				]
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)(Button, {
				type: "button",
				variant: "outline",
				className: "mt-5 w-full",
				onClick: resetSettings,
				children: "Reset to demo profile"
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
				className: "mt-6 rounded-2xl bg-white/4 p-4 shadow-border",
				children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
					className: "mb-2 text-xs uppercase tracking-[0.16em] text-muted",
					children: "Account"
				}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)(AccountSlot, {})]
			})
		]
	});
}
var ICONS = {
	STRUCTURAL: AudioLines,
	TONAL: Music2,
	TIMBRAL: Waves,
	LINGUISTIC: MessageCircle,
	AFFECTIVE: Heart,
	CONTEXTUAL: MapPin,
	PHOTOMETRIC: Eye,
	KINETIC: Activity,
	GENEALOGICAL: GitBranch
};
function VaspSheet() {
	const sheet = usePlayer((s) => s.sheet);
	const setSheet = usePlayer((s) => s.setSheet);
	const activePillar = usePlayer((s) => s.activePillar);
	const setPillar = usePlayer((s) => s.setPillar);
	const track = usePlayer((s) => s.current());
	const meta = PILLAR_META[activePillar];
	const fields = flattenPillar(track.vasp, activePillar);
	const Icon = ICONS[activePillar];
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)(BottomSheet, {
		open: sheet === "vasp",
		onOpenChange: (o) => setSheet(o ? "vasp" : "none"),
		eyebrow: `VASP ${track.vasp.VAP_VERSION}`,
		title: "Nine-pillar profile",
		children: [
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
				className: "mb-4 text-pretty text-sm leading-relaxed text-muted",
				children: "Creative metadata for how this audio is structured, feels, appears, and moves. Visual mappings only — not medical, psychological, biometric, or scientific diagnoses."
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
				className: "mb-5 grid grid-cols-3 gap-2",
				children: PILLAR_KEYS.map((key) => {
					const m = PILLAR_META[key];
					const KIcon = ICONS[key];
					const on = key === activePillar;
					return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("button", {
						type: "button",
						onClick: () => setPillar(key),
						className: cn("flex min-h-16 flex-col items-start gap-1 rounded-2xl p-2.5 text-left shadow-border transition-colors", on ? "bg-white/10 text-fg" : "bg-white/3 text-muted hover:bg-white/6"),
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)(KIcon, { className: cn("size-4", on ? "text-accent" : "text-muted") }), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
							className: "text-[11px] font-medium leading-tight text-fg",
							children: m.label
						})]
					}, key);
				})
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("section", {
				className: "rounded-3xl bg-white/4 p-4 shadow-border",
				children: [
					/* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
						className: "mb-3 flex items-center gap-3",
						children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
							className: "grid size-11 place-items-center rounded-2xl bg-accent/15 text-accent",
							children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(Icon, { className: "size-5" })
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", { children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("h3", {
							className: "font-display text-lg font-semibold text-fg",
							children: meta.label
						}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "text-xs uppercase tracking-[0.16em] text-muted",
							children: meta.archetype
						})] })]
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
						className: "mb-4 text-pretty text-sm leading-relaxed text-muted",
						children: meta.purpose
					}),
					/* @__PURE__ */ (0, import_jsx_runtime.jsx)("dl", {
						className: "flex flex-col gap-3",
						children: fields.map((field) => /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
							className: "flex items-start justify-between gap-4",
							children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("dt", {
								className: "text-xs uppercase tracking-[0.12em] text-muted",
								children: field.label
							}), /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("dd", {
								className: "max-w-[60%] text-right text-sm font-medium text-fg",
								children: [field.value, field.status !== "known" ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("span", {
									className: "ml-2 text-[10px] uppercase tracking-wider text-muted",
									children: field.status
								}) : null]
							})]
						}, field.label))
					})
				]
			})
		]
	});
}
var VisualizerScene = class {
	particles = [];
	rain = [];
	rot = 0;
	aurora = 0;
	width = 0;
	height = 0;
	resize(w, h) {
		this.width = w;
		this.height = h;
	}
	tick(dt, sample, map, settings) {
		const w = this.width;
		const h = this.height;
		if (!w || !h) return;
		const motion = settings.reducedMotion ? .15 : settings.motionIntensity;
		this.rot += dt * (.18 + map.bpm / 220) * motion * (.6 + sample.energy);
		this.aurora += dt * .12 * motion;
		const target = settings.particles && !settings.reducedMotion ? Math.round(70 * map.particleDensity * (.55 + settings.colorIntensity)) : 0;
		while (this.particles.length < target) this.particles.push(this.spawnParticle(w, h, map));
		if (this.particles.length > target) this.particles.length = target;
		for (const p of this.particles) {
			p.x += p.vx * (1 + sample.bass * 1.4) * motion * 60 * dt;
			p.y += p.vy * (1 + sample.energy) * motion * 60 * dt;
			p.life -= dt * .12;
			p.a = Math.max(0, p.life);
			if (p.x < -20 || p.x > w + 20 || p.y < -20 || p.y > h + 20 || p.life <= 0) Object.assign(p, this.spawnParticle(w, h, map));
		}
		const rainTarget = map.rain && !settings.reducedMotion ? Math.round(42 * motion) : 0;
		while (this.rain.length < rainTarget) this.rain.push({
			x: Math.random() * w,
			y: Math.random() * h,
			vy: 280 + Math.random() * 340,
			len: 8 + Math.random() * 18,
			a: .08 + Math.random() * .12
		});
		if (this.rain.length > rainTarget) this.rain.length = rainTarget;
		for (const d of this.rain) {
			d.y += d.vy * dt * (.7 + motion);
			d.x += dt * 40 * motion;
			if (d.y > h + 20) {
				d.y = -20;
				d.x = Math.random() * w;
			}
		}
	}
	spawnParticle(w, h, map) {
		const cx = w / 2;
		const cy = h * .42;
		const ang = Math.random() * Math.PI * 2;
		const dist = 30 + Math.random() * Math.min(w, h) * .42;
		return {
			x: cx + Math.cos(ang) * dist,
			y: cy + Math.sin(ang) * dist * .72,
			vx: Math.cos(ang) * (.15 + Math.random() * .55) * (.4 + map.movementEnergy),
			vy: Math.sin(ang) * (.1 + Math.random() * .4) - .12,
			r: .6 + Math.random() * 2.2,
			a: .2 + Math.random() * .55,
			life: .6 + Math.random() * 1.4
		};
	}
};
function drawFrame(ctx, w, h, sample, map, settings, scene) {
	const primary = hexToRgb(map.primary);
	const secondary = hexToRgb(map.secondary);
	const intensity = settings.colorIntensity;
	const beat = settings.beatPulse ? sample.beatPulse : 0;
	const motion = settings.reducedMotion ? .2 : settings.motionIntensity;
	const fade = settings.reducedMotion ? 1 : Math.min(.42, map.fade + .12);
	const brightness = map.brightness * intensity;
	ctx.globalCompositeOperation = "source-over";
	ctx.fillStyle = `rgba(7, 6, 12, ${.55 + fade * .4})`;
	ctx.fillRect(0, 0, w, h);
	const cx = w / 2;
	const cy = h * .4;
	const warm = map.temperature === "warm" ? 28 : map.temperature === "neutral" ? 10 : 0;
	const pr = Math.min(primary.r + warm * map.accentWarmth * 40, 255);
	const pg = primary.g;
	const pb = primary.b;
	const g0 = ctx.createRadialGradient(cx, cy, 8, cx, cy, Math.max(w, h) * .72);
	g0.addColorStop(0, `rgba(${pr},${pg},${pb},${.22 * brightness})`);
	g0.addColorStop(.38, `rgba(${secondary.r},${secondary.g},${secondary.b},${.1 * brightness})`);
	g0.addColorStop(1, "rgba(7,6,12,0)");
	ctx.fillStyle = g0;
	ctx.fillRect(0, 0, w, h);
	const ax = cx + Math.cos(scene.aurora) * w * .18;
	const ay = cy + Math.sin(scene.aurora * .7) * h * .08;
	const g1 = ctx.createRadialGradient(ax, ay, 20, ax, ay, w * .45);
	g1.addColorStop(0, `rgba(${secondary.r},${secondary.g},${secondary.b},${.16 * brightness * (.5 + sample.energy)})`);
	g1.addColorStop(1, "rgba(0,0,0,0)");
	ctx.fillStyle = g1;
	ctx.fillRect(0, 0, w, h);
	const bx = cx + Math.cos(scene.aurora + 2.1) * w * .22;
	const by = cy + Math.sin(scene.aurora * .9 + 1.2) * h * .1;
	const g2 = ctx.createRadialGradient(bx, by, 10, bx, by, w * .38);
	g2.addColorStop(0, `rgba(${pr},${pg},${Math.min(255, pb + 40)},${.12 * brightness})`);
	g2.addColorStop(1, "rgba(0,0,0,0)");
	ctx.fillStyle = g2;
	ctx.fillRect(0, 0, w, h);
	if (scene.rain.length) {
		ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},0.18)`;
		ctx.lineWidth = 1;
		ctx.beginPath();
		for (const d of scene.rain) {
			ctx.moveTo(d.x, d.y);
			ctx.lineTo(d.x + 3, d.y + d.len);
		}
		ctx.stroke();
	}
	if (settings.particles) for (const p of scene.particles) {
		ctx.beginPath();
		ctx.fillStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${p.a * .7 * intensity})`;
		ctx.arc(p.x, p.y, p.r * (1 + sample.treble * .8), 0, Math.PI * 2);
		ctx.fill();
	}
	const orbR = Math.min(w, h) * (.16 + sample.bass * .04 + beat * .035 * map.impactScale);
	drawMandala(ctx, cx, cy, orbR, scene.rot, map, sample, primary, secondary, intensity, motion);
	drawOrb(ctx, cx, cy, orbR, sample, primary, secondary, brightness, beat, map);
	if (settings.spectrum) drawSpectrum(ctx, cx, cy, orbR, sample, secondary, primary, intensity, map, scene.rot);
	drawWaveform(ctx, w, h, sample, secondary, intensity, beat);
	drawHorizon(ctx, w, h, primary, secondary, sample, brightness);
	if (map.grain > .05 && !settings.reducedMotion) {
		ctx.save();
		ctx.globalAlpha = map.grain * .18 * intensity;
		const step = 7;
		for (let y = 0; y < h; y += step) for (let x = 0; x < w; x += step) if ((x * 13 + y * 7 + Math.floor(sample.timeSec * 40) & 7) === 0) {
			ctx.fillStyle = "#ffffff";
			ctx.fillRect(x, y, 1, 1);
		}
		ctx.restore();
	}
}
function drawOrb(ctx, cx, cy, r, sample, primary, secondary, brightness, beat, map) {
	ctx.save();
	ctx.shadowColor = `rgba(${primary.r},${primary.g},${primary.b},${.65 * brightness})`;
	ctx.shadowBlur = 36 + beat * 40 * map.impactScale;
	const glow = ctx.createRadialGradient(cx, cy, r * .1, cx, cy, r * 1.65);
	glow.addColorStop(0, `rgba(255,255,255,${.55 * brightness})`);
	glow.addColorStop(.18, `rgba(${secondary.r},${secondary.g},${secondary.b},${.55 * brightness})`);
	glow.addColorStop(.55, `rgba(${primary.r},${primary.g},${primary.b},${.55 * brightness})`);
	glow.addColorStop(1, "rgba(7,6,12,0)");
	ctx.fillStyle = glow;
	ctx.beginPath();
	ctx.arc(cx, cy, r * 1.55, 0, Math.PI * 2);
	ctx.fill();
	const core = ctx.createRadialGradient(cx - r * .22, cy - r * .28, r * .05, cx, cy, r);
	core.addColorStop(0, `rgba(255,255,255,${.8 * brightness})`);
	core.addColorStop(.35, `rgba(${secondary.r},${secondary.g},${secondary.b},0.85)`);
	core.addColorStop(1, `rgba(${primary.r},${primary.g},${primary.b},0.95)`);
	ctx.fillStyle = core;
	ctx.shadowBlur = 18;
	ctx.beginPath();
	ctx.arc(cx, cy, r * (.72 + sample.energy * .08), 0, Math.PI * 2);
	ctx.fill();
	ctx.restore();
}
function drawMandala(ctx, cx, cy, r, rot, map, sample, primary, secondary, intensity, motion) {
	const sides = Math.max(3, map.geometrySides);
	ctx.save();
	ctx.translate(cx, cy);
	for (let ring = 0; ring < 3; ring++) {
		const rr = r * (1.15 + ring * .28 + sample.mid * .08);
		ctx.rotate((ring % 2 === 0 ? rot : -rot * .7) * motion);
		ctx.strokeStyle = ring === 1 ? `rgba(${secondary.r},${secondary.g},${secondary.b},${.28 * intensity})` : `rgba(${primary.r},${primary.g},${primary.b},${.22 * intensity})`;
		ctx.lineWidth = ring === 0 ? 1.6 : 1;
		ctx.beginPath();
		for (let i = 0; i <= sides * 2; i++) {
			const a = i / (sides * 2) * Math.PI * 2 - Math.PI / 2;
			const rad = i % 2 === 0 ? rr : rr * (.72 + map.harmonicShift);
			const x = Math.cos(a) * rad;
			const y = Math.sin(a) * rad;
			if (i === 0) ctx.moveTo(x, y);
			else ctx.lineTo(x, y);
		}
		ctx.closePath();
		ctx.stroke();
	}
	ctx.restore();
	ctx.save();
	ctx.translate(cx, cy);
	ctx.rotate(-rot * .35 * motion);
	ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${.35 * intensity})`;
	ctx.lineWidth = 1.2;
	ctx.setLineDash([4, 8]);
	ctx.beginPath();
	ctx.arc(0, 0, r * 1.95, 0, Math.PI * 2);
	ctx.stroke();
	ctx.setLineDash([]);
	ctx.restore();
}
function drawSpectrum(ctx, cx, cy, orbR, sample, secondary, primary, intensity, map, rot) {
	const bars = 64;
	const inner = orbR * 2.15;
	ctx.save();
	ctx.translate(cx, cy);
	ctx.rotate(rot * .08);
	for (let i = 0; i < bars; i++) {
		const bin = Math.floor(i / bars * (sample.freq.length * .55));
		const mag = (sample.freq[bin] ?? 0) / 255;
		const a = i / bars * Math.PI * 2 - Math.PI / 2;
		const len = mag * Math.min(88, orbR * 1.15) * map.edgeSharpness * intensity;
		const x0 = Math.cos(a) * inner;
		const y0 = Math.sin(a) * inner;
		const x1 = Math.cos(a) * (inner + len);
		const y1 = Math.sin(a) * (inner + len);
		const mix = i % 2 === 0 ? secondary : primary;
		ctx.strokeStyle = `rgba(${mix.r},${mix.g},${mix.b},${.28 + mag * .55})`;
		ctx.lineWidth = 2.4;
		ctx.lineCap = "round";
		ctx.beginPath();
		ctx.moveTo(x0, y0);
		ctx.lineTo(x1, y1);
		ctx.stroke();
	}
	ctx.restore();
}
function drawWaveform(ctx, w, h, sample, secondary, intensity, beat) {
	const y = h * .78;
	const amp = h * .055 * (1 + beat * .4);
	ctx.beginPath();
	const n = sample.time.length;
	for (let i = 0; i < n; i++) {
		const x = i / (n - 1) * w;
		const yy = y + (sample.time[i] - 128) / 128 * amp;
		if (i === 0) ctx.moveTo(x, yy);
		else ctx.lineTo(x, yy);
	}
	ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${.55 * intensity})`;
	ctx.lineWidth = 1.8;
	ctx.shadowColor = `rgba(${secondary.r},${secondary.g},${secondary.b},0.45)`;
	ctx.shadowBlur = 8;
	ctx.stroke();
	ctx.shadowBlur = 0;
	ctx.globalAlpha = .12 * intensity;
	ctx.lineTo(w, h);
	ctx.lineTo(0, h);
	ctx.closePath();
	ctx.fillStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},0.4)`;
	ctx.fill();
	ctx.globalAlpha = 1;
}
function drawHorizon(ctx, w, h, primary, secondary, sample, brightness) {
	const g = ctx.createLinearGradient(0, h * .62, 0, h);
	g.addColorStop(0, "rgba(7,6,12,0)");
	g.addColorStop(.45, `rgba(${primary.r},${primary.g},${primary.b},${.08 * brightness})`);
	g.addColorStop(1, `rgba(${secondary.r},${secondary.g},${secondary.b},${.1 + sample.bass * .08})`);
	ctx.fillStyle = g;
	ctx.fillRect(0, h * .62, w, h * .38);
}
function VisualizerCanvas() {
	const canvasRef = (0, import_react.useRef)(null);
	const mapping = usePlayer((s) => s.mapping);
	const settings = usePlayer((s) => s.settings);
	const mapRef = (0, import_react.useRef)(mapping);
	const setRef = (0, import_react.useRef)(settings);
	mapRef.current = mapping;
	setRef.current = settings;
	(0, import_react.useEffect)(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		const ctx = canvas.getContext("2d", { alpha: false });
		if (!ctx) return;
		const scene = new VisualizerScene();
		let raf = 0;
		let last = performance.now();
		let running = true;
		const resize = () => {
			const parent = canvas.parentElement ?? canvas;
			const dpr = Math.min(2, window.devicePixelRatio || 1);
			const w = parent.clientWidth;
			const h = parent.clientHeight;
			canvas.width = Math.max(1, Math.floor(w * dpr));
			canvas.height = Math.max(1, Math.floor(h * dpr));
			canvas.style.width = `${w}px`;
			canvas.style.height = `${h}px`;
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
			scene.resize(w, h);
		};
		resize();
		const ro = new ResizeObserver(resize);
		ro.observe(canvas.parentElement ?? canvas);
		const loop = (now) => {
			if (!running) return;
			const dt = Math.min(.05, (now - last) / 1e3);
			last = now;
			const sample = engine.sample();
			const map = mapRef.current;
			const set = setRef.current;
			scene.tick(dt, sample, map, set);
			drawFrame(ctx, scene.width, scene.height, sample, map, set, scene);
			raf = requestAnimationFrame(loop);
		};
		raf = requestAnimationFrame(loop);
		const onVis = () => {
			if (document.hidden) {
				running = false;
				cancelAnimationFrame(raf);
			} else {
				running = true;
				last = performance.now();
				raf = requestAnimationFrame(loop);
			}
		};
		document.addEventListener("visibilitychange", onVis);
		return () => {
			running = false;
			cancelAnimationFrame(raf);
			ro.disconnect();
			document.removeEventListener("visibilitychange", onVis);
		};
	}, []);
	return /* @__PURE__ */ (0, import_jsx_runtime.jsx)("canvas", {
		ref: canvasRef,
		className: "absolute inset-0 h-full w-full",
		"aria-hidden": true
	});
}
function PlayerApp() {
	const hydrate = usePlayer((s) => s.hydrate);
	const importFiles = usePlayer((s) => s.importFiles);
	const setDropActive = usePlayer((s) => s.setDropActive);
	const dropActive = usePlayer((s) => s.dropActive);
	const notice = usePlayer((s) => s.notice);
	const setNotice = usePlayer((s) => s.setNotice);
	const readable = usePlayer((s) => s.settings.readableType);
	const togglePlay = usePlayer((s) => s.togglePlay);
	(0, import_react.useEffect)(() => {
		hydrate();
	}, [hydrate]);
	(0, import_react.useEffect)(() => {
		const unlock = () => engine.unlock();
		window.addEventListener("pointerdown", unlock);
		return () => window.removeEventListener("pointerdown", unlock);
	}, []);
	(0, import_react.useEffect)(() => {
		if (!notice) return;
		const t = window.setTimeout(() => setNotice(null), 4200);
		return () => window.clearTimeout(t);
	}, [notice, setNotice]);
	(0, import_react.useEffect)(() => {
		const onKey = (e) => {
			const tag = e.target?.tagName;
			if (tag === "INPUT" || tag === "TEXTAREA") return;
			if (e.code === "Space") {
				e.preventDefault();
				togglePlay();
			}
		};
		window.addEventListener("keydown", onKey);
		return () => window.removeEventListener("keydown", onKey);
	}, [togglePlay]);
	return /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
		className: cn("relative h-dvh w-full overflow-hidden bg-bg text-fg", readable && "readable-type"),
		onDragOver: (e) => {
			e.preventDefault();
			setDropActive(true);
		},
		onDragLeave: () => setDropActive(false),
		onDrop: (e) => {
			e.preventDefault();
			setDropActive(false);
			if (e.dataTransfer.files.length) importFiles(e.dataTransfer.files);
		},
		children: [
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)(VisualizerCanvas, {}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("header", {
				className: "pointer-events-none absolute inset-x-0 top-0 z-10 flex items-start justify-between px-4 pt-[max(0.9rem,env(safe-area-inset-top))]",
				children: /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
					className: "pointer-events-auto",
					children: [
						/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "font-display text-[11px] font-semibold uppercase tracking-[0.28em] text-accent",
							children: "Aurphyx"
						}),
						/* @__PURE__ */ (0, import_jsx_runtime.jsx)("h1", {
							className: "font-display text-xl font-semibold tracking-tight text-fg",
							children: "Vibe Audio Player"
						}),
						/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
							className: "text-[11px] uppercase tracking-[0.16em] text-muted",
							children: "VASP 3.69"
						})
					]
				})
			}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
				className: "pointer-events-none absolute inset-x-0 bottom-0 z-10 px-3 pb-[max(3.75rem,calc(env(safe-area-inset-bottom)+2.25rem))]",
				children: /* @__PURE__ */ (0, import_jsx_runtime.jsx)(NowPlayingCard, {})
			}),
			dropActive ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
				className: "absolute inset-0 z-30 grid place-items-center bg-bg/70 backdrop-blur-sm",
				children: /* @__PURE__ */ (0, import_jsx_runtime.jsxs)("div", {
					className: "rounded-3xl bg-surface px-8 py-6 text-center shadow-sheet",
					children: [/* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
						className: "font-display text-lg font-semibold",
						children: "Drop audio to add"
					}), /* @__PURE__ */ (0, import_jsx_runtime.jsx)("p", {
						className: "mt-1 text-sm text-muted",
						children: "MP3, WAV, FLAC, OGG, M4A"
					})]
				})
			}) : null,
			notice ? /* @__PURE__ */ (0, import_jsx_runtime.jsx)("div", {
				className: "absolute inset-x-0 top-20 z-20 mx-auto w-[min(92%,24rem)] rounded-2xl bg-surface px-4 py-3 text-center text-sm text-fg shadow-sheet",
				children: notice
			}) : null,
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)(LibrarySheet, {}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)(VaspSheet, {}),
			/* @__PURE__ */ (0, import_jsx_runtime.jsx)(SettingsSheet, {})
		]
	});
}
function Home() {
	return /* @__PURE__ */ (0, import_jsx_runtime.jsx)(PlayerApp, {});
}
//#endregion
export { Home as component };
